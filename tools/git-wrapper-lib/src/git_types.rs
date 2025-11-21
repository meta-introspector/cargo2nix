use anyhow::Context; // Result is not directly used in this file's imports
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;

// From git_operations.rs
#[derive(Debug, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: Option<String>,
}

// From fs_cache.rs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub len: u64,
    pub hash: String,
    pub git_object_hash: Option<String>,
    pub is_git_tracked: bool,
}

// From repo_sync_lib/submodule_stat.rs
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SubmoduleStat {
    pub head_commit: String,
    pub workdir_hash: String, // A hash representing the state of the working directory (e.g., from git status --porcelain)
}

// From repo_sync_lib/rollup_lock.rs
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

    pub fn new() -> Self {
        Default::default()
    }

    pub fn load(root_dir: &Path) -> anyhow::Result<Self> {
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

    pub fn save(&self, root_dir: &Path) -> anyhow::Result<()> {
        let path = root_dir.join(Self::FILE_NAME);
        let content = serde_json::to_string_pretty(self)
            .with_context(|| format!("Failed to serialize RollupLock to JSON for {}", path.display()))?;
        std::fs::write(&path, content)
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

// From execv.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapturedCommand {
    pub program: String,
    pub args: Vec<String>,
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
}