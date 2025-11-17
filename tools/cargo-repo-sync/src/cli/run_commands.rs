use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;
use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::cli::args::{Cli, AddSubmodulesArgs, SubmoduleStatusArgs, GenerateNixArgs, GeneratePatchesArgs};
use cargo_repo_sync::{run_submodule_status, RepoSyncConfig};
use cargo_repo_sync::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};
use cargo_repo_sync::fs_cache::{RealFileSystemStat, FileSystemStat};
use cargo_repo_sync::RollupLock;
use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
use cargo2nix::generate_cargo_nix::generate_cargo_nix;

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