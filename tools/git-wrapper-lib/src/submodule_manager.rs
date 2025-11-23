use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Fallback for Result

use crate::git_types::RollupLock;
use crate::git_wrapper_lib_trait::GitWrapperLibTrait; // Added

pub struct SubmoduleManager {
    git_wrapper: Arc<dyn GitWrapperLibTrait + Send + Sync>, // Changed
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl SubmoduleManager {
    pub fn new(
        git_wrapper: Arc<dyn GitWrapperLibTrait + Send + Sync>, // Changed
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: PathBuf,
    ) -> Self {
        SubmoduleManager {
            git_wrapper, // Changed
            rollup_lock,
            root_dir,
        }
    }

    // Add methods here that use the injected traits to perform submodule operations
    // For example:
    pub fn add_submodule(&self, repo_url: &str, path: &Path) -> Result<()> {
        // Example of using git_repo_operations
        self.git_wrapper.git_repo_operations().git_submodule_add(
            &self.root_dir,
            repo_url,
            path.to_str().unwrap(),
            None,
            None,
        )?; // Changed
        Ok(())
    }

    pub fn update_submodules(&self) -> Result<()> {
        self.git_wrapper
            .git_repo_operations()
            .git_submodule_update(&self.root_dir, true, true)?; // Changed
        Ok(())
    }

    pub fn status_submodules(&self) -> Result<String> {
        let output = self
            .git_wrapper
            .git_repo_operations()
            .git_submodule_status(&self.root_dir)?; // Changed
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
