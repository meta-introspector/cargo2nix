// tools/cargo-submodule-tool-lib/src/bin/add_submodules.rs
use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use git_wrapper_lib::execv::DummyExecv as RealExecv;
use git_wrapper_lib::dummy_git_executor::DummyGitExecutor;
use git_wrapper_lib::dummy_rollup_lock::DummyRollupLock as RollupLock;
use git_wrapper_lib::git_traits::GitExecutor;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct AddSubmodulesArgs {
    #[clap(long, default_value = ".")]
    pub project_root: PathBuf,
}

fn run_add_submodules_command(args: &AddSubmodulesArgs) -> Result<()> {
    println!("Running add_submodules command for project_root: {:?}", args.project_root);
    let _real_execv = RealExecv {};
    let _dummy_git_executor = DummyGitExecutor {};
    let _rollup_lock = RollupLock::load(&args.project_root)?;
    let _git_executor: Arc<dyn GitExecutor + Send + Sync> = Arc::new(_dummy_git_executor);
    Ok(())
}

fn main() -> Result<()> {
    let args = AddSubmodulesArgs::parse();
    run_add_submodules_command(&args)
}
