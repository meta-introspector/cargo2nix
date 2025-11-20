use anyhow::{Result, Context};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use toml_edit::{DocumentMut, Table, Item, value};
use cargo_metadata::MetadataCommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    project_root: PathBuf,

    /// Path to the submodules directory.
    #[arg(long, default_value = "submodules")]
    submodules_dir: PathBuf,

    /// Path to the root Cargo.toml file.
    #[arg(long, default_value = "Cargo.toml")]
    root_cargo_toml: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let project_root = args.project_root.canonicalize().context("Failed to canonicalize project_root")?;
    let submodules_dir = project_root.join(&args.submodules_dir);
    let root_cargo_toml_path = project_root.join(&args.root_cargo_toml);

    println!("Scanning submodules in: {}", submodules_dir.display());

    let mut workspace_dependencies: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        let submodule_root = cargo_toml_path.parent().unwrap();

        // Try to get cargo metadata for the submodule
        let metadata_result = MetadataCommand::new()
            .manifest_path(cargo_toml_path)
            .no_deps()
            .exec();

        match metadata_result {
            Ok(metadata) => {
                // Check if this submodule is a workspace itself
                let workspace_root = metadata.workspace_root.to_string();
                if Path::new(&workspace_root) == submodule_root {
                    // This is a submodule that is also a workspace
                    println!("Found submodule workspace: {}", submodule_root.display());
                    for member_id in &metadata.workspace_members {
                        if let Some(member_package) = metadata.packages.iter().find(|p| &p.id == member_id) {
                            let member_manifest_path = PathBuf::from(&member_package.manifest_path);
                            let member_crate_root = member_manifest_path.parent().unwrap();
                            let relative_path = pathdiff::diff_paths(member_crate_root, &project_root)
                                .context(format!("Failed to get relative path for member crate {}", member_package.name))?;
                            workspace_dependencies.insert(
                                member_package.name.to_string(),
                                format!("{{ path = \"{}\" }}", relative_path.display()),
                            );
                        }
                    }
                } else {
                    // It's a regular package within a submodule
                    if let Some(package) = metadata.packages.get(0) {
                        let relative_path = pathdiff::diff_paths(submodule_root, &project_root)
                            .context(format!("Failed to get relative path for package {}", package.name))?;
                        workspace_dependencies.insert(
                            package.name.to_string(),
                            format!("{{ path = \"{}\" }}", relative_path.display()),
                        );
                    }
                }
            },
            Err(e) => {
                eprintln!("Warning: Could not get cargo metadata for {}: {}", cargo_toml_path.display(), e);
                // Fallback: try to parse as a single package if metadata fails
                let content = fs::read_to_string(cargo_toml_path)
                    .with_context(|| format!("Failed to read Cargo.toml at {}", cargo_toml_path.display()))?;
                let doc = content.parse::<DocumentMut>()
                    .with_context(|| format!("Failed to parse Cargo.toml at {}", cargo_toml_path.display()))?;
                
                if let Some(package_name) = doc.get("package").and_then(|item| item.as_table())
                                                .and_then(|table| table.get("name")).and_then(|item| item.as_str()) {
                    let relative_path = pathdiff::diff_paths(submodule_root, &project_root)
                        .context(format!("Failed to get relative path for package {}", package_name))?;
                    workspace_dependencies.insert(
                        package_name.to_string(),
                        format!("{{ path = \"{}\" }}", relative_path.display()),
                    );
                }
            }
        }
    }

    // Read the root Cargo.toml
    let root_cargo_toml_content = fs::read_to_string(&root_cargo_toml_path)
        .with_context(|| format!("Failed to read root Cargo.toml at {}", root_cargo_toml_path.display()))?;
    let mut root_doc = root_cargo_toml_content.parse::<DocumentMut>()
        .context("Failed to parse root Cargo.toml")?;

    // Create a new [workspace.dependencies] table
    let mut new_workspace_deps_table = Table::new();
    new_workspace_deps_table.set_dotted(false);

    // Sort dependencies by name
    let mut sorted_deps: Vec<_> = workspace_dependencies.into_iter().collect();
    sorted_deps.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, definition) in sorted_deps {
        new_workspace_deps_table.insert(&name, value(definition));
    }

    // Replace the [workspace.dependencies] section
    root_doc["workspace"]["dependencies"] = Item::Table(new_workspace_deps_table);

    // Write the updated Cargo.toml back
    fs::write(&root_cargo_toml_path, root_doc.to_string().as_bytes())
        .with_context(|| format!("Failed to write to root Cargo.toml at {}", root_cargo_toml_path.display()))?;

    println!("Successfully updated [workspace.dependencies] in {}", root_cargo_toml_path.display());

    Ok(())
}