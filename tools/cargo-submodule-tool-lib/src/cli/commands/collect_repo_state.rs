use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::executors::{RepoStateCollector, RealRepoStateCollector}; // Use our re-exported RepoStateCollector
use crate::executors::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use crate::executors::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use crate::executors::SystemGitExecutor; // Added for non-git2 case
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyGitExecutor; // Use our dummy GitExecutor
#[cfg(feature = "git_enabled")]
use crate::executors::RealExecv; // Use our re-exported RealExecv
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
#[cfg(feature = "git_enabled")]
use crate::executors::RollupLock; // Use our re-exported RollupLock
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyRollupLock as RollupLock; // Use dummy for RollupLock when git is not enabled
use crate::fs_cache::RealFileSystemStat; // Still in cargo-submodule-tool-lib
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
#[cfg(not(feature = "nix_generation"))]
use crate::analysis::cargo_metadata_provider::DummyCargoMetadataProvider;


pub fn collect_repo_state_command(project_root: PathBuf) -> Result<()> {
    println!("Collecting repository state for: {:?}", project_root);

    // Initialize GitExecutor
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let file_system_stat = Arc::new(RealFileSystemStat {});
    let rollup_lock = Arc::new(Mutex::new(RollupLock::load(&project_root)?)); // Use load method

    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git_enabled")]
        {
            Arc::new(PureRustGitExecutor::new(rollup_lock.clone(), project_root.clone()))
        }
        #[cfg(not(feature = "git_enabled"))]
        {
            Arc::new(DummyGitExecutor) // Use the dummy struct directly
        }
    };

    let cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync> = {
        #[cfg(feature = "nix_generation")]
        {
            Box::new(RealCargoMetadataProvider)
        }
        #[cfg(not(feature = "nix_generation"))]
        {
            Box::new(DummyCargoMetadataProvider)
        }
    };

    let collector = RealRepoStateCollector::new(git_executor, cargo_metadata_provider);
    let repo_state = collector.collect_repo_state(&project_root)?;

    println!("--- Collected Repository State ---");
    println!("{:#?}", repo_state);

    Ok(())
}
