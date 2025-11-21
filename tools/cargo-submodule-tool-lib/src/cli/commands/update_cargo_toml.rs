use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
#[cfg(feature = "system_git")]
use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;
#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, UpdateCargoTomlArgs};
use crate::{run_submodule_status, RepoSyncConfig};
use crate::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};
use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
//use crate::RollupLock;
#[cfg(feature = "nix_generation")]
use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
#[cfg(feature = "nix_generation")]
use cargo2nix::generate_cargo_nix::generate_cargo_nix;

// Import analysis modules
use crate::analysis::dep_graph_processor::{DepGraphProcessor, RealDepGraphProcessor};
use crate::analysis::non_vendored_module_finder::{NonVendoredModuleFinder, RealNonVendoredModuleFinder};
use crate::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger, MergedCrateInfo};
use crate::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
use crate::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
use crate::analysis::submodule_config_patcher::{SubmoduleConfigPatcher, RealSubmoduleConfigPatcher};
use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};

use crate::analysis::workspace_remover::RealWorkspaceRemover;
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
use cargo_toml_editor_lib::{CargoTomlUpdater, RealCargoTomlUpdater};

#[cfg(feature = "nix_generation")]
pub fn run_update_cargo_toml_command(args: &UpdateCargoTomlArgs, _cli: &Cli) -> Result<()> {
    let project_root = args.project_root.canonicalize().context("Failed to canonicalize project_root")?;
    let cargo_toml_path = project_root.join(&args.cargo_toml_path);

    println!("--- Updating Cargo.toml ---");

    let cargo_toml_updater = RealCargoTomlUpdater;
    let generated_deps_content = cargo_toml_updater.generate_deps_from_submodules(&project_root)?;
    cargo_toml_updater.update_cargo_toml(&cargo_toml_path, &generated_deps_content)?;

    println!("Successfully updated Cargo.toml: {}", cargo_toml_path.display());

    Ok(())
}

#[cfg(not(feature = "nix_generation"))]
pub fn run_update_cargo_toml_command(_args: &UpdateCargoTomlArgs, _cli: &Cli) -> Result<()> {
    anyhow::bail!("`update-cargo-toml` command is not available because the `nix_generation` feature is not enabled.");
}