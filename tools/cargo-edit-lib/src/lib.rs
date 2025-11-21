use anyhow::Result;
use git_wrapper_lib::git_adapters::GitAdapter;
use cargo_metadata::Metadata;
use std::path::Path;

pub trait CargoMetadataProvider: Send + Sync {
    fn get_metadata(&self, cargo_toml_path: &Path) -> Result<Metadata>;
}

pub trait CargoEditAdapter: Send + Sync {
    fn generate_cargo_config(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
    ) -> Result<String>;
}