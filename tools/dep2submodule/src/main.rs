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
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(feature = "toml_edit_enabled")]
use toml_edit::{value, DocumentMut, Item, Table};
#[cfg(feature = "walkdir_enabled")]
use walkdir::WalkDir;

//use crate::metadata_provider::{CargoMetadataProvider, Metadata, Package, PackageId};

//#[cfg(feature = "real_cargo_metadata")]
//use crate::metadata_provider::RealCargoMetadataProvider;
//#[cfg(not(feature = "real_cargo_metadata"))]
//use DummyCargoMetadataProvider;

#[cfg(feature = "clap_enabled")]
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

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
struct Args {
    project_root: PathBuf,
    submodules_dir: PathBuf,
    root_cargo_toml: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
impl Args {
    fn parse() -> Self {
        Args {
            project_root: PathBuf::from("."),
            submodules_dir: PathBuf::from("submodules"),
            root_cargo_toml: PathBuf::from("Cargo.toml"),
        }
    }
}


#[cfg(all(feature = "clap_enabled", feature = "toml_edit_enabled", feature = "walkdir_enabled"))]
fn main() -> Result<()> {
    let args = Args::parse();

    let project_root = args
        .project_root
        .canonicalize()
        .context("Failed to canonicalize project_root")?;
    let submodules_dir = project_root.join(&args.submodules_dir);
    let root_cargo_toml_path = project_root.join(&args.root_cargo_toml);

    println!("Scanning submodules in: {}", submodules_dir.display());

    let mut workspace_dependencies: HashMap<String, String> = HashMap::new();

    #[cfg(feature = "real_cargo_metadata")]
    let metadata_provider = RealCargoMetadataProvider;
    #[cfg(not(feature = "real_cargo_metadata"))]
    let metadata_provider = DummyCargoMetadataProvider;

    for entry in WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        let submodule_root = cargo_toml_path.parent().unwrap();

        // Try to get cargo metadata for the submodule
        let metadata = metadata_provider
            .provide_metadata(cargo_toml_path)
            .context(format!(
                "Failed to get metadata for submodule Cargo.toml: {:?}",
                cargo_toml_path
            ))?;

        // Check if this submodule is a workspace itself
        let workspace_root_path = PathBuf::from(&metadata.workspace_root);
        if workspace_root_path == submodule_root {
            // This is a submodule that is also a workspace
            println!("Found submodule workspace: {}", submodule_root.display());
            for member_id in &metadata.workspace_members {
                if let Some(member_package) = metadata
                    .packages
                    .iter()
                    .find(|p| p.id.repr == member_id.repr)
                {
                    let member_manifest_path = PathBuf::from(&member_package.manifest_path);
                    let member_crate_root = member_manifest_path.parent().unwrap();
                    let relative_path = pathdiff::diff_paths(member_crate_root, &project_root)
                        .context(format!(
                            "Failed to get relative path for member crate {}",
                            member_package.name
                        ))?;
                    workspace_dependencies.insert(
                        member_package.name.to_string(),
                        format!("{{ path = \"{}\" }}", relative_path.display()),
                    );
                }
            }
        } else {
            // It's a regular package within a submodule
            if let Some(package) = metadata.packages.get(0) {
                let relative_path = pathdiff::diff_paths(submodule_root, &project_root).context(
                    format!("Failed to get relative path for package {}", package.name),
                )?;
                workspace_dependencies.insert(
                    package.name.to_string(),
                    format!("{{ path = \"{}\" }}", relative_path.display()),
                );
            }
        }
    }

    // Read the root Cargo.toml
    let root_cargo_toml_content = fs::read_to_string(&root_cargo_toml_path).with_context(|| {
        format!(
            "Failed to read root Cargo.toml at {}",
            root_cargo_toml_path.display()
        )
    })?;
    let mut root_doc = root_cargo_toml_content
        .parse::<DocumentMut>()
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
    fs::write(&root_cargo_toml_path, root_doc.to_string().as_bytes()).with_context(|| {
        format!(
            "Failed to write to root Cargo.toml at {}",
            root_cargo_toml_path.display()
        )
    })?;

    println!(
        "Successfully updated [workspace.dependencies] in {}",
        root_cargo_toml_path.display()
    );

    Ok(())
}

#[cfg(not(all(feature = "clap_enabled", feature = "toml_edit_enabled", feature = "walkdir_enabled")))]
fn main() -> Result<()> {
    println!("`dep2submodule` is running in dummy mode. Enable `clap_enabled`, `toml_edit_enabled`, and `walkdir_enabled` features for full functionality.");
    Ok(())
}

