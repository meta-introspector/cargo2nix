use anyhow::{Context, Result};
use crate::executors::{GitExecutor, PureRustGitExecutor};
use crate::fs_cache::{FileSystemStat, FileMetadata};
use crate::repo_discovery::{PureRustRepoDiscoverer, RepoDiscoverer};
use crate::RollupLock;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::cargo_toml_patcher::patch_cargo_toml;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target GitHub organization for forked repositories
    #[clap(long, default_value = "meta-introspector")]
    target_org: String,

    /// Target branch for forked repositories
    #[clap(long, default_value = "feature/CRQ-016-nixify")]
    target_branch: String,
}

// Dummy implementation for FileSystemStat
struct DummyFileSystemStat;

impl FileSystemStat for DummyFileSystemStat {
    fn get_metadata(&self, _path: &Path) -> Result<FileMetadata> {
        // For now, we'll return a dummy entry.
        // In a real scenario, this would interact with the actual file system or a cache.
        Ok(FileMetadata {
            modified: SystemTime::now(),
            len: 0,
            hash: "dummy_hash".to_string(),
            git_object_hash: None,
            is_git_tracked: false,
        })
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let root_dir = std::env::current_dir().context("Failed to get current working directory")?;
    let submodules_dir = root_dir.join("submodules");

    // Ensure the submodules directory exists
    if !submodules_dir.exists() {
        std::fs::create_dir_all(&submodules_dir)
            .with_context(|| format!("Failed to create submodules directory at {:?}", submodules_dir))?;
    }

    let discoverer = PureRustRepoDiscoverer::new(args.target_org, args.target_branch);
    let (repos, all_vendored_crate_names) = discoverer.discover_repos(&root_dir)?;
    println!("All vendored crate names: {:?}", all_vendored_crate_names);

    let rollup_lock = Arc::new(Mutex::new(RollupLock::load(&root_dir)?));
    let fs_stat = Arc::new(DummyFileSystemStat);
    let git_executor = PureRustGitExecutor::new(fs_stat, rollup_lock.clone(), root_dir.clone());

    println!("Discovered {} repositories. Adding as submodules...", repos.len());

    for repo_info in repos {
        let submodule_path = submodules_dir.join(&repo_info.repo_name);
        println!(
            "Attempting to add submodule: {} at {:?}",
            repo_info.repo_url, submodule_path
        );
        git_executor
            .submodule_add(
                &repo_info.repo_url,
                &submodule_path,
                rollup_lock.clone(),
                &root_dir,
            )
            .with_context(|| {
                format!(
                    "Failed to add submodule {} at {:?}",
                    repo_info.repo_url, submodule_path
                )
            })?;
        println!(
            "Successfully added submodule: {} at {:?}",
            repo_info.repo_url, submodule_path
        );

        // Patch the Cargo.toml of the newly added submodule
        let submodule_cargo_toml_path = submodule_path.join("Cargo.toml");
        if submodule_cargo_toml_path.exists() {
            patch_cargo_toml(&submodule_cargo_toml_path, &repo_info.repo_name, &all_vendored_crate_names)?;
            println!("Successfully patched Cargo.toml for submodule: {:?}", submodule_path);
        } else {
            eprintln!("WARNING: Cargo.toml not found for submodule at {:?}", submodule_cargo_toml_path);
        }
    }

    println!("All discovered repositories processed.");

    Ok(())
}