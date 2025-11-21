use std::{
    sync::{Arc, Mutex},
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::git_traits::{GitExecutor, GitRepositoryOperations, GhExecutor, Execv};
use crate::git_types::RollupLock;

pub struct SubmoduleManager {
    git_executor: Arc<dyn GitExecutor + Send + Sync>,
    git_repo_operations: Arc<dyn GitRepositoryOperations + Send + Sync>,
    gh_executor: Arc<dyn GhExecutor + Send + Sync>,
    execv_executor: Arc<dyn Execv + Send + Sync>,
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl SubmoduleManager {
    pub fn new(
        git_executor: Arc<dyn GitExecutor + Send + Sync>,
        git_repo_operations: Arc<dyn GitRepositoryOperations + Send + Sync>,
        gh_executor: Arc<dyn GhExecutor + Send + Sync>,
        execv_executor: Arc<dyn Execv + Send + Sync>,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: PathBuf,
    ) -> Self {
        SubmoduleManager {
            git_executor,
            git_repo_operations,
            gh_executor,
            execv_executor,
            rollup_lock,
            root_dir,
        }
    }

    // Add methods here that use the injected traits to perform submodule operations
    // For example:
    pub fn add_submodule(&self, repo_url: &str, path: &Path) -> Result<()> {
        // Example of using git_repo_operations
        self.git_repo_operations.git_submodule_add(&self.root_dir, repo_url, path.to_str().unwrap(), None, None)?;
        Ok(())
    }

    pub fn update_submodules(&self) -> Result<()> {
        self.git_repo_operations.git_submodule_update(&self.root_dir, true, true)?;
        Ok(())
    }

    pub fn status_submodules(&self) -> Result<String> {
        let output = self.git_repo_operations.git_submodule_status(&self.root_dir)?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
