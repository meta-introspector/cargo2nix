use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
use crate::fs_cache::FileMetadata;
use super::submodule_stat::SubmoduleStat; // Import SubmoduleStat

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RollupLock {
    pub file_metadata_cache: HashMap<PathBuf, FileMetadata>,
    pub submodule_stat_cache: HashMap<PathBuf, SubmoduleStat>, // New field for submodule stats
    pub git_tree_cache: HashMap<PathBuf, String>, // New field for caching Git tree hashes
    pub last_snapshot_time: Option<SystemTime>,

    // Granular hashes for cache invalidation
    pub project_root_hash: Option<String>,
    pub crate_hashes: HashMap<PathBuf, String>, // Path to Cargo.toml -> hash of crate content
    pub submodule_hashes: HashMap<PathBuf, String>, // Path to submodule -> hash of submodule state
    pub cargo_toml_hashes: HashMap<PathBuf, String>, // Path to Cargo.toml -> hash of file content
    pub rust_file_hashes: HashMap<PathBuf, String>, // Path to .rs file -> hash of file content
}

impl RollupLock {
    const FILE_NAME: &'static str = "rollup.lock";

    pub fn load(root_dir: &Path) -> Result<Self> {
        let path = root_dir.join(Self::FILE_NAME);
        if path.exists() {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse {} as JSON", path.display()))
        } else {
            Ok(RollupLock::default())
        }
    }

    pub fn save(&self, root_dir: &Path) -> Result<()> {
        let path = root_dir.join(Self::FILE_NAME);
        let content = serde_json::to_string_pretty(self)
            .with_context(|| format!("Failed to serialize RollupLock to JSON for {}", path.display()))?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))
    }

    pub fn get_metadata(&self, path: &Path) -> Option<&FileMetadata> {
        self.file_metadata_cache.get(path)
    }

    pub fn set_metadata(&mut self, path: PathBuf, metadata: FileMetadata) {
        self.file_metadata_cache.insert(path, metadata);
    }

    pub fn get_submodule_stat(&self, path: &Path) -> Option<&SubmoduleStat> {
        self.submodule_stat_cache.get(path)
    }

    pub fn set_submodule_stat(&mut self, path: PathBuf, stat: SubmoduleStat) {
        self.submodule_stat_cache.insert(path, stat);
    }
}
