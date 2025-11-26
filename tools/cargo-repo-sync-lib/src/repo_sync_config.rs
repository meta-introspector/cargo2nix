use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct RepoSyncConfig {
    pub root_dir: PathBuf,
    pub target_org: String,
    pub target_branch: String,
    pub output_file: Option<PathBuf>,
    pub json_input_file: Option<PathBuf>,
    pub dry_run: bool,
    pub json_log_file: Option<PathBuf>,
    pub report: bool,
    pub use_pure_rust_git: bool,
}

impl RepoSyncConfig {
    pub fn load_from_file(_path: &Path) -> Result<Self> {
        unimplemented!()
    }
}
