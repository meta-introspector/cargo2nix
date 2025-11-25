#[cfg(feature = "anyhow_enabled")]
use anyhow::{Context, Result};
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error;
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(not(feature = "anyhow_enabled"))]
trait Context<T> {
    fn context<C>(self, _context: C) -> Result<T>
    where C: std::fmt::Display + Send + Sync + 'static;
}

#[cfg(not(feature = "anyhow_enabled"))]
impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}: {}", context, e))) as Box<dyn Error>)
    }
}

#[cfg(feature = "clap_enabled")]
use clap::Parser;
#[cfg(feature = "pathdiff_enabled")]
use pathdiff::diff_paths;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
#[cfg(feature = "toml_edit_enabled")]
use toml_edit::{value, DocumentMut, Item, Table};
#[cfg(feature = "walkdir_enabled")]
use walkdir::WalkDir;

use crate::metadata_provider::{CargoMetadataProvider, Metadata, Package, PackageId};

#[cfg(feature = "real_cargo_metadata")]
use crate::metadata_provider::RealCargoMetadataProvider;
#[cfg(not(feature = "real_cargo_metadata"))]
use crate::metadata_provider::DummyCargoMetadataProvider;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root directory of the project.
    #[arg(long, default_value = ".")]
    project_root: PathBuf,

    /// The output directory for the generated workspace.
    #[arg(long, default_value = "generated_workspaces")]
    output_dir: PathBuf,

    /// The name of the package to invert the dependency tree for. If not provided, all workspace members are considered.
    #[arg(long)]
    package_name: Option<String>,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
struct Args {
    project_root: PathBuf,
    output_dir: PathBuf,
    package_name: Option<String>,
}

#[cfg(not(feature = "clap_enabled"))]
impl Args {
    fn parse() -> Self {
        Args {
            project_root: PathBuf::from("."),
            output_dir: PathBuf::from("generated_workspaces"),
            package_name: None,
        }
    }
}


fn main() -> Result<()> {
    let args = Args::parse();

    let project_root = args
        .project_root
        .canonicalize()
        .context("Failed to canonicalize project_root")?;
    let output_dir = project_root.join(&args.output_dir);
    let submodules_dir = project_root.join("submodules");

    // 1. Get cargo metadata
    println!("Collecting cargo metadata...");
    
    #[cfg(feature = "real_cargo_metadata")]
    let metadata_provider = RealCargoMetadataProvider;
    #[cfg(not(feature = "real_cargo_metadata"))]
    let metadata_provider = DummyCargoMetadataProvider;

    let metadata = metadata_provider
        .provide_metadata(&project_root)
        .context("Failed to get cargo metadata")?;



    let workspace_members: Vec<&Package> = metadata
        .workspace_members
        .iter()
        .filter_map(|id| metadata.packages.iter().find(|pkg| &pkg.id.repr == &id.repr))
        .collect();

    let target_package_ids: Vec<PackageId> =
        if let Some(pkg_name) = &args.package_name {
            workspace_members
                .iter()
                .filter(|pkg| pkg.name.as_str() == *pkg_name)
                .map(|pkg| pkg.id.clone())
                .collect()
        } else {
            workspace_members.iter().map(|pkg| pkg.id.clone()).collect()
        };

    if target_package_ids.is_empty() {
        anyhow::bail!("No target packages found for inversion. Check package_name or ensure workspace has members.");
    }

    // 2. Build the inverse dependency graph (dependee -> [dependents])
    let mut inverse_graph: HashMap<PackageId, Vec<PackageId>> =
        HashMap::new();
    for package in &metadata.packages {
        for dep in &package.dependencies {
            // Find the actual PackageId for the dependency name
            if let Some(dep_pkg) = metadata
                .packages
                .iter()
                .find(|p| p.name.as_str() == dep.name.as_str())
            {
                inverse_graph
                    .entry(dep_pkg.id.clone())
                    .or_default()
                    .push(package.id.clone());
            }
        }
    }

    // 3. Build a forward dependency graph (package -> [direct_dependencies])
    let mut forward_graph: HashMap<PackageId, Vec<PackageId>> =
        HashMap::new();
    for package in &metadata.packages {
        for dep in &package.dependencies {
            if let Some(dep_pkg) = metadata
                .packages
                .iter()
                .find(|p| p.name.as_str() == dep.name.as_str())
            {
                forward_graph
                    .entry(package.id.clone())
                    .or_default()
                    .push(dep_pkg.id.clone());
            }
        }
    }

    // 4. Traverse the inverse graph to find all packages that depend on the target packages
    let mut inverted_dependencies: HashSet<PackageId> = HashSet::new();
    let mut queue: Vec<PackageId> = target_package_ids.clone();
    let mut visited: HashSet<PackageId> = HashSet::new();

    while let Some(current_id) = queue.pop() {
        if !visited.insert(current_id.clone()) {
            continue;
        }
        inverted_dependencies.insert(current_id.clone());

        if let Some(dependents) = inverse_graph.get(&current_id) {
            for dependent_id in dependents {
                if !visited.contains(dependent_id) {
                    queue.push(dependent_id.clone());
                }
            }
        }
    }

    // 5. Discover local submodules
    println!("Discovering local submodules...");
    let mut submodule_paths: HashMap<String, PathBuf> = HashMap::new(); // package_name -> relative_path_from_project_root

    for entry in WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        
        #[cfg(feature = "real_cargo_metadata")]
        let submodule_metadata_provider = RealCargoMetadataProvider;
        #[cfg(not(feature = "real_cargo_metadata"))]
        let submodule_metadata_provider = DummyCargoMetadataProvider;

        let submodule_package_metadata = submodule_metadata_provider
            .provide_metadata(cargo_toml_path.parent().unwrap())
            .context(format!(
                "Failed to get metadata for submodule Cargo.toml: {:?}",
                cargo_toml_path
            ))?;

        if let Some(pkg) = submodule_package_metadata.packages.first() {
            let relative_path = diff_paths(cargo_toml_path.parent().unwrap(), &project_root)
                .context(format!(
                    "Failed to get relative path for submodule: {:?}",
                    cargo_toml_path
                ))?;
            submodule_paths.insert(pkg.name.to_string(), relative_path);
        }
    }

    // Ensure the main output directory exists
    std::fs::create_dir_all(&output_dir).context("Failed to create main output directory")?;

    let mut missing_submodule_commands: HashSet<String> = HashSet::new(); // Collect missing submodule commands

    // 6. Generate a new Cargo.toml for each inverted dependency as a root
    for current_root_pkg_id in &inverted_dependencies {
        let current_root_pkg = metadata
            .packages
            .iter()
            .find(|p| p.id.repr == current_root_pkg_id.repr)
            .unwrap();
        let current_root_pkg_name = &current_root_pkg.name;

        // Create a subdirectory for this specific root workspace
        let current_workspace_output_dir = output_dir.join(current_root_pkg_name.as_str());
        std::fs::create_dir_all(&current_workspace_output_dir).context(format!(
            "Failed to create output directory for {}",
            current_root_pkg_name
        ))?;

        let new_cargo_toml_path = current_workspace_output_dir.join("Cargo.toml");
        let mut doc = DocumentMut::new();

        // Determine members for this specific workspace
        let mut members_for_this_workspace: Vec<String> = Vec::new();
        if submodule_paths.contains_key(current_root_pkg_name.as_str()) {
            members_for_this_workspace.push(current_root_pkg_name.to_string());
        }

        // Determine direct dependencies of the current root that are also local submodules
        let mut direct_submodule_deps: HashMap<String, PathBuf> = HashMap::new();
        for dep in &current_root_pkg.dependencies {
            if let Some(sub_path) = submodule_paths.get(dep.name.as_str()) {
                direct_submodule_deps.insert(dep.name.to_string(), sub_path.clone());
            }
        }

        // Add [workspace] section
        let mut workspace_table = Table::new();
        let mut members_array = toml_edit::Array::new();
        for member_name in &members_for_this_workspace {
            if let Some(sub_path) = submodule_paths.get(member_name.as_str()) {
                let relative_to_output =
                    diff_paths(project_root.join(sub_path), &current_workspace_output_dir)
                        .context(format!(
                            "Failed to get relative path for member {}",
                            member_name
                        ))?;
                members_array.push(relative_to_output.to_string_lossy().to_string());
            }
        }
        workspace_table.insert("members", value(members_array));

        // Add [workspace.dependencies] section
        let mut workspace_deps_table = Table::new();
        for (dep_name, sub_path) in &direct_submodule_deps {
            let relative_to_output =
                diff_paths(project_root.join(sub_path), &current_workspace_output_dir).context(
                    format!(
                        "Failed to get relative path for workspace dependency {}",
                        dep_name
                    ),
                )?;
            let mut dep_table = Table::new();
            dep_table.insert(
                "path",
                value(relative_to_output.to_string_lossy().to_string()),
            );
            workspace_deps_table.insert(dep_name, Item::Table(dep_table));
        }
        workspace_table.insert("dependencies", Item::Table(workspace_deps_table));
        doc.insert("workspace", Item::Table(workspace_table));

        // Determine transitive dependencies for patching and missing submodules
        let mut transitive_submodule_deps_for_patching: HashMap<String, PathBuf> = HashMap::new();
        let mut patch_queue: Vec<PackageId> = vec![current_root_pkg_id.clone()];
        let mut patch_visited: HashSet<PackageId> = HashSet::new();

        while let Some(pkg_id) = patch_queue.pop() {
            if !patch_visited.insert(pkg_id.clone()) {
                continue;
            }

            let pkg = metadata.packages.iter().find(|p| p.id.repr == pkg_id.repr).unwrap();

            // If it's a local submodule and not already handled as a member or direct dependency
            if let Some(sub_path) = submodule_paths.get(pkg.name.as_str()) {
                if pkg.name.as_str() != current_root_pkg_name.as_str()
                    && !members_for_this_workspace.contains(&pkg.name.to_string())
                    && !direct_submodule_deps.contains_key(pkg.name.as_str())
                {
                    transitive_submodule_deps_for_patching
                        .insert(pkg.name.to_string(), sub_path.clone());
                }
            } else {
                // It's a non-local transitive dependency, check if it needs to be added as a submodule
                if let Some(source) = &pkg.source {
                    if source.repr.starts_with("git+") {
                        let full_repo_url = source.repr.trim_start_matches("git+").to_string();
                        let mut branch_or_tag = "main".to_string(); // Default branch
                        let mut repo_url_to_use = full_repo_url.clone(); // Initialize with the full URL

                        // Extract branch/tag if present
                        if let Some(idx) = full_repo_url.find('#') {
                            let (url_part, ref_part_str) = full_repo_url.split_at(idx);
                            repo_url_to_use = url_part.to_string(); // Update repo_url_to_use
                            let ref_part = ref_part_str.trim_start_matches('#');
                            if ref_part.starts_with("branch=") {
                                branch_or_tag = ref_part.trim_start_matches("branch=").to_string();
                            } else if ref_part.starts_with("tag=") {
                                branch_or_tag = ref_part.trim_start_matches("tag=").to_string();
                            }
                        }

                        let submodule_target_path = submodules_dir.join(pkg.name.as_str());
                        let command = format!(
                            "git submodule add -b {} {} {}",
                            branch_or_tag,
                            repo_url_to_use, // Use the new variable here
                            submodule_target_path.to_string_lossy()
                        );
                        missing_submodule_commands.insert(command);
                    }
                }
            }

            if let Some(deps) = forward_graph.get(&pkg_id) {
                for dep_id in deps {
                    if !patch_visited.contains(dep_id) {
                        patch_queue.push(dep_id.clone());
                    }
                }
            }
        }

        // Add [patch.crates-io] section
        let mut patch_section_table = Table::new();
        let mut crates_io_table = Table::new();
        for (pkg_name, original_path) in &transitive_submodule_deps_for_patching {
            let relative_to_output = diff_paths(
                project_root.join(original_path),
                &current_workspace_output_dir,
            )
            .context(format!(
                "Failed to get relative path for patch entry {}",
                pkg_name
            ))?;
            let mut dep_table = Table::new();
            dep_table.insert(
                "path",
                value(relative_to_output.to_string_lossy().to_string()),
            );
            crates_io_table.insert(pkg_name, Item::Table(dep_table));
        }
        patch_section_table.insert("crates-io", Item::Table(crates_io_table));
        doc.insert("patch", Item::Table(patch_section_table));

        std::fs::write(&new_cargo_toml_path, doc.to_string().as_bytes()).context(format!(
            "Failed to write new Cargo.toml for {}",
            current_root_pkg_name
        ))?;

        println!(
            "Generated new workspace for '{}' at: {:?}",
            current_root_pkg_name, current_workspace_output_dir
        );
        println!("New Cargo.toml written to: {:?}", new_cargo_toml_path);
    }

    if !missing_submodule_commands.is_empty() {
        println!("\n--- Missing Submodules Identified ---");
        println!("The following submodules are required but not present locally.");
        println!("Please execute these commands to add them:");
        let mut sorted_commands: Vec<String> = missing_submodule_commands.into_iter().collect();
        sorted_commands.sort();
        for cmd in sorted_commands {
            println!("{}", cmd);
        }
        println!("-------------------------------------\n");
    }

    Ok(())
}
