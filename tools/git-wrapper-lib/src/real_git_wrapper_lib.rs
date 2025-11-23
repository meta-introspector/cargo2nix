use crate::execv::RealExecv;
use crate::git_adapters::{GitAdapter, ShellGitAdapter};
use crate::git_traits::{Execv, GhExecutor, GitExecutor, GitRepositoryOperations};
use crate::git_types::RollupLock;
use crate::git_wrapper_lib_trait::GitWrapperLibTrait;
use crate::real_git_repository_operations::RealGitRepositoryOperations;
use crate::system_gh_executor::SystemGhExecutor;
use crate::system_git_executor::SystemGitExecutor;
use std::sync::{Arc, Mutex}; // Added Mutex // Added

pub struct RealGitWrapperLib {
    git_executor_impl: SystemGitExecutor,
    git_repo_operations_impl: RealGitRepositoryOperations,
    gh_executor_impl: SystemGhExecutor,
    execv_impl: RealExecv,
    git_adapter_impl: ShellGitAdapter,
}

impl RealGitWrapperLib {
    pub fn new() -> Self {
        let execv_arc: Arc<dyn Execv + Send + Sync> = Arc::new(RealExecv); // Create Arc for Execv
        let git_executable_path = std::path::PathBuf::from("git"); // Dummy path
        let rollup_lock_arc: Arc<Mutex<RollupLock>> = Arc::new(Mutex::new(RollupLock::new())); // Dummy RollupLock
        let root_dir = std::path::PathBuf::from("/"); // Dummy root dir

        let git_executor_arc: Arc<dyn GitExecutor + Send + Sync> =
            Arc::new(SystemGitExecutor::new(
                git_executable_path.clone(),
                execv_arc.clone(),
                rollup_lock_arc.clone(),
                root_dir.clone(),
            ));

        RealGitWrapperLib {
            git_executor_impl: SystemGitExecutor::new(
                git_executable_path.clone(),
                execv_arc.clone(),
                rollup_lock_arc.clone(),
                root_dir.clone(),
            ),
            git_repo_operations_impl: RealGitRepositoryOperations::new(git_executor_arc.clone()),
            gh_executor_impl: SystemGhExecutor::new(
                std::path::PathBuf::from("gh"),
                execv_arc.clone(),
            ),
            execv_impl: RealExecv,
            git_adapter_impl: ShellGitAdapter::new(execv_arc.clone()),
        }
    }
}

impl GitWrapperLibTrait for RealGitWrapperLib {
    fn git_executor(&self) -> &dyn GitExecutor {
        &self.git_executor_impl
    }

    fn git_repo_operations(&self) -> &dyn GitRepositoryOperations {
        &self.git_repo_operations_impl
    }

    fn gh_executor(&self) -> &dyn GhExecutor {
        &self.gh_executor_impl
    }

    fn execv_executor(&self) -> &dyn Execv {
        &self.execv_impl
    }

    fn git_adapter(&self) -> &dyn GitAdapter {
        &self.git_adapter_impl
    }
}
