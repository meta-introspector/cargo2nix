use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::fs;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use md5;

use git_wrapper_lib::git_types::{FileMetadata, RollupLock};
use git_wrapper_lib::git_traits::GitExecutor;

pub trait FileSystemStat: Send + Sync {
    fn get_metadata(&self, path: &Path) -> Result<FileMetadata>;
}

pub struct RealFileSystemStat {
    git_executor: Arc<dyn GitExecutor + Send + Sync>,
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl RealFileSystemStat {
    pub fn new(git_executor: Arc<dyn GitExecutor + Send + Sync>, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: PathBuf) -> Self {
        RealFileSystemStat { git_executor, rollup_lock, root_dir }
    }
}

impl Clone for RealFileSystemStat {
    fn clone(&self) -> Self {
        RealFileSystemStat {
            git_executor: self.git_executor.clone(),
            rollup_lock: self.rollup_lock.clone(),
            root_dir: self.root_dir.clone(),
        }
    }
}

impl FileSystemStat for RealFileSystemStat {
    fn get_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let metadata = fs::metadata(path)?;
        let file_content = fs::read(path)?;
        let hash = calculate_file_hash(&file_content);

        let (is_git_tracked, git_object_hash) = self.git_executor.get_file_git_info(&self.root_dir, path)?;

        Ok(FileMetadata {
            modified: metadata.modified()?,
            len: metadata.len(),
            hash,
            git_object_hash,
            is_git_tracked,
        })
    }
}

pub struct CachedFileSystemStat {
    pub inner: Arc<dyn FileSystemStat>,
    pub rollup_lock: Arc<Mutex<RollupLock>>,
    pub root_dir: PathBuf,
}

impl CachedFileSystemStat {
    pub fn new(inner: Arc<dyn FileSystemStat>, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: PathBuf) -> Self {
        CachedFileSystemStat {
            inner,
            rollup_lock,
            root_dir,
        }
    }
}

impl FileSystemStat for CachedFileSystemStat {
    fn get_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let mut rollup_lock_guard = self.rollup_lock.lock().unwrap();
        let relative_path = path.strip_prefix(&self.root_dir).unwrap_or(path);

        if let Some(metadata) = rollup_lock_guard.get_metadata(relative_path) {
            // Check if the file has actually changed on disk
            let current_metadata = self.inner.get_metadata(path)?;
            if current_metadata == *metadata {
                return Ok(metadata.clone());
            }
        }

        let metadata = self.inner.get_metadata(path)?;
        rollup_lock_guard.set_metadata(relative_path.to_path_buf(), metadata.clone());
        Ok(metadata)
    }
}

pub fn calculate_file_hash(content: &[u8]) -> String {
    format!("{:x}", md5::compute(content))
}
