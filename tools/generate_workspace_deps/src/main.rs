use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::metadata_provider::CargoMetadataProvider;

// Mini metadata provider for local Cargo.toml files within submodules
mod metadata_provider {
    use anyhow::Result;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PackageId {
        pub repr: String,
    }

    #[derive(Debug, Clone)]
    pub struct Package {
        pub id: PackageId,
        pub name: String,
        pub version: String,
        pub manifest_path: PathBuf,
        // No dependencies or source needed for this specific use case
    }

    #[derive(Debug)]
    pub struct Metadata {
        pub packages: Vec<Package>,
    }

    pub trait CargoMetadataProvider: Send + Sync {
        fn provide_metadata(&self, project_root: &Path) -> Result<Metadata>;
    }

    pub struct RealCargoMetadataProvider;

    impl CargoMetadataProvider for RealCargoMetadataProvider {
        fn provide_metadata(&self, project_root: &Path) -> Result<Metadata> {
            let metadata = cargo_metadata::MetadataCommand::new()
                .current_dir(project_root)
                .no_deps() // We only need package info, not the full dependency graph
                .exec()?;

            let packages: Vec<Package> = metadata.packages.into_iter().map(|p| Package {
                id: PackageId { repr: p.id.repr },
                name: p.name.to_string(),
                version: p.version.to_string(),
                manifest_path: p.manifest_path.into(),
            }).collect();

            Ok(Metadata { packages })
        }
    }
}

fn main() -> Result<()> {
    // This root_dir needs to be correct for the context where this binary is run.
    // In Nix flakes, this usually means the flake root.
    // For local development, it might be the project root of cargo2nix.
    // For now, hardcode to the current working directory, assuming it's run from the project root.
    let root_dir = std::env::current_dir().context("Failed to get current directory")?;
    let submodules_dir = root_dir.join("submodules");

    println!("[workspace.dependencies]");

    let mut submodule_path_map: HashMap<String, PathBuf> = HashMap::new();
    let metadata_provider = metadata_provider::RealCargoMetadataProvider;

    for entry in WalkDir::new(&submodules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        
        let submodule_package_metadata = metadata_provider
            .provide_metadata(cargo_toml_path.parent().unwrap())
            .context(format!(
                "Failed to get metadata for submodule Cargo.toml: {:?}",
                cargo_toml_path
            ))?;

        if let Some(pkg) = submodule_package_metadata.packages.first() {
            let relative_path = pathdiff::diff_paths(cargo_toml_path.parent().unwrap(), &root_dir)
                .context(format!(
                    "Failed to get relative path for submodule: {:?}",
                    cargo_toml_path
                ))?;
            submodule_path_map.insert(pkg.name.clone(), relative_path);
        }
    }

    // Sort the keys for consistent output
    let mut sorted_keys: Vec<&String> = submodule_path_map.keys().collect();
    sorted_keys.sort();

    for crate_name in sorted_keys {
        if let Some(relative_path) = submodule_path_map.get(crate_name) {
            let path_str = format!("./{}", relative_path.display());
            println!("{} = {{ path = \"{}\" }}", crate_name, path_str);
        }
    }

    Ok(())
}