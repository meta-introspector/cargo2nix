use anyhow::Result;
use cargo_edit_lib::CargoMetadataProvider;
#[cfg(feature = "cargo_metadata")]
use cargo_metadata::{Metadata, MetadataCommand};
use std::path::Path;

#[cfg(feature = "cargo_metadata")]
pub struct RealCargoMetadataProvider;

#[cfg(feature = "cargo_metadata")]
impl CargoMetadataProvider for RealCargoMetadataProvider {
    fn get_metadata(&self, manifest_path: &Path) -> Result<Metadata> {
        MetadataCommand::new()
            .manifest_path(manifest_path)
            .no_deps()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to get cargo metadata for {:?}: {}", manifest_path, e))
    }
}

#[cfg(feature = "cargo_metadata")]
pub struct MockCargoMetadataProvider {
    pub mock_metadata: Metadata,
}

#[cfg(feature = "cargo_metadata")]
impl MockCargoMetadataProvider {
    pub fn new() -> Self {
        // Provide a minimal, valid mock Metadata struct
        MockCargoMetadataProvider {
            mock_metadata: Metadata {
                packages: vec![],
                workspace_members: vec![],
                resolve: None,
                target_directory: Path::new("/mock/target").to_path_buf(),
                workspace_root: Path::new("/mock/workspace").to_path_buf(),
                custom_metadata: None,
                version: 1,
            },
        }
    }
}

#[cfg(feature = "cargo_metadata")]
impl CargoMetadataProvider for MockCargoMetadataProvider {
    fn get_metadata(&self, manifest_path: &Path) -> Result<Metadata> {
        println!("[MockCargoMetadataProvider] Providing mock metadata for {:?}", manifest_path);
        Ok(self.mock_metadata.clone())
    }
}

#[cfg(feature = "cargo_metadata")]
pub struct DummyCargoMetadataProvider;

#[cfg(feature = "cargo_metadata")]
impl CargoMetadataProvider for DummyCargoMetadataProvider {
    fn get_metadata(&self, manifest_path: &Path) -> Result<Metadata> {
        anyhow::bail!("DummyCargoMetadataProvider: get_metadata is not implemented. Manifest path: {:?}", manifest_path);
    }
}

#[cfg(not(feature = "cargo_metadata"))]
pub struct NoopCargoMetadataProvider;

#[cfg(not(feature = "cargo_metadata"))]
impl CargoMetadataProvider for NoopCargoMetadataProvider {
    fn get_metadata(&self, manifest_path: &Path) -> Result<Metadata> {
        anyhow::bail!("CargoMetadataProvider is not available because the 'cargo_metadata' feature is not enabled. Manifest path: {:?}", manifest_path);
    }
}
