use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::fs;

#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::RepoSyncConfig;
use crate::fs_writer::RealFileSystemWriter;
use crate::fs_writer::CachedFileSystemWriter;
use crate::fs_writer::FileSystemWriter;
use crate::fs_cache::RealFileSystemStat;
use git_wrapper_lib::git_types::RollupLock;
use crate::cli::args::Cli;
use crate::cli::args::AddSubmodulesArgs;
use git_wrapper_lib::git_traits::GhExecutor;
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
use git_wrapper_lib::system_git_executor::SystemGitExecutor; // Added for non-git2 case
use git_wrapper_lib::dummy_git_executor::DummyGitExecutor; // Added for default dummy git
use git_wrapper_lib::git_traits::GitExecutor;
use git_wrapper_lib::execv::RealExecv;

pub fn run_add_submodules_command(args: &AddSubmodulesArgs, cli: &Cli) -> Result<()> {
    println!("Collecting repository state for: {:?}", project_root);

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


    // Initialize RealFileSystemStat using the created git_executor
    let real_file_system_stat = RealFileSystemStat::new(git_executor.clone(), rollup_lock_arc.clone(), root_dir.clone());

    let _file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()))
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
