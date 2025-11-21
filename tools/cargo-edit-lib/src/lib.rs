use anyhow::Result;
use git_wrapper_lib::git_adapters::GitAdapter;
use cargo_metadata::Metadata;
use std::path::{Path, PathBuf}; // Added PathBuf

pub trait CargoMetadataProvider: Send + Sync {
    fn get_metadata(&self, cargo_toml_path: &Path) -> Result<Metadata>;
}

pub trait WorkspaceInfoProvider: Send + Sync {
    fn parse_members_file(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
        project_root: &Path,
    ) -> Result<Vec<WorkspaceInfo>>;
}

pub struct WorkspaceInfo {
    pub member_crates: Vec<String>,
    pub submodule_base_path_rel: PathBuf,
}

pub trait CargoEditAdapter: Send + Sync {
    fn generate_cargo_config(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
    ) -> Result<String>;
}