use anyhow::{Result, Context};
use std::collections::HashSet;
use std::path::Path;
use cargo_metadata::{MetadataCommand, Package};

pub trait CargoMetadataProvider: Send + Sync {
    /// Retrieves a set of package names from a Cargo.lock file.
    fn get_package_names_from_lock_file(&self, cargo_lock_file: &Path) -> Result<HashSet<String>>;

    /// Retrieves a list of packages in the workspace defined by the given manifest path.
    fn get_workspace_packages(&self, manifest_path: &Path) -> Result<Vec<Package>>;
}

pub struct RealCargoMetadataProvider;

impl CargoMetadataProvider for RealCargoMetadataProvider {
    fn get_package_names_from_lock_file(&self, cargo_lock_file: &Path) -> Result<HashSet<String>> {
        let metadata = MetadataCommand::new()
            .manifest_path(cargo_lock_file)
            .no_deps() // Only parse the lock file itself, not its dependencies
            .exec()
            .with_context(|| format!("Failed to get cargo metadata for lock file: {}", cargo_lock_file.display()))?;

        let package_names: HashSet<String> = metadata.packages.into_iter()
            .map(|pkg| pkg.name.to_string())
            .collect();

        Ok(package_names)
    }

    fn get_workspace_packages(&self, manifest_path: &Path) -> Result<Vec<Package>> {
        let metadata = MetadataCommand::new()
            .manifest_path(manifest_path)
            .exec()
            .with_context(|| format!("Failed to get cargo metadata for manifest: {}", manifest_path.display()))?;
        
        Ok(metadata.packages)
    }
}
