#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error; // For fallback Result
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>; // Fallback for Result

use crate::git_traits::GhExecutor;

pub struct MockGhExecutor;

impl GhExecutor for MockGhExecutor {
    fn repo_fork(&self, _repo_url: &str, _target_org: &str) -> Result<()> {
        // Mock implementation: always succeed
        Ok(())
    }

    fn repo_view(&self, _forked_repo_url: &str) -> Result<bool> {
        // Mock implementation: always return true (repo exists)
        Ok(true)
    }
}
