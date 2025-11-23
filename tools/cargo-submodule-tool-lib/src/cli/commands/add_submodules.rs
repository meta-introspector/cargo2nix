use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::AddSubmodulesArgs;
use crate::cli::args::Cli;
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyGitExecutor; // Use our dummy GitExecutor
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyRollupLock as RollupLock;
use crate::executors::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use crate::executors::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use crate::executors::RealExecv; // Use our re-exported RealExecv
#[cfg(feature = "git_enabled")]
use crate::executors::RollupLock; // Use our re-exported RollupLock
#[cfg(feature = "git_enabled")]
use crate::executors::SystemGitExecutor; // Added for non-git2 case
use crate::fs_cache::RealFileSystemStat;
use crate::fs_writer::CachedFileSystemWriter;
use crate::fs_writer::FileSystemWriter;
use crate::fs_writer::RealFileSystemWriter;
use crate::RepoSyncConfig;
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::git_traits::GhExecutor; // Use dummy for RollupLock when git is not enabled

pub fn run_add_submodules_command(args: &AddSubmodulesArgs, cli: &Cli) -> Result<()> {
    println!("Collecting repository state for: {:?}", args.root_dir); // Changed project_root to args.root_dir

    // Initialize common dependencies
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&args.root_dir)?)); // Changed root_dir to args.root_dir

    // Initialize GitExecutor
    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git_enabled")]
        {
            Arc::new(PureRustGitExecutor::new(
                rollup_lock_arc.clone(),
                args.root_dir.clone(),
            )) // Changed root_dir to args.root_dir
        }
        #[cfg(not(feature = "git_enabled"))]
        {
            Arc::new(DummyGitExecutor) // Use the dummy struct directly
        }
    };

    // Initialize RealFileSystemStat using the created git_executor
    let real_file_system_stat = RealFileSystemStat::new(
        git_executor.clone(),
        rollup_lock_arc.clone(),
        args.root_dir.clone(),
    ); // Changed root_dir to args.root_dir

    let _file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(
            Arc::new(real_file_system_stat.clone()),
            rollup_lock_arc.clone(),
            args.root_dir.clone(),
        )) // Changed root_dir to args.root_dir
    } else {
        Box::new(RealFileSystemWriter)
    };

    let _config = RepoSyncConfig {
        root_dir: args.root_dir.clone(),
        target_org: args.target_org.clone(),
        target_branch: args.target_branch.clone(),
        output_file: Some(args.output_file.clone()),
        json_input_file: args.json_input_file.clone(),
        dry_run: cli.dry_run,
        json_log_file: cli.json_log_file.clone(),
        report: cli.report,
        use_pure_rust_git: cli.pure_rust_git,
    };
    // run_add_submodules(config, file_system_writer.as_ref())
    Ok(())
}
