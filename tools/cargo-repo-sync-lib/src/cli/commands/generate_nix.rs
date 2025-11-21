use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, GenerateNixArgs};
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


pub fn run_generate_nix_command(args: &GenerateNixArgs, cli: &Cli) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?));
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let real_file_system_stat = RealFileSystemStat::new(repo.clone());

    let file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()))
    } else {
        Box::new(RealFileSystemWriter)
    };

    // --- Read executable paths from Cargo.toml metadata ---
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml_path = manifest_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read Cargo.toml at {:?}", cargo_toml_path))?;
    let cargo_toml_doc = cargo_toml_content
        .parse::<toml_edit::DocumentMut>()
        .with_context(|| format!("Failed to parse Cargo.toml at {:?}", cargo_toml_path))?;

    let git_executable_path = cargo_toml_doc
        .get("package")
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("metadata"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("repo-manager"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("git_path"))
        .and_then(|item| item.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("'git_path' not found in Cargo.toml metadata"))?;

    let gh_executable_path = cargo_toml_doc
        .get("package")
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("metadata"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("repo-manager"))
        .and_then(|item| item.as_table())
        .and_then(|table| table.get("gh_path"))
        .and_then(|item| item.as_str())
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("'gh_path' not found in Cargo.toml metadata"))?;
    // --- End Read executable paths from Cargo.toml metadata ---

    // --- Executor Setup with Decorators ---
    let mut base_executor: Arc<dyn Execv> = Arc::new(SystemExecv);
    let json_capture_executor: Option<Arc<JsonCaptureExecv>> = if cli.json_log_file.is_some() {
        let json_exec = Arc::new(JsonCaptureExecv::new(base_executor.clone()));
        base_executor = json_exec.clone();
        Some(json_exec)
    } else {
        None
    };

    if cli.report {
        base_executor = Arc::new(ReportExecv::new(base_executor.clone()));
    }

    if cli.dry_run {
        base_executor = Arc::new(DryRunExecv::new(base_executor.clone()));
        println!("--- DRY RUN MODE ACTIVE ---");
    }

    let git_executor: Box<dyn GitExecutor>;
    if cli.pure_rust_git {
        git_executor = Box::new(PureRustGitExecutor::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()));
    } else {
        git_executor = Box::new(SystemGitExecutor::new(git_executable_path.clone(), base_executor.clone(), rollup_lock_arc.clone(), root_dir.clone()));
    }
    // The gh_executor is not directly used in generate_nix, but it's part of the common setup.
    let _gh_executor = SystemGhExecutor::new(gh_executable_path.clone(), base_executor.clone());
    // --- End Executor Setup ---

    println!("Discovering Cargo.toml and Cargo.lock files in: {}", args.root_dir.display());

    let manifests = find_cargo_manifests(&args.root_dir)?;
    let locks = find_cargo_locks(&args.root_dir)?;

    println!("Found {} Cargo.toml files and {} Cargo.lock files.", manifests.len(), locks.len());

    for manifest_path in manifests {
        println!("  Cargo.toml: {}", manifest_path.display());
        let parent_dir = manifest_path.parent().unwrap().to_path_buf();
        let cargo_lock_path = parent_dir.join("Cargo.lock");
        let output_nix_path = parent_dir.join("Cargo.nix");

        let current_manifest_metadata = real_file_system_stat.get_metadata(&manifest_path)?;
        let current_lock_metadata = if cargo_lock_path.exists() {
            Some(real_file_system_stat.get_metadata(&cargo_lock_path)?)
        } else {
            None
        };

        let rollup_lock_guard = rollup_lock_arc.lock().unwrap();
        let stored_manifest_metadata = rollup_lock_guard.get_metadata(&manifest_path);
        let stored_lock_metadata = rollup_lock_guard.get_metadata(&cargo_lock_path);
        // drop(rollup_lock_guard); // Release the lock early - removed as per previous instruction

        let should_generate = match (stored_manifest_metadata, stored_lock_metadata) {
            (Some(sm), Some(sl)) => {
                // Both manifest and lock metadata stored
                current_manifest_metadata != *sm || current_lock_metadata.clone().map_or(true, |cl| cl != *sl)
            },
            (Some(sm), None) => {
                // Only manifest metadata stored, no lock stored
                current_manifest_metadata != *sm || current_lock_metadata.is_some()
            },
            (None, Some(_sl)) => { // _sl is unused
                // Only lock metadata stored, no manifest stored (unlikely but handle)
                true // Always regenerate if manifest metadata is missing
            },
            (None, None) => {
                // No metadata stored
                true
            },
        };

        if should_generate {
            println!("    Generating Cargo.nix for {}", manifest_path.display());
            let rendered_nix = generate_cargo_nix(&parent_dir, false)?; // Assuming not locked for now
            file_system_writer.write_file(&output_nix_path, rendered_nix.as_bytes())?;

            // Update metadata in rollup_lock
            let mut rollup_lock_guard = rollup_lock_arc.lock().unwrap();
            rollup_lock_guard.set_metadata(manifest_path.clone(), current_manifest_metadata);
            if let Some(clm) = current_lock_metadata {
                rollup_lock_guard.set_metadata(cargo_lock_path.clone(), clm);
            }
            println!("    Generated: {}", output_nix_path.display());
        } else {
            println!("    Skipping generation for {} (metadata unchanged).", manifest_path.display());
        }
    }

    // for lock_path in locks {
    //     println!("  Cargo.lock: {}", lock_path.display());
    // }

    file_system_writer.save_lock()?;
    Ok(())
}
