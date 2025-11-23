use anyhow::Result; // Added anyhow imports
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use toml_edit::{self}; // Added toml_edit::
use walkdir::WalkDir; // Added WalkDir

use git_wrapper_lib::git_traits::Execv; // Import Execv trait
use std::ffi::OsStr;
use std::sync::Arc;

#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter};

pub trait WorkspaceGenerator {
    fn generate_workspace_dependencies(
        &self,
        root_dir: &Path,
        dry_run: bool,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DefaultWorkspaceGenerator;

impl WorkspaceGenerator for DefaultWorkspaceGenerator {
    fn generate_workspace_dependencies(
        &self,
        root_dir: &Path,
        dry_run: bool,
        executor: Arc<dyn Execv + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating comprehensive [workspace.dependencies] section...");

        let submodules_dir = root_dir.join("submodules");
        let generated_deps_path = root_dir.join("generated_workspace_deps.toml");

        // 1. Run `cargo metadata`
        let output = executor
            .execv(
                OsStr::new("cargo"),
                &[
                    OsStr::new("metadata"),
                    OsStr::new("--format-version"),
                    OsStr::new("1"),
                ],
                Some(root_dir),
            )
            .map_err(|e| format!("Failed to execute cargo metadata: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "cargo metadata failed:\nStdout: {}\nStderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        #[cfg(feature = "tool_traits_lib_enabled")]
        let serde_adapter = CurrentSerdeAdapter;
        #[cfg(feature = "tool_traits_lib_enabled")]
        let metadata: serde_json::Value = serde_adapter
            .from_str(String::from_utf8_lossy(&output.stdout).as_ref())
            .map_err(|e| format!("Failed to parse cargo metadata output: {}", e))?;
        #[cfg(not(feature = "tool_traits_lib_enabled"))]
        let metadata: serde_json::Value = serde_json::Value::Null; // Dummy value

        let mut all_dependencies: HashMap<String, String> = HashMap::new(); // name -> version

        // Iterate over all packages in the workspace
        if let Some(packages) = metadata["packages"].as_array() {
            for pkg in packages {
                if let Some(name) = pkg["name"].as_str() {
                    if let Some(version) = pkg["version"].as_str() {
                        // Only add if not already present or if new version is higher
                        let current_version = all_dependencies.get::<str>(name);
                        if current_version.is_none()
                            || (current_version.is_some()
                                && version > current_version.unwrap().as_str())
                        {
                            all_dependencies.insert(name.to_string(), version.to_string());
                        }
                    }
                }
            }
        }

        // Get a list of submodule names
        let mut submodule_names: Vec<String> = fs::read_dir(&submodules_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.file_type().ok()?.is_dir() {
                    entry.file_name().into_string().ok()
                } else {
                    None
                }
            })
            .collect();
        submodule_names.sort();

        let mut doc = toml_edit::DocumentMut::new();
        let mut workspace_deps_table = toml_edit::Table::new();

        for (dep_name, dep_version) in all_dependencies.iter() {
            let mut is_submodule = false;
            let mut submodule_path = PathBuf::new();

            // Check for direct submodule match
            if submodule_names.contains(dep_name) {
                is_submodule = true;
                submodule_path = submodules_dir.join(dep_name);
            } else {
                // Check for nested submodules (e.g., time-rs/time)
                for sm_name in &submodule_names {
                    let potential_path = submodules_dir.join(sm_name).join(dep_name);
                    if potential_path.exists() && potential_path.is_dir() {
                        is_submodule = true;
                        submodule_path = potential_path;
                        break;
                    }
                }
            }

            let mut dep_table = toml_edit::Table::new();
            if is_submodule {
                dep_table.insert(
                    "path",
                    toml_edit::value(format!(
                        "./{}",
                        submodule_path
                            .strip_prefix(root_dir)
                            .unwrap()
                            .to_string_lossy()
                    )),
                );
            } else {
                dep_table.insert("version", toml_edit::value(dep_version.clone()));
            }
            workspace_deps_table.insert(dep_name, toml_edit::Item::Table(dep_table));
        }

        doc.insert("workspace", toml_edit::Item::Table(toml_edit::Table::new()));
        doc["workspace"]
            .as_table_mut()
            .unwrap()
            .insert("dependencies", toml_edit::Item::Table(workspace_deps_table));

        if dry_run {
            println!("--- DRY RUN: Generated [workspace.dependencies] content ---");
            println!("{}", doc.to_string());
            println!("---------------------------------------------");
        } else {
            fs::write(&generated_deps_path, doc.to_string())?;
            println!(
                "Successfully generated [workspace.dependencies] to {:?}",
                generated_deps_path
            );
        }

        Ok(())
    }
}

pub fn add_workspace_submodules(
    root_dir: &Path,
    dry_run: bool,
    executor: Arc<dyn Execv + Send + Sync>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Adding all submodules as path dependencies to [workspace.dependencies]...");

    let cargo_toml_path = root_dir.join("Cargo.toml");
    let submodules_dir = root_dir.join("submodules");

    let mut doc = fs::read_to_string(&cargo_toml_path)?.parse::<toml_edit::DocumentMut>()?;

    let workspace_deps = doc
        .get_mut("workspace")
        .and_then(|item| item.as_table_mut())
        .and_then(|table| table.get_mut("dependencies"))
        .and_then(|item| item.as_table_mut())
        .ok_or("Could not find [workspace.dependencies] in Cargo.toml")?;

    let mut submodule_names: Vec<String> = fs::read_dir(&submodules_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    submodule_names.sort();

    for submodule in submodule_names {
        let dep_path = format!("./submodules/{}", submodule);
        let item = toml_edit::Item::Table(toml_edit::Table::new());
        let mut dep_table = item.into_table().unwrap();
        dep_table.insert("path", toml_edit::value(dep_path));

        match submodule.as_str() {
            "serde" => {
                workspace_deps.insert("serde", toml_edit::Item::Table(dep_table.clone()));
                workspace_deps.insert(
                    "serde_derive",
                    toml_edit::value(format!("{{ path = \"./submodules/serde/serde_derive\" }}")),
                );
                workspace_deps.insert(
                    "serde_core",
                    toml_edit::value(format!("{{ path = \"./submodules/serde/serde_core\" }}")),
                );
            }
            "time-rs" => {
                workspace_deps.insert("time", toml_edit::Item::Table(dep_table.clone()));
                workspace_deps.insert(
                    "time-core",
                    toml_edit::value(format!("{{ path = \"./submodules/time-rs/time-core\" }}")),
                );
                workspace_deps.insert(
                    "time-macros",
                    toml_edit::value(format!("{{ path = \"./submodules/time-rs/time-macros\" }}")),
                );
            }
            "rand" => {
                workspace_deps.insert("rand", toml_edit::Item::Table(dep_table.clone()));
                workspace_deps.insert(
                    "rand08",
                    toml_edit::value(format!("{{ path = \"./submodules/rand\" }}")),
                );
                workspace_deps.insert(
                    "rand09",
                    toml_edit::value(format!("{{ path = \"./submodules/rand\" }}")),
                );
            }
            _ => {
                workspace_deps.insert(&submodule, toml_edit::Item::Table(dep_table));
            }
        }
    }

    if dry_run {
        println!("--- DRY RUN: Generated Cargo.toml content ---");
        println!("{}", doc.to_string());
        println!("---------------------------------------------");
    } else {
        fs::write(&cargo_toml_path, doc.to_string())?;
        println!("Successfully updated Cargo.toml with submodule workspace dependencies.");
    }

    Ok(())
}

pub fn comment_submodule_workspaces(
    root_dir: &Path,
    dry_run: bool,
    executor: Arc<dyn Execv + Send + Sync>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Commenting out [workspace] sections in submodule Cargo.toml files...");

    let submodules_dir = root_dir.join("submodules");

    for entry in walkdir::WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        println!("Processing Cargo.toml: {:?}", cargo_toml_path);
        println!("  Dry run: {}", dry_run);

        let mut doc = fs::read_to_string(&cargo_toml_path)?.parse::<toml_edit::DocumentMut>()?;

        if let Some(workspace_item) = doc.get_mut("workspace") {
            if dry_run {
                println!(
                    "--- DRY RUN: Would remove [workspace] section in {:?} ---",
                    cargo_toml_path
                );
            } else {
                doc.remove("workspace");
                println!("  Removed [workspace] section from {:?}", cargo_toml_path);
            }
        } else {
            println!("  No [workspace] section found in {:?}", cargo_toml_path);
        }

        if dry_run {
            println!(
                "--- DRY RUN: Generated Cargo.toml content for {:?} ---",
                cargo_toml_path
            );
            println!("{}", doc.to_string());
            println!("---------------------------------------------");
        } else {
            fs::write(&cargo_toml_path, doc.to_string())?;
            println!("Successfully updated Cargo.toml: {:?}", cargo_toml_path);
        }
    }

    println!("Finished commenting out [workspace] sections.");
    Ok(())
}
