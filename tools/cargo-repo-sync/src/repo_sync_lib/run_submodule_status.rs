use anyhow::{Context, Result};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    time::{SystemTime, Duration}, // Add SystemTime and Duration
};
use toml_edit::DocumentMut;
use walkdir::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;
use git2::Repository; // Add git2 import

use crate::executors::{GitExecutor, GhExecutor, SystemGitExecutor, SystemGhExecutor, PureRustGitExecutor};
use crate::traits::execv::{Execv, SystemExecv, DryRunExecv, JsonCaptureExecv, ReportExecv};
use crate::fs_cache::{FileSystemStat, FileMetadata, RealFileSystemStat, CachedFileSystemStat};
use crate::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};

use super::repo_action::RepoAction;
use super::rollup_lock::RollupLock;
use super::submodule_stat::SubmoduleStat;
use super::submodule_stat_provider::{SubmoduleStatProvider};
use super::real_submodule_stat_provider::RealSubmoduleStatProvider;
use super::cached_submodule_stat_provider::CachedSubmoduleStatProvider;
use super::repo_sync_config::RepoSyncConfig;
use super::git_snapshot::create_snapshot; // Import create_snapshot

const STALENESS_THRESHOLD: Duration = Duration::from_secs(365 * 24 * 60 * 60); // 1 year

pub fn run_submodule_status(config: RepoSyncConfig) -> Result<()> {
    let root_dir = config.root_dir.canonicalize().context("Failed to canonicalize root_dir")?;
    let repo = Arc::new(Mutex::new(Repository::open(&root_dir).context("Failed to open git repository")?)); // Open the repository and wrap in Mutex

    // Load RollupLock
    let rollup_lock_data = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));

    // Check for staleness before creating a snapshot
    let should_create_snapshot = {
        let rollup_lock_guard = rollup_lock_data.lock().unwrap();
        if let Some(last_snapshot_time) = rollup_lock_guard.last_snapshot_time {
            SystemTime::now().duration_since(last_snapshot_time).map_or(true, |d| d > STALENESS_THRESHOLD)
        } else {
            true // No previous snapshot, so create one
        }
    };

    if should_create_snapshot {
        println!("Creating new snapshot...");
        create_snapshot(&root_dir, rollup_lock_data.clone())?;
    } else {
        println!("Snapshot is recent enough, skipping snapshot creation.");
    }

    let file_system_stat: Arc<dyn FileSystemStat> = Arc::new(CachedFileSystemStat::new(
        Arc::new(RealFileSystemStat::new(repo.clone())), // Pass the repository
        rollup_lock_data.clone(),
        config.root_dir.clone(),
    ));

    let file_system_writer: Box<dyn FileSystemWriter> = if config.dry_run {
        Box::new(CachedFileSystemWriter::new(file_system_stat.clone(), rollup_lock_data.clone(), root_dir.clone()))
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

    // --- Diagnostic: Check for git and gh executables ---
    println!("Checking for 'git' executable...");
    let git_version_output = Command::new(&git_executable_path).arg("--version").output();
    match git_version_output {
        Ok(output) => {
            if output.status.success() {
                println!("'git' found: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                eprintln!("'git' command failed: {}", String::from_utf8_lossy(&output.stderr).trim());
                anyhow::bail!("'git' executable not working correctly. Please ensure Git is installed and in your PATH.");
            }
        },
        Err(e) => anyhow::bail!("Failed to execute '{:?} --version': {}. Please ensure Git is installed and in your PATH.", git_executable_path, e),
    }

    println!("Checking for 'gh' executable...");
    let gh_version_output = Command::new(&gh_executable_path).arg("--version").output();
    match gh_version_output {
        Ok(output) => {
            if output.status.success() {
                println!("'gh' found: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                eprintln!("'gh' command failed: {}", String::from_utf8_lossy(&output.stderr).trim());
                anyhow::bail!("'gh' executable not working correctly. Please ensure GitHub CLI is installed and in your PATH.");
            }
        },
        Err(e) => eprintln!("Warning: Failed to execute '{:?} --version': {}. GitHub CLI might not be installed or not working correctly. Forking operations might fail.", gh_executable_path, e),
    }
    // --- End Diagnostic ---

    // --- Executor Setup with Decorators ---
    let mut base_executor: Arc<dyn Execv> = Arc::new(SystemExecv);
    let json_capture_executor: Option<Arc<JsonCaptureExecv>> = if config.json_log_file.is_some() {
        let json_exec = Arc::new(JsonCaptureExecv::new(base_executor.clone()));
        base_executor = json_exec.clone();
        Some(json_exec)
    } else {
        None
    };

    if config.report {
        base_executor = Arc::new(ReportExecv::new(base_executor.clone()));
    }

    if config.dry_run {
        base_executor = Arc::new(DryRunExecv::new(base_executor.clone()));
        println!("--- DRY RUN MODE ACTIVE ---");
    }

    let git_executor: Box<dyn GitExecutor>;
    if config.use_pure_rust_git {
        git_executor = Box::new(PureRustGitExecutor::new(file_system_stat.clone(), rollup_lock_data.clone(), root_dir.clone()));
    } else {
        git_executor = Box::new(SystemGitExecutor::new(git_executable_path.clone(), base_executor.clone(), rollup_lock_data.clone(), root_dir.clone()));
    }
    let gh_executor = SystemGhExecutor::new(gh_executable_path.clone(), base_executor.clone());
    // --- End Executor Setup ---

    let submodules_dir = root_dir.join("submodules");

    // Create the submodules directory if it doesn't exist
    file_system_writer.create_dir_all(&submodules_dir)?;

    let actions_plan: Vec<RepoAction>;

    if let Some(json_path) = &config.json_input_file {
        println!("Reading actions plan from JSON file: {:?}", json_path);
        let json_content = fs::read_to_string(json_path)
            .with_context(|| format!("Failed to read JSON input file at {:?}", json_path))?;
        actions_plan = serde_json::from_str(&json_content)
            .with_context(|| format!("Failed to parse JSON from {:?}", json_path))?;
    } else {
        // For submodule-status, if no json_input_file, we need to discover existing submodules
        // This is a simplified discovery for status, assuming submodules are already added
        let mut discovered_actions_plan: Vec<RepoAction> = Vec::new();
        for entry in WalkDir::new(&submodules_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| e.path().join(".git").exists() || e.path().join(".git").is_file()) // Check if it's a git repo
        {
            let submodule_path = entry.path().to_path_buf();
            let repo_name = submodule_path.file_name().unwrap().to_string_lossy().to_string();
            // Dummy values for other fields, as they are not strictly needed for status
            discovered_actions_plan.push(RepoAction {
                repo_url: String::new(),
                owner: String::new(),
                repo_name,
                submodule_path,
                target_org: String::new(),
                target_branch: String::new(),
            });
        }
        actions_plan = discovered_actions_plan;
    }

    // Instantiate RealSubmoduleStatProvider and CachedSubmoduleStatProvider
    let real_submodule_stat_provider = RealSubmoduleStatProvider {
        git_executable_path: git_executable_path.clone(),
        base_executor: base_executor.clone(),
    };
    let cached_submodule_stat_provider = CachedSubmoduleStatProvider {
        real_provider: real_submodule_stat_provider,
        rollup_lock: rollup_lock_data.clone(),
    };

    println!("Checking status of submodules:");
    for action in &actions_plan {
        if action.submodule_path.exists() {
            let current_stat = cached_submodule_stat_provider.get_submodule_stat(&action.submodule_path)?;
            let mut rollup_lock_guard = rollup_lock_data.lock().unwrap();
            let cached_stat_option = rollup_lock_guard.submodule_stat_cache.get(&action.submodule_path);

            if let Some(cached_stat) = cached_stat_option {
                if cached_stat == &current_stat {
                    println!("--- Submodule {:?} (cached) ---", action.submodule_path);
                    println!("Status: Unchanged (HEAD: {}, Workdir: {})", current_stat.head_commit, current_stat.workdir_hash);
                } else {
                    println!("--- Submodule {:?} (changed) ---", action.submodule_path);
                    println!("Status: Changed (Cached HEAD: {}, Current HEAD: {})", cached_stat.head_commit, current_stat.head_commit);
                    println!("Status: Changed (Cached Workdir: {}, Current Workdir: {})", cached_stat.workdir_hash, current_stat.workdir_hash);
                    let status_output = git_executor.status(&action.submodule_path)?;
                    println!("{}", status_output);
                    // Update cache with new status
                    rollup_lock_guard.submodule_stat_cache.insert(action.submodule_path.clone(), current_stat);
                }
            } else {
                println!("--- Submodule {:?} (new/uncached) ---", action.submodule_path);
                let status_output = git_executor.status(&action.submodule_path)?;
                println!("{}", status_output);
                // Add to cache
                rollup_lock_guard.submodule_stat_cache.insert(action.submodule_path.clone(), current_stat);
            }
            drop(rollup_lock_guard); // Release the lock early
        } else {
            println!("Submodule path {:?} does not exist.", action.submodule_path);
        }
    }

    file_system_writer.save_lock()?;

    // Write JSON log if enabled
    if let Some(json_log_file) = config.json_log_file {
        if let Some(json_exec) = json_capture_executor {
            let captured_commands = json_exec.commands.lock().unwrap();
            let json_output = serde_json::to_string_pretty(&*captured_commands)
                .context("Failed to serialize captured commands to JSON")?;
            fs::write(&json_log_file, json_output)
                .with_context(|| format!("Failed to write JSON log to {:?}", json_log_file))?;
            println!("Captured commands written to JSON log file: {:?}", json_log_file);
        }
    }

    Ok(())
}
