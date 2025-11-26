use anyhow::Result;
use std::path::Path;
use super::git_types::RollupLock; // Assuming RollupLock is in git_types

pub struct DummyRollupLock;

impl DummyRollupLock {
    pub fn load(_root_dir: &Path) -> Result<RollupLock> {
        Ok(RollupLock::new())
    }
}
