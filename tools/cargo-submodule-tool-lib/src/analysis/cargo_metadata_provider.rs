use anyhow::Result;
use std::path::Path;

// Conditionally import the real CargoMetadataProvider trait and implementations
#[cfg(feature = "nix_generation")] // Using nix_generation feature to enable real cargo_metadata
pub use cargo_edit_lib::{CargoMetadataProvider, RealCargoMetadataProvider};

// Dummy CargoMetadataProvider for when nix_generation feature is not active
#[cfg(not(feature = "nix_generation"))]
pub trait CargoMetadataProvider: Send + Sync {
    fn get_metadata(&self, cargo_toml_path: &Path) -> Result<DummyMetadata> {
        anyhow::bail!("Dummy CargoMetadataProvider: get_metadata not implemented for {:?}", cargo_toml_path)
    }
}

#[cfg(not(feature = "nix_generation"))]
#[derive(Debug)]
pub struct DummyMetadata; // Define a dummy Metadata struct

#[cfg(not(feature = "nix_generation"))]
pub struct RealCargoMetadataProvider; // Dummy struct for RealCargoMetadataProvider

#[cfg(not(feature = "nix_generation"))]
impl CargoMetadataProvider for RealCargoMetadataProvider {}

#[cfg(not(feature = "nix_generation"))]
pub struct DummyCargoMetadataProvider;

#[cfg(not(feature = "nix_generation"))]
impl CargoMetadataProvider for DummyCargoMetadataProvider {}
