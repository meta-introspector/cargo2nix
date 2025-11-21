use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, AddSubmodulesArgs};
use crate::{run_submodule_status, RepoSyncConfig};
use crate::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};
use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
use crate::RollupLock;
use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
use cargo2nix::generate_cargo_nix::generate_cargo_nix;

// Import analysis modules
use crate::analysis::dep_graph_processor::{DepGraphProcessor, RealDepGraphProcessor};
use crate::analysis::non_vendored_module_finder::{NonVendoredModuleFinder, RealNonVendoredModuleFinder};
use crate::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger, MergedCrateInfo};
use crate::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
use crate::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
use crate::analysis::submodule_config_patcher::{SubmoduleConfigPatcher, RealSubmoduleConfigPatcher};
use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
use crate::traits::execv::{Execv, SystemExecv, DryRunExecv, JsonCaptureExecv, ReportExecv};
use crate::executors::{PureRustGitExecutor, SystemGitExecutor, GitExecutor, SystemGhExecutor};
use crate::analysis::workspace_remover::RealWorkspaceRemover;
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
use crate::analysis::cargo_toml_updater::{CargoTomlUpdater, RealCargoTomlUpdater};

pub fn run_add_submodules_command(args: &AddSubmodulesArgs, cli: &Cli) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?)); // Open the repository and wrap in Mutex
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let real_file_system_stat = RealFileSystemStat::new(repo.clone()); // Pass the repository

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
