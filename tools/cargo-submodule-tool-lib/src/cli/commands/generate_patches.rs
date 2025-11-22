#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, GeneratePatchesArgs};
use crate::{run_submodule_status, RepoSyncConfig};
use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
#[cfg(feature = "git_enabled")]
use crate::executors::RollupLock; // Use our re-exported RollupLock
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyRollupLock as RollupLock; // Use dummy for RollupLock when git is not enabled
#[cfg(feature = "nix_generation")]
use std::path::{Path, PathBuf};
#[cfg(feature = "nix_generation")]
use std::fs;
#[cfg(feature = "nix_generation")]
use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
#[cfg(feature = "nix_generation")]
use cargo2nix::generate_cargo_nix::generate_cargo_nix;
use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};

// Import analysis modules
use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};

#[cfg(feature = "cargo-toml-editor-lib")]
use crate::analysis::workspace_remover::RealWorkspaceRemover;
#[cfg(feature = "git_enabled")]
use crate::executors::RealExecv; // Use our re-exported RealExecv
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
use crate::executors::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use crate::executors::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use crate::executors::SystemGitExecutor; // Added for non-git2 case
#[cfg(not(feature = "git_enabled"))]
use crate::executors::DummyGitExecutor; // Use our dummy GitExecutor
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
#[cfg(not(feature = "nix_generation"))]
use crate::analysis::cargo_metadata_provider::DummyCargoMetadataProvider;


#[cfg(feature = "nix_generation")]
pub fn run_generate_patches_command(args: &GeneratePatchesArgs, cli: &Cli) -> Result<()> {
    let project_root = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    println!("Generating patches for workspace submodules in: {}", project_root.display());

    let config_toml_path = project_root.join(".cargo/config.toml");

    // Initialize common dependencies
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&project_root)?));

    // Initialize GitExecutor
    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git_enabled")]
        {
            Arc::new(PureRustGitExecutor::new(rollup_lock_arc.clone(), project_root.clone()))
        }
        #[cfg(not(feature = "git_enabled"))]
        {
            Arc::new(DummyGitExecutor) // Use the dummy struct directly
        }
    };

    let real_file_system_stat = RealFileSystemStat::new(git_executor.clone(), rollup_lock_arc.clone(), project_root.clone());
    let cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync> = if cli.pure_rust_git {
        Box::new(RealCargoMetadataProvider)
    } else {
        Box::new(DummyCargoMetadataProvider)
    };

    let workspace_info = parse_members_file(git_executor.as_ref(), cargo_metadata_provider.as_ref(), &project_root)?;
    let new_patches = generate_patch_entries(&project_root, &workspace_info);
    
    update_config_toml(&config_toml_path, &new_patches)?;

    println!("Generated patch entries written to: {}", config_toml_path.display());

    Ok(())
}

#[cfg(not(feature = "nix_generation"))]
pub fn run_generate_patches_command(_args: &GeneratePatchesArgs, _cli: &Cli) -> Result<()> {
    anyhow::bail!("`generate-patches` command is not available because the `nix_generation` feature is not enabled.");
}
