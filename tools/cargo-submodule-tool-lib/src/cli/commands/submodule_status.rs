use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::cli::args::SubmoduleStatusArgs;
use git_wrapper_lib::git_traits::GhExecutor;
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
use git_wrapper_lib::system_git_executor::SystemGitExecutor; // Added for non-git2 case
use git_wrapper_lib::dummy_git_executor::DummyGitExecutor; // Added for default dummy git
use git_wrapper_lib::git_traits::GitExecutor;
use git_wrapper_lib::execv::RealExecv;
use crate::fs_cache::RealFileSystemStat; // Still in cargo-submodule-tool-lib
use git_wrapper_lib::git_types::RollupLock;

pub fn run_submodule_status_command(args: &SubmoduleStatusArgs) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;

    // Initialize common dependencies
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));

    // Initialize GitExecutor
    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git2")]
        {
            Arc::new(PureRustGitExecutor::new(rollup_lock_arc.clone(), root_dir.clone()))
        }
        #[cfg(not(feature = "git2"))]
        {
            Arc::new(DummyGitExecutor::new())
        }
    };

    let status_output = git_executor.status(&root_dir)?;
    println!("{}", status_output);

    Ok(())
}
