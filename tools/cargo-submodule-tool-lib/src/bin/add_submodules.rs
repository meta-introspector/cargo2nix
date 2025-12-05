use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use anyhow::{Context, Result};
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::executors::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::executors::DummyGitExecutor;
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::executors::DummyRollupLock as RollupLock; // Use dummy for RollupLock when git is not enabled
use git_wrapper_lib::executors::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::executors::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::executors::RealExecv; // Use our re-exported RealExecv
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::executors::RollupLock; // Use our re-exported RollupLock
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex}; // Use our dummy GitExecutor

fn main() -> Result<()> {
    let root_dir = PathBuf::from(".")
        .canonicalize()
        .context("Failed to canonicalize root_dir")?;
    let repo_url = "https://github.com/rust-lang/cargo.git"; // Example repository
    let submodule_path = root_dir.join("submodules").join("cargo");
    let branch = "master";

    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));

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

    println!("Adding submodule: {} at {:?}", repo_url, submodule_path);
    git_executor.add_submodule(repo_url, &submodule_path, Some("cargo"), Some(branch))?;
    println!("Submodule added successfully.");

    Ok(())
}
