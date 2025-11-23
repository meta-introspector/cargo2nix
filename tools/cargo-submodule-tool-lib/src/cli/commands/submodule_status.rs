use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::cli::args::SubmoduleStatusArgs;
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyGitExecutor; // Use our dummy GitExecutor
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyRollupLock as RollupLock;
use crate::executors::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use crate::executors::RealExecv; // Use our re-exported RealExecv
#[cfg(feature = "git_enabled")]
use crate::executors::RollupLock; // Use our re-exported RollupLock
use crate::fs_cache::RealFileSystemStat; // Still in cargo-submodule-tool-lib
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::git_traits::GhExecutor;
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::system_git_executor::SystemGitExecutor; // Added for non-git2 case // Use dummy for RollupLock when git is not enabled

pub fn run_submodule_status_command(args: &SubmoduleStatusArgs) -> Result<()> {
    let root_dir = args
        .root_dir
        .canonicalize()
        .context("Failed to canonicalize root_dir")?;

    // Initialize common dependencies
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));

    // Initialize GitExecutor
    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git_enabled")]
        {
            Arc::new(PureRustGitExecutor::new(
                rollup_lock_arc.clone(),
                root_dir.clone(),
            ))
        }
        #[cfg(not(feature = "git_enabled"))]
        {
            Arc::new(DummyGitExecutor) // Use the dummy struct directly
        }
    };

    let status_output = git_executor.status(&root_dir)?;
    println!("{}", status_output);

    Ok(())
}
