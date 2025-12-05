use crate::git_adapters::GitAdapter;
use crate::git_traits::{Execv, GhExecutor, GitExecutor, GitRepositoryOperations}; // Added

pub trait GitWrapperLibTrait: Send + Sync {
    fn git_executor(&self) -> &dyn GitExecutor;
    fn git_repo_operations(&self) -> &dyn GitRepositoryOperations;
    fn gh_executor(&self) -> &dyn GhExecutor;
    fn execv_executor(&self) -> &dyn Execv;
    fn git_adapter(&self) -> &dyn GitAdapter; // Added
}
