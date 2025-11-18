use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, AddSubmodulesArgs, SubmoduleStatusArgs, GenerateNixArgs, GeneratePatchesArgs, AnalyzeArgs};
use cargo_repo_sync::{run_submodule_status, RepoSyncConfig};
use cargo_repo_sync::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};
use cargo_repo_sync::fs_cache::{RealFileSystemStat, FileSystemStat};
use cargo_repo_sync::RollupLock;
use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
use cargo2nix::generate_cargo_nix::generate_cargo_nix;

// Import analysis modules
use cargo_repo_sync::analysis::dep_graph_processor::{DepGraphProcessor, RealDepGraphProcessor};
use cargo_repo_sync::analysis::non_vendored_module_finder::{NonVendoredModuleFinder, RealNonVendoredModuleFinder};
use cargo_repo_sync::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger, MergedCrateInfo};
use cargo_repo_sync::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
use cargo_repo_sync::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
use cargo_repo_sync::analysis::submodule_config_patcher::{SubmoduleConfigPatcher, RealSubmoduleConfigPatcher};
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
    if cli.use_pure_rust_git {
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

pub fn run_generate_patches_command(args: &GeneratePatchesArgs, cli: &Cli) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    println!("Generating patches for workspace submodules in: {}", root_dir.display());

    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?));
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let real_file_system_stat = RealFileSystemStat::new(repo.clone());

    let file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()))
    } else {
        Box::new(RealFileSystemWriter)
    };

    let manifests = find_cargo_manifests(&root_dir)?;
    let mut patch_entries: Vec<String> = Vec::new();
    let submodules_dir = root_dir.join("submodules");

    for manifest_path in manifests {
        let metadata = MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()
            .with_context(|| format!("Failed to get cargo metadata for {:?}", manifest_path))?;

        // Check if this manifest is a workspace root
        if let Some(workspace_root) = metadata.workspace_root.to_str() {
            if Path::new(workspace_root) == manifest_path.parent().unwrap() {
                println!("Found workspace root: {}", manifest_path.display());

                for member_id in &metadata.workspace_members {
                    let member_package = metadata.packages.iter().find(|p| &p.id == member_id)
                        .context("Workspace member package not found in metadata")?;

                    let member_manifest_path = PathBuf::from(&member_package.manifest_path);
                    let member_path = member_manifest_path.parent().unwrap();

                    let relative_member_path = member_path.strip_prefix(&root_dir)
                        .context("Failed to get relative path for workspace member")?;

                    let expected_submodule_path = submodules_dir.join(relative_member_path);

                    if expected_submodule_path.exists() && expected_submodule_path.is_dir() {
                        // Check if it's a git repository
                        let git_dir = expected_submodule_path.join(".git");
                        if git_dir.exists() {
                            println!("  Found workspace submodule: {} at {:?}", member_package.name, expected_submodule_path);
                            let submodule_name = relative_member_path.to_string_lossy().replace("/", "-"); // Use a sanitized name for the patch entry

                            let patch_entry = format!(
                                "[patch.\"https://github.com/meta-introspector/{}\"]\n{} = {{ path = \"./submodules/{}\" }}",
                                submodule_name, member_package.name, relative_member_path.display()
                            );
                            patch_entries.push(patch_entry);
                        }
                    }
                }
            }
        }
    }

    if !patch_entries.is_empty() {
        let cargo_config_dir = root_dir.join(".cargo");
        file_system_writer.create_dir_all(&cargo_config_dir)?;
        let cargo_config_path = cargo_config_dir.join("config.toml");

        let mut existing_content = String::new();
        if cargo_config_path.exists() {
            existing_content = fs::read_to_string(&cargo_config_path)
                .with_context(|| format!("Failed to read existing .cargo/config.toml at {:?}", cargo_config_path))?;
        }

        let new_content = format!("{}\n{}", existing_content, patch_entries.join("\n\n"));
        file_system_writer.write_file(&cargo_config_path, new_content.as_bytes())?;
        println!("Generated patch entries written to: {}", cargo_config_path.display());
    } else {
        println!("No workspace submodules found to generate patches for.");
    }

    file_system_writer.save_lock()?; // Save any changes made by the file_system_writer

    Ok(())
}

pub fn run_analyze_command(args: &AnalyzeArgs, cli: &Cli) -> Result<()> {
    let project_root = args.project_root.canonicalize().context("Failed to canonicalize project_root")?;
    let depgraph_dot_file = project_root.join(&args.depgraph_dot_file);
    let tree_file = project_root.join(&args.tree_file);
    let cargo_lock_file = project_root.join(&args.cargo_lock_file);
    let cargo_config_file = project_root.join(&args.cargo_config_file);
    let members_file = project_root.join(&args.members_file);
    let submodules_dir = project_root.join(&args.submodules_dir);

    println!("--- Running Analysis ---");

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
    let usage_counts = non_vendored_finder.find_and_count_non_vendored(&tree_file, &project_root)?;
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
        println!("  {}: Layer {}, Usage Count: {}", crate_name, info.layer, info.usage_count);
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
    let cargo_config_patcher = RealCargoConfigPatcher;
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
    println!("\n--- Generating Submodule Config Patches (tools/update_cargo_config_patches.py) ---");
    let submodule_config_patcher = RealSubmoduleConfigPatcher;
    let submodule_patches = submodule_config_patcher.generate_submodule_patches(
        &members_file,
        &project_root,
    )?;
    if !submodule_patches.is_empty() {
        println!("Generated submodule patch entries:");
        for (header, entries) in submodule_patches {
            println!("{}", header);
            for entry in entries {
                println!("  {}", entry);
            }
        }
    } else {
        println!("No submodule patch entries to generate.");
    }

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
        output_file: args.output_file.clone(),
        json_input_file: args.json_input_file.clone(),
        dry_run: cli.dry_run,
        json_log_file: cli.json_log_file.clone(),
        report: cli.report,
        use_pure_rust_git: cli.pure_rust_git,
    };
    // run_add_submodules(config, file_system_writer.as_ref())
    Ok(())
}

pub fn run_submodule_status_command(args: &SubmoduleStatusArgs, cli: &Cli) -> Result<()> {
    let config = RepoSyncConfig {
        root_dir: args.root_dir.clone(),
        target_org: String::new(), // Not used for status, provide dummy
        target_branch: String::new(), // Not used for status, provide dummy
        output_file: None, // Not used for status
        json_input_file: args.json_input_file.clone(),
        dry_run: cli.dry_run,
        json_log_file: cli.json_log_file.clone(),
        report: cli.report,
        use_pure_rust_git: cli.pure_rust_git,
    };
    run_submodule_status(config)
}

pub fn run_generate_nix_command(args: &GenerateNixArgs, cli: &Cli) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?)); // Open the repository and wrap in Mutex
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let real_file_system_stat = RealFileSystemStat::new(repo.clone()); // Pass the repository

    let _file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()))
    } else {
        Box::new(RealFileSystemWriter)
    };

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
            _file_system_writer.write_file(&output_nix_path, rendered_nix.as_bytes())?;

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

    _file_system_writer.save_lock()?;
    Ok(())
}

pub fn run_generate_patches_command(args: &GeneratePatchesArgs, cli: &Cli) -> Result<()> {
    let root_dir = args.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    println!("Generating patches for workspace submodules in: {}", root_dir.display());

    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?));
    let rollup_lock_arc = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let real_file_system_stat = RealFileSystemStat::new(repo.clone());

    let file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
        Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), root_dir.clone()))
    } else {
        Box::new(RealFileSystemWriter)
    };

    let manifests = find_cargo_manifests(&root_dir)?;
    let mut patch_entries: Vec<String> = Vec::new();
    let submodules_dir = root_dir.join("submodules");

    for manifest_path in manifests {
        let metadata = MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()
            .with_context(|| format!("Failed to get cargo metadata for {:?}", manifest_path))?;

        // Check if this manifest is a workspace root
        if let Some(workspace_root) = metadata.workspace_root.to_str() {
            if Path::new(workspace_root) == manifest_path.parent().unwrap() {
                println!("Found workspace root: {}", manifest_path.display());

                for member_id in &metadata.workspace_members {
                    let member_package = metadata.packages.iter().find(|p| &p.id == member_id)
                        .context("Workspace member package not found in metadata")?;

                    let member_manifest_path = PathBuf::from(&member_package.manifest_path);
                    let member_path = member_manifest_path.parent().unwrap();

                    let relative_member_path = member_path.strip_prefix(&root_dir)
                        .context("Failed to get relative path for workspace member")?;

                    let expected_submodule_path = submodules_dir.join(relative_member_path);

                    if expected_submodule_path.exists() && expected_submodule_path.is_dir() {
                        // Check if it's a git repository
                        let git_dir = expected_submodule_path.join(".git");
                        if git_dir.exists() {
                            println!("  Found workspace submodule: {} at {:?}", member_package.name, expected_submodule_path);
                            let submodule_name = relative_member_path.to_string_lossy().replace("/", "-"); // Use a sanitized name for the patch entry

                            let patch_entry = format!(
                                "[patch.\"https://github.com/meta-introspector/{}\"]\n{} = {{ path = \"./submodules/{}\" }}",
                                submodule_name, member_package.name, relative_member_path.display()
                            );
                            patch_entries.push(patch_entry);
                        }
                    }
                }
            }
        }
    }

    if !patch_entries.is_empty() {
        let cargo_config_dir = root_dir.join(".cargo");
        file_system_writer.create_dir_all(&cargo_config_dir)?;
        let cargo_config_path = cargo_config_dir.join("config.toml");

        let mut existing_content = String::new();
        if cargo_config_path.exists() {
            existing_content = fs::read_to_string(&cargo_config_path)
                .with_context(|| format!("Failed to read existing .cargo/config.toml at {:?}", cargo_config_path))?;
        }

        let new_content = format!("{}\n{}", existing_content, patch_entries.join("\n\n"));
        file_system_writer.write_file(&cargo_config_path, new_content.as_bytes())?;
        println!("Generated patch entries written to: {}", cargo_config_path.display());
    } else {
        println!("No workspace submodules found to generate patches for.");
    }

    file_system_writer.save_lock()?; // Save any changes made by the file_system_writer

    Ok(())
}