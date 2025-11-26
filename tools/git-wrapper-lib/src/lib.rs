pub mod dummy_git_executor;
pub mod executors; // New
pub mod execv;
pub mod git_adapters; // New
pub mod git_traits;
pub mod git_types;
pub mod git_wrapper_lib_trait;
pub mod pure_rust_git_executor;
pub mod real_git_repository_operations;
pub mod repo_sync_lib;
pub mod repo_state_collector;
pub mod submodule_manager;
pub mod system_gh_executor;
pub mod system_git_executor; // Added

pub mod dummy_rollup_lock; // Moved here

pub use git_wrapper_lib_trait::GitWrapperLibTrait; // Added
pub use dummy_rollup_lock::*;
pub use repo_state_collector::*;
pub mod mock_git_wrapper_lib;
pub mod real_git_wrapper_lib; // Added // Added
