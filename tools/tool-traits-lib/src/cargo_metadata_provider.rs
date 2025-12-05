use anyhow::Result;
use cargo_metadata::Metadata;
use std::path::Path;

pub trait CargoMetadataProvider: Send + Sync {
    fn provide_metadata(&self, manifest_path: &Path) -> Result<Metadata>;
}

pub struct DummyCargoMetadataProvider;

impl CargoMetadataProvider for DummyCargoMetadataProvider {
    fn provide_metadata(&self, _manifest_path: &Path) -> Result<Metadata> {
        anyhow::bail!(
            "`CargoMetadataProvider` requires the `cargo_metadata_enabled` feature to be enabled."
        )
    }
}

pub struct RealCargoMetadataProvider;

impl CargoMetadataProvider for RealCargoMetadataProvider {
    fn provide_metadata(&self, manifest_path: &Path) -> Result<Metadata> {
        cargo_metadata::MetadataCommand::new()
            .manifest_path(manifest_path)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to get cargo metadata: {}", e))
    }
}
