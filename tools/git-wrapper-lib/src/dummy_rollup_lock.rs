use super::git_types::RollupLock;
use anyhow::Result;
use std::path::Path; // Assuming RollupLock is in git_types

pub struct DummyRollupLock;

impl DummyRollupLock {
    pub fn load(_root_dir: &Path) -> Result<RollupLock> {
        Ok(RollupLock::new())
    }
}
