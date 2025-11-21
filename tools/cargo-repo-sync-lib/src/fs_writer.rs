use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};

use crate::fs_cache::{FileSystemStat};
use crate::RollupLock;

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
        fs::write(path, contents)
            .with_context(|| format!("Failed to write file: {:?}", path))
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {:?}", path))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
            .with_context(|| format!("Failed to remove file: {:?}", path))
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove directory: {:?}", path))
    }

    fn save_lock(&self) -> Result<()> {
        // RealFileSystemWriter doesn't manage a lock file directly, so this is a no-op
        Ok(())
    }
}

pub struct CachedFileSystemWriter {
    file_system_stat: Arc<dyn FileSystemStat>, // To get updated metadata after write
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf, // Need root_dir to save the rollup.lock
}

impl CachedFileSystemWriter {
    pub fn new(
        file_system_stat: Arc<dyn FileSystemStat>,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: PathBuf,
    ) -> Self {
        CachedFileSystemWriter {
            file_system_stat,
            rollup_lock,
            root_dir,
        }
    }
}

impl FileSystemWriter for CachedFileSystemWriter {
    fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents)
            .with_context(|| format!("Failed to write file: {:?}", path))?;
        // After writing, update the cache
        let mut rollup_lock = self.rollup_lock.lock().unwrap();
        let metadata = self.file_system_stat.get_metadata(path)?; // Get fresh metadata and hash
        rollup_lock.file_metadata_cache.insert(path.to_path_buf(), metadata);
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {:?}", path))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
            .with_context(|| format!("Failed to remove file: {:?}", path))?;
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
        rollup_lock.file_metadata_cache.retain(|p, _| !p.starts_with(path));
        Ok(())
    }

    fn save_lock(&self) -> Result<()> {
        let rollup_lock = self.rollup_lock.lock().unwrap();
        rollup_lock.save(&self.root_dir)
    }
}
