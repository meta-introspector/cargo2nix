use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::dummy_rollup_lock::DummyRollupLock;
use git_wrapper_lib::git_types::RollupLock; // Use our re-exported RollupLock
use crate::fs_cache::FileSystemStat;

pub trait FileSystemWriter {
    fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn remove_dir_all(&self, path: &Path) -> Result<()>;
    fn save_lock(&self) -> Result<()>; // Add save_lock method
                                       // Add other file system write operations as needed
}

pub struct RealFileSystemWriter;

impl FileSystemWriter for RealFileSystemWriter {
    fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents).with_context(|| format!("Failed to write file: {:?}", path))
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).with_context(|| format!("Failed to create directory: {:?}", path))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path).with_context(|| format!("Failed to remove file: {:?}", path))
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path).with_context(|| format!("Failed to remove directory: {:?}", path))
    }

    fn save_lock(&self) -> Result<()> {
        // RealFileSystemWriter doesn't manage a lock file directly, so this is a no-op
        Ok(())
    }
}

#[cfg(feature = "git_enabled")]
pub struct CachedFileSystemWriter {
    pub inner: Arc<dyn FileSystemStat>,
    pub rollup_lock: Arc<Mutex<RollupLock>>,
    pub root_dir: PathBuf, // Need root_dir to save the rollup.lock
}

#[cfg(feature = "git_enabled")]
impl CachedFileSystemWriter {
    pub fn new(
        file_system_stat: Arc<dyn FileSystemStat>,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: PathBuf,
    ) -> Self {
        CachedFileSystemWriter {
            inner,
            rollup_lock,
            root_dir,
        }
    }
}

#[cfg(feature = "git_enabled")]
impl FileSystemWriter for CachedFileSystemWriter {
    fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents).with_context(|| format!("Failed to write file: {:?}", path))?;
        // After writing, update the cache
        let mut rollup_lock = self.rollup_lock.lock().unwrap();
        let metadata = self.file_system_stat.get_metadata(path)?; // Get fresh metadata and hash
        rollup_lock
            .file_metadata_cache
            .insert(path.to_path_buf(), metadata);
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).with_context(|| format!("Failed to create directory: {:?}", path))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path).with_context(|| format!("Failed to remove file: {:?}", path))?;
        // After removing, invalidate cache entry
        let mut rollup_lock = self.rollup_lock.lock().unwrap();
        rollup_lock.file_metadata_cache.remove(path);
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove directory: {:?}", path))?;
        // Invalidate all cache entries under this directory.
        let mut rollup_lock = self.rollup_lock.lock().unwrap();
        rollup_lock
            .file_metadata_cache
            .retain(|p, _| !p.starts_with(path));
        Ok(())
    }

    fn save_lock(&self) -> Result<()> {
        let rollup_lock = self.rollup_lock.lock().unwrap();
        rollup_lock.save(&self.root_dir)
    }
}

#[cfg(not(feature = "git_enabled"))]
pub struct CachedFileSystemWriter;

#[cfg(not(feature = "git_enabled"))]
impl CachedFileSystemWriter {
    pub fn new(
        _file_system_stat: Arc<dyn FileSystemStat>,
        _rollup_lock: Arc<Mutex<RollupLock>>,
        _root_dir: PathBuf,
    ) -> Self {
        CachedFileSystemWriter {}
    }
}

#[cfg(not(feature = "git_enabled"))]
impl FileSystemWriter for CachedFileSystemWriter {
    fn write_file(&self, path: &Path, _contents: &[u8]) -> Result<()> {
        println!("Dummy CachedFileSystemWriter: write_file to {:?}", path);
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        println!("Dummy CachedFileSystemWriter: create_dir_all {:?}", path);
        Ok(())
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        println!("Dummy CachedFileSystemWriter: remove_file {:?}", path);
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        println!("Dummy CachedFileSystemWriter: remove_dir_all {:?}", path);
        Ok(())
    }

    fn save_lock(&self) -> Result<()> {
        println!("Dummy CachedFileSystemWriter: save_lock");
        Ok(())
    }
}
