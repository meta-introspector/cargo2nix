use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use super::submodule_stat::SubmoduleStat;
use super::submodule_stat_provider::SubmoduleStatProvider;
use super::real_submodule_stat_provider::RealSubmoduleStatProvider;
use super::rollup_lock::RollupLock;

// CachedSubmoduleStatProvider will use RollupLock
pub struct CachedSubmoduleStatProvider {
    pub real_provider: RealSubmoduleStatProvider,
    pub rollup_lock: Arc<Mutex<RollupLock>>,
}

impl SubmoduleStatProvider for CachedSubmoduleStatProvider {
    fn get_submodule_stat(&self, path: &Path) -> Result<SubmoduleStat> {
        let mut rollup_lock_guard = self.rollup_lock.lock().unwrap();
        if let Some(cached_stat) = rollup_lock_guard.submodule_stat_cache.get(path) {
            // Check if the cached stat is still valid (e.g., by comparing with real stat)
            // For now, we'll assume if it's in the cache, it's valid.
            // A more robust solution would involve comparing timestamps or hashes.
            // For this task, we'll rely on the user to explicitly update the cache if needed.
            return Ok(cached_stat.clone());
        }
        drop(rollup_lock_guard); // Release lock early

        let real_stat = self.real_provider.get_submodule_stat(path)?;
        let mut rollup_lock_guard = self.rollup_lock.lock().unwrap();
        rollup_lock_guard.submodule_stat_cache.insert(path.to_path_buf(), real_stat.clone());
        Ok(real_stat)
    }

    fn update_submodule_stat(&self, path: PathBuf, stat: SubmoduleStat) -> Result<()> {
        let mut rollup_lock_guard = self.rollup_lock.lock().unwrap();
        rollup_lock_guard.submodule_stat_cache.insert(path, stat);
        Ok(())
    }
}
