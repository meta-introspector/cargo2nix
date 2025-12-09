use anyhow::{Context, Result};
#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};
#[cfg(feature = "system_git")]
use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

//use super::args::update_cargo_toml::UpdateCargoTomlArgs;
//use super::args::Cli;
use crate::fs_cache::{FileSystemStat, RealFileSystemStat};
use crate::fs_writer::{CachedFileSystemWriter, FileSystemWriter, RealFileSystemWriter};
use crate::repo_sync_lib::repo_sync_config::RepoSyncConfig;
use crate::repo_sync_lib::run_submodule_status::run_submodule_status;
//use crate::RollupLock;
#[cfg(feature = "nix_generation")]
use cargo2nix::discovery::{find_cargo_locks, find_cargo_manifests};
#[cfg(feature = "nix_generation")]
use cargo2nix::generate_cargo_nix::generate_cargo_nix;

// Import analysis modules
// use crate::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
use crate::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger};
use crate::analysis::dep_graph_processor::RealDepGraphProcessor;
use crate::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
use crate::analysis::non_vendored_module_finder::{
    NonVendoredModuleFinder, RealNonVendoredModuleFinder,
};
use tool_traits_lib::dep_graph_processor::DepGraphProcessor;
use tool_traits_lib::types::MergedCrateInfo;
// use crate::analysis::submodule_config_patcher::RealSubmoduleConfigPatcher;
// use crate::analysis::submodule_config_patcher::SubmoduleConfigPatcher;
// use crate::cargo_config_generator::{
//     generate_patch_entries, parse_members_file, update_config_toml,
// };

use crate::analysis::cargo_metadata_provider::{
    CargoMetadataProvider, RealCargoMetadataProvider,
};
use crate::analysis::workspace_remover::{RealWorkspaceRemover, WorkspaceRemover};
use git_wrapper_lib::execv::RealExecv;
use git_wrapper_lib::git_traits::GitExecutor;
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
use git_wrapper_lib::system_git_executor::SystemGitExecutor;
// #[cfg(feature = "toml_edit_enabled")]
// use cargo_toml_editor_lib::{CargoTomlUpdater, RealCargoTomlUpdater}; // This is the line causing the error // Added for non-git2 case

// #[cfg(feature = "toml_edit_enabled")]
// pub fn run_update_cargo_toml_command(args: UpdateCargoTomlArgs) -> Result<()> {
//     let project_root = args
//         .project_root
//         .canonicalize()
//         .context("Failed to canonicalize project_root")?;
//     let cargo_toml_path = project_root.join(&args.cargo_toml_path);

//     println!("--- Updating Cargo.toml ---");

//     let cargo_toml_updater = RealCargoTomlUpdater;
//     let generated_deps_content = cargo_toml_updater.generate_deps_from_submodules(&project_root)?;
//     cargo_toml_updater.update_cargo_toml(&cargo_toml_path, &generated_deps_content)?;

//     println!(
//         "Successfully updated Cargo.toml: {}",
//         cargo_toml_path.display()
//     );

//     Ok(())
// }

// #[cfg(not(feature = "nix_generation"))]
// pub fn run_update_cargo_toml_command(_args: &UpdateCargoTomlArgs, _cli: &Cli) -> Result<()> {
//     anyhow::bail!("`update-cargo-toml` command is not available because the `nix_generation` feature is not enabled.");
// }
