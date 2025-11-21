use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use git_wrapper_lib::repo_state_collector::{RepoStateCollector, RealRepoStateCollector};
use git_wrapper_lib::git_traits::GitExecutor;
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
use git_wrapper_lib::system_git_executor::SystemGitExecutor; // Added for non-git2 case
use git_wrapper_lib::dummy_git_executor::DummyGitExecutor; // Added for default dummy git
use git_wrapper_lib::execv::RealExecv;
use git_wrapper_lib::git_types::RollupLock;
use crate::fs_cache::RealFileSystemStat; // Still in cargo-submodule-tool-lib
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider, DummyCargoMetadataProvider};

pub fn collect_repo_state_command(project_root: PathBuf) -> Result<()> {
    println!("Collecting repository state for: {:?}", project_root);

    // Initialize GitExecutor
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let file_system_stat = Arc::new(RealFileSystemStat {});
    let rollup_lock = Arc::new(Mutex::new(RollupLock::new()));

    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git2")]
        {
            Arc::new(PureRustGitExecutor::new(rollup_lock.clone(), project_root.clone()))
        }
        #[cfg(not(feature = "git2"))]
        {
            Arc::new(DummyGitExecutor::new())
        }
    };

    let cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync> = Box::new(RealCargoMetadataProvider);

    let collector = RealRepoStateCollector::new(git_executor, cargo_metadata_provider);
    let repo_state = collector.collect_repo_state(&project_root)?;

    println!("--- Collected Repository State ---");
    println!("{:#?}", repo_state);

    Ok(())
}
