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
pub mod submodule_manager;
pub mod system_gh_executor;
pub mod system_git_executor; // Added
pub use git_wrapper_lib_trait::GitWrapperLibTrait; // Added
pub mod mock_git_wrapper_lib;
pub mod real_git_wrapper_lib; // Added // Added
