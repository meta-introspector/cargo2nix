use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::fs;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use git2::Repository; // Add git2 import

use crate::RollupLock;

pub trait FileSystemStat: Send + Sync {
    fn get_metadata(&self, path: &Path) -> Result<FileMetadata>;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub len: u64,
    pub hash: String,
    pub git_object_hash: Option<String>,
    pub is_git_tracked: bool,
}

pub struct RealFileSystemStat {
    repo: Arc<Mutex<Repository>>, // Change to Arc<Mutex<Repository>>
}

impl RealFileSystemStat {
    pub fn new(repo: Arc<Mutex<Repository>>) -> Self {
        RealFileSystemStat { repo }
    }
}

impl Clone for RealFileSystemStat {
    fn clone(&self) -> Self {
        RealFileSystemStat {
            repo: self.repo.clone(),
        }
    }
}

impl FileSystemStat for RealFileSystemStat {
    fn get_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let repo = self.repo.lock().unwrap(); // Lock the mutex to access the repository
        let metadata = fs::metadata(path)?;
        let file_content = fs::read(path)?;
        let hash = calculate_file_hash(&file_content);

        let mut git_object_hash = None;
        let mut is_git_tracked = false;

        // Check if the path is tracked by Git
        if let Ok(status) = repo.status_file(path) {
            if status.is_empty() { // No changes, so it's tracked and clean
                is_git_tracked = true;
                // Try to get the Oid for the file/blob
                if let Ok(relative_path) = path.strip_prefix(repo.workdir().context("No workdir for repo")?) {
                    if let Ok(object) = repo.revparse_single(&format!("HEAD:{}", relative_path.display())) {
                        git_object_hash = Some(object.id().to_string());
                    }
                }
            } else if status.is_wt_new() || status.is_wt_modified() || status.is_wt_deleted() || status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                // It's tracked but has changes, so we still consider it tracked
                is_git_tracked = true;
                // For changed files, the HEAD object hash might not reflect the current content,
                // so we might leave git_object_hash as None or get the hash of the current content.
                // For now, let's leave it as None if it's modified, as the file hash will capture content changes.
            }
        }


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
