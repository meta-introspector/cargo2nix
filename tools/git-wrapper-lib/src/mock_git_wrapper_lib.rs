use crate::git_wrapper_lib_trait::GitWrapperLibTrait;
use crate::git_traits::{GitExecutor, GitRepositoryOperations, GhExecutor, Execv};
use crate::dummy_git_executor::DummyGitExecutor; // Assuming DummyGitExecutor is the mock for GitExecutor
use crate::real_git_repository_operations::mock_git_repository_operations::MockGitRepositoryOperations;
use crate::system_gh_executor::mock_gh_executor::MockGhExecutor;
use crate::execv::mock_execv::MockExecv;
use crate::git_adapters::{GitAdapter, MockGitAdapter}; // Added MockGitAdapter

pub struct MockGitWrapperLib {
    git_executor_impl: DummyGitExecutor,
    git_repo_operations_impl: MockGitRepositoryOperations,
    gh_executor_impl: MockGhExecutor,
    execv_impl: MockExecv,
    git_adapter_impl: MockGitAdapter, // Added
}

impl MockGitWrapperLib {
    pub fn new() -> Self {
        MockGitWrapperLib {
            git_executor_impl: DummyGitExecutor,
            git_repo_operations_impl: MockGitRepositoryOperations,
            gh_executor_impl: MockGhExecutor,
            execv_impl: MockExecv,
            git_adapter_impl: MockGitAdapter::new(), // Added
        }
    }
}

impl GitWrapperLibTrait for MockGitWrapperLib {
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

    fn git_adapter(&self) -> &dyn GitAdapter { // Added
        &self.git_adapter_impl
    }
}
