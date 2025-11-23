#[cfg(feature = "anyhow_enabled")]
use anyhow::{Context, Result as AnyhowResult};
#[cfg(feature = "anyhow_enabled")]
type Result<T, E = anyhow::Error> = AnyhowResult<T, E>;
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error; // For fallback Result
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>; // Fallback for Result

#[cfg(feature = "serde_enabled")]
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "serde_enabled"))]
#[derive(Debug, Clone, PartialEq, Eq)] // Provide dummy derives if serde is not enabled
pub struct Serialize;
#[cfg(not(feature = "serde_enabled"))]
#[derive(Debug, Clone, PartialEq, Eq)] // Provide dummy derives if serde is not enabled
pub struct Deserialize;

#[cfg(feature = "serde_json_enabled")]
use serde_json;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// From git_operations.rs
#[cfg_attr(feature = "serde_enabled", derive(Debug, Serialize, Deserialize))] // Conditionally derive
#[cfg_attr(not(feature = "serde_enabled"), derive(Debug))] // Fallback derive
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: Option<String>,
}

// From fs_cache.rs
#[cfg_attr(
    feature = "serde_enabled",
    derive(Debug, PartialEq, Clone, Serialize, Deserialize)
)] // Conditionally derive
#[cfg_attr(not(feature = "serde_enabled"), derive(Debug, PartialEq, Clone))] // Fallback derive
pub struct FileMetadata {
    pub modified: SystemTime,
    pub len: u64,
    pub hash: String,
    pub git_object_hash: Option<String>,
    pub is_git_tracked: bool,
}

// From repo_sync_lib/submodule_stat.rs
#[cfg_attr(
    feature = "serde_enabled",
    derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)
)] // Conditionally derive
#[cfg_attr(not(feature = "serde_enabled"), derive(Debug, PartialEq, Eq, Clone))] // Fallback derive
pub struct SubmoduleStat {
    pub head_commit: String,
    pub workdir_hash: String, // A hash representing the state of the working directory (e.g., from git status --porcelain)
}

// From repo_sync_lib/rollup_lock.rs
#[cfg_attr(
    feature = "serde_enabled",
    derive(Debug, Serialize, Deserialize, Default)
)] // Conditionally derive
#[cfg_attr(not(feature = "serde_enabled"), derive(Debug, Default))] // Fallback derive
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
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "serde_json_enabled")]
    pub fn load(root_dir: &Path) -> Result<Self> {
        let path = root_dir.join(Self::FILE_NAME);
        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse {} as JSON", path.display()))
        } else {
            Ok(RollupLock::default())
        }
    }

    #[cfg(not(feature = "serde_json_enabled"))]
    pub fn load(_root_dir: &Path) -> Result<Self> {
        // Fallback if serde_json is not enabled
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "serde_json feature not enabled for RollupLock::load",
        )))
    }

    #[cfg(feature = "serde_json_enabled")]
    pub fn save(&self, root_dir: &Path) -> Result<()> {
        let path = root_dir.join(Self::FILE_NAME);
        let content = serde_json::to_string_pretty(self).with_context(|| {
            format!(
                "Failed to serialize RollupLock to JSON for {}",
                path.display()
            )
        })?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write to {}", path.display()))
    }

    #[cfg(not(feature = "serde_json_enabled"))]
    pub fn save(&self, _root_dir: &Path) -> Result<()> {
        // Fallback if serde_json is not enabled
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "serde_json feature not enabled for RollupLock::save",
        )))
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

// From execv.rs
#[cfg_attr(
    feature = "serde_enabled",
    derive(Serialize, Deserialize, Debug, Clone)
)] // Conditionally derive
#[cfg_attr(not(feature = "serde_enabled"), derive(Debug, Clone))] // Fallback derive
pub struct CapturedCommand {
    pub program: String,
    pub args: Vec<String>,
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
}
