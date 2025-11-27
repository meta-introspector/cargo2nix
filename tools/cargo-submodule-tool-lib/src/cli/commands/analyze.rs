#[cfg(feature = "cargo-toml-editor-lib")]
// use crate::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
#[cfg(not(feature = "nix_generation"))]
use crate::analysis::cargo_metadata_provider::DummyCargoMetadataProvider;
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
use crate::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger};
use tool_traits_lib::types::MergedCrateInfo;
use crate::analysis::dep_graph_processor::{RealDepGraphProcessor};
use tool_traits_lib::dep_graph_processor::DepGraphProcessor;
use crate::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
use crate::analysis::non_vendored_module_finder::{
    NonVendoredModuleFinder, RealNonVendoredModuleFinder,
};
#[cfg(feature = "git_enabled")]
// use crate::analysis::submodule_config_patcher::{
//     RealSubmoduleConfigPatcher, SubmoduleConfigPatcher,
// };
#[cfg(feature = "cargo-toml-editor-lib")]
use crate::analysis::workspace_remover::{RealWorkspaceRemover, WorkspaceRemover};
// use crate::cargo_config_generator::{
//     generate_patch_entries, parse_members_file, update_config_toml,
// };
use crate::args::analyze::AnalyzeArgs; use crate::args::Cli;
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::execv::DummyExecv as RealExecv; // Use dummy for RealExecv when git is not enabled
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::dummy_git_executor::DummyGitExecutor; // Use our dummy struct directly
#[cfg(not(feature = "git_enabled"))]
use git_wrapper_lib::dummy_rollup_lock::DummyRollupLock as RollupLock;
use git_wrapper_lib::git_traits::GitExecutor; // Use our re-exported GitExecutor
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::RealExecv; // Use our re-exported RealExecv
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::git_types::RollupLock; // Use our re-exported RollupLock
#[cfg(feature = "git_enabled")]
use git_wrapper_lib::system_git_executor::SystemGitExecutor; // Added for non-git2 case
use crate::fs_cache::{FileSystemStat, RealFileSystemStat};
use crate::fs_writer::{CachedFileSystemWriter, FileSystemWriter, RealFileSystemWriter};
use anyhow::{anyhow, Context, Result};
#[cfg(feature = "nix_generation")]
use cargo_metadata::{MetadataCommand, Package, PackageId};
//#[cfg(feature = "cargo_repo_sync_lib_enabled")]
//use cargo_repo_sync_lib::repo_sync_config::RepoSyncConfig;
//#[cfg(feature = "cargo_repo_sync_lib_enabled")]
//use cargo_repo_sync_lib::run_submodule_status::run_submodule_status;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex}; // Use dummy for RollupLock when git is not enabled

#[cfg(feature = "nix_generation")]
pub fn run_analyze_command(args: &AnalyzeArgs, cli: &Cli) -> Result<()> {
    let project_root = args
        .project_root
        .canonicalize()
        .context("Failed to canonicalize project_root")?;
    let depgraph_dot_file = project_root.join(&args.depgraph_dot_file);
    let tree_file = project_root.join(&args.tree_file);
    let cargo_lock_file = project_root.join(&args.cargo_lock_file);
    let cargo_config_file = project_root.join(&args.cargo_config_file);
    let members_file = project_root.join(&args.members_file);
    let submodules_dir = project_root.join(&args.submodules_dir);

    println!("--- Running Analysis ---");

    // Initialize common dependencies
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let executor = Arc::new(RealExecv {});
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&project_root)?));

    // Initialize GitExecutor
    let git_executor: Arc<dyn GitExecutor + Send + Sync> = {
        #[cfg(feature = "git_enabled")]
        {
            Arc::new(PureRustGitExecutor::new(
                rollup_lock_arc.clone(),
                project_root.clone(),
            ))
        }
        #[cfg(not(feature = "git_enabled"))]
        {
            Arc::new(DummyGitExecutor) // Use the dummy struct directly
        }
    };

    let real_file_system_stat = RealFileSystemStat::new(
        git_executor.clone(),
        rollup_lock_arc.clone(),
        project_root.clone(),
    );

    // Instantiate CargoMetadataProvider
    let metadata_provider: Arc<dyn CargoMetadataProvider> = {
        #[cfg(feature = "nix_generation")]
        {
            Arc::new(RealCargoMetadataProvider)
        }
        #[cfg(not(feature = "nix_generation"))]
        {
            Arc::new(DummyCargoMetadataProvider)
        }
    };

    // 1. Process Dependency Graph
    println!("\n--- Processing Dependency Graph (process_depgraph.py) ---");
    let dep_graph_processor = RealDepGraphProcessor;
    let (graph, nodes) = dep_graph_processor.parse_dot_file(&depgraph_dot_file)?;
    let layer_data = dep_graph_processor.calculate_layers(&graph, &nodes)?;
    for (crate_name, layer) in &layer_data {
        println!("{}: Layer {}", crate_name, layer);
    }

    // 2. Find Non-Vendored Modules
    println!("\n--- Finding Non-Vendored Modules (find_non_vendored.py) ---");
    let non_vendored_finder = RealNonVendoredModuleFinder;
    let usage_counts =
        non_vendored_finder.find_and_count_non_vendored(&tree_file, &project_root)?;
    for (module, count) in &usage_counts {
        println!("{}: Usage Count {}", module, count);
    }

    // 3. Merge Depgraph Data
    println!("\n--- Merging Dependency Graph Data (merge_depgraph_data.py) ---");
    let data_merger = RealDepGraphDataMerger;
    let merged_data = data_merger.merge_data(layer_data.clone(), usage_counts.clone())?;
    println!("Crates ordered by Layer (0-N) with Usage Counts:");
    let mut sorted_merged_data: Vec<(&String, &MergedCrateInfo)> = merged_data.iter().collect();
    sorted_merged_data.sort_by(|a, b| a.1.layer.cmp(&b.1.layer).then_with(|| a.0.cmp(&b.0)));
    for (crate_name, info) in sorted_merged_data {
        println!(
            "  {}: Layer {}, Usage Count: {}",
            crate_name, info.layer, info.usage_count
        );
    }

    // 4. Analyze Layer 0 Usage
    println!("\n--- Analyzing Layer 0 Usage (analyze_layer0_usage.py) ---");
    let layer0_analyzer = RealLayer0Analyzer;
    if let Some((most_used, count)) = layer0_analyzer.find_most_used_layer0_module(&merged_data)? {
        println!("The single most used Layer 0 module (not being overridden) is: {} with Usage Count: {}", most_used, count);
    } else {
        println!("Could not determine the most used Layer 0 module.");
    }

    // 5. Generate Cargo Config Patches
    println!("\n--- Generating Cargo Config Patches (generate_config_patches.py) ---");
    let cargo_config_patcher = RealCargoConfigPatcher::new(metadata_provider.clone());
    let new_cargo_config_patches = cargo_config_patcher.generate_patches(
        &tree_file,
        &cargo_lock_file,
        &cargo_config_file,
        &project_root,
    )?;
    if !new_cargo_config_patches.is_empty() {
        println!("Generated [patch.crates-io] entries:");
        for patch in new_cargo_config_patches {
            println!("{}", patch);
        }
    } else {
        println!("No new [patch.crates-io] entries to generate.");
    }

    // 6. Generate Submodule Config Patches
    println!(
        "\n--- Generating Submodule Config Patches (tools/update_cargo_config_patches.py) ---"
    );
    // let submodule_config_patcher = RealSubmoduleConfigPatcher;
    // let submodule_patches =
        // submodule_config_patcher.generate_submodule_patches(&members_file, &project_root)?;
    // if !submodule_patches.is_empty() {
    //     println!("Generated submodule patch entries:");
    //     for (header, entries) in submodule_patches {
    //         println!("{}", header);
    //         for entry in entries {
    //             println!("{}", entry);
    //         }
    //     }
    // } else {
    //     println!("No submodule patch entries to generate.");
    // }

    // 7. Remove Submodule Workspaces
    println!("\n--- Removing Submodule Workspaces (tools/remove_submodule_workspaces.py) ---");
    let workspace_remover = RealWorkspaceRemover;
    if cli.dry_run {
        println!("[DRY RUN] Would remove workspace sections from Cargo.toml files in submodules.");
    } else {
        workspace_remover.remove_workspace_sections(&submodules_dir)?;
    }

    println!("\n--- Analysis Complete ---");
    Ok(())
}

#[cfg(not(feature = "nix_generation"))]
pub fn run_analyze_command(_args: &AnalyzeArgs, _cli: &Cli) -> Result<()> {
    anyhow::bail!(
        "`analyze` command is not available because the `nix_generation` feature is not enabled."
    );
}
