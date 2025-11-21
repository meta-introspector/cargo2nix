

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use git2::{Repository, StatusOptions};
use sha2::{Sha256, Digest};
use hex;

use crate::RollupLock;
use crate::fs_cache::{FileMetadata, RealFileSystemStat, FileSystemStat};

// Helper function to recursively traverse the Git tree and calculate composite hashes
fn traverse_git_tree(
    repo: &Repository,
    tree: &git2::Tree,
    current_path: &Path,
    rollup_lock_guard: &mut RollupLock,
    real_file_system_stat: &RealFileSystemStat,
    repo_root_path: &Path,
) -> Result<String> {
    let mut current_tree_hasher = Sha256::new();
    let mut children_hashes = Vec::new();

    for entry in tree.iter() {
        let entry_name = entry.name().context("Tree entry name is not valid UTF-8")?;
        let entry_path = current_path.join(entry_name);
        let full_path = repo_root_path.join(&entry_path);

        match entry.kind() {
            Some(git2::ObjectType::Blob) => {
                let blob_oid = entry.id();
                let blob_hash = blob_oid.to_string();

                // Update metadata cache
                if let Ok(metadata) = real_file_system_stat.get_metadata(&full_path) {
                    rollup_lock_guard.set_metadata(entry_path.clone(), FileMetadata {
                        git_object_hash: Some(blob_hash.clone()),
                        is_git_tracked: true,
                        ..metadata
                    });
                }

                // Store hashes for Cargo.toml and .rs files
                if entry_name == "Cargo.toml" {
                    rollup_lock_guard.cargo_toml_hashes.insert(entry_path.clone(), blob_hash.clone());
                } else if entry_path.extension().map_or(false, |ext| ext == "rs") {
                    rollup_lock_guard.rust_file_hashes.insert(entry_path.clone(), blob_hash.clone());
                }
                children_hashes.push(format!("{}:{}", entry_name, blob_hash));
            },
            Some(git2::ObjectType::Tree) => {
                let subtree = entry.to_object(repo)?.peel_to_tree()?;
                let subtree_hash = traverse_git_tree(
                    repo,
                    &subtree,
                    &entry_path,
                    rollup_lock_guard,
                    real_file_system_stat,
                    repo_root_path,
                )?;
                rollup_lock_guard.git_tree_cache.insert(entry_path.clone(), subtree_hash.clone());
                children_hashes.push(format!("{}:{}", entry_name, subtree_hash));
            },
            Some(git2::ObjectType::Commit) => { // Submodule gitlink
                let submodule_name = entry_name;
                let submodule_path = repo_root_path.join(&entry_path);

                let sub_repo = Repository::open(&submodule_path)
                    .with_context(|| format!("Failed to open submodule repository at {}", submodule_path.display()))?;
                let sub_head_commit = sub_repo.head()?.peel_to_commit()?;
                let sub_tree = sub_head_commit.tree()?;

                let sub_tree_hash = traverse_git_tree(
                    &sub_repo,
                    &sub_tree,
                    &PathBuf::from(""), // Submodule's root is its own path
                    rollup_lock_guard,
                    real_file_system_stat,
                    &submodule_path, // Pass submodule_path as repo_root_path for recursive calls
                )?;

                // Calculate submodule's workdir hash
                let mut submodule_workdir_hasher = Sha256::new();
                let mut status_options = StatusOptions::new();
                status_options.include_untracked(true)
                              .recurse_untracked_dirs(true);
                let statuses = sub_repo.statuses(Some(&mut status_options))?;
                for status_entry in statuses.iter() {
                    if let Some(path_str) = status_entry.path() {
                        submodule_workdir_hasher.update(path_str);
                        submodule_workdir_hasher.update(format!("{:?}", status_entry.status()));
                    }
                }
                let workdir_hash = hex::encode(submodule_workdir_hasher.finalize());

                let submodule_composite_hash = format!("{}:{}:{}", sub_head_commit.id(), sub_tree_hash, workdir_hash);
                rollup_lock_guard.submodule_hashes.insert(entry_path.clone(), submodule_composite_hash.clone());
                children_hashes.push(format!("{}:{}", submodule_name, submodule_composite_hash));
            },
            _ => {},
        }
    }

    children_hashes.sort(); // Ensure consistent order for composite hash
    for hash_part in children_hashes {
        current_tree_hasher.update(hash_part);
    }

    Ok(hex::encode(current_tree_hasher.finalize()))
}


pub fn create_snapshot(repo_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>) -> Result<()> {
    let repo_mutex_arc = Arc::new(Mutex::new(Repository::open(repo_path).context("Failed to open git repository")?));
    let mut rollup_lock_guard = rollup_lock.lock().unwrap();

    // Clear previous Git-related cache entries and new granular hashes
    rollup_lock_guard.file_metadata_cache.retain(|_, metadata| !metadata.is_git_tracked);
    rollup_lock_guard.git_tree_cache.clear();
    rollup_lock_guard.crate_hashes.clear();
    rollup_lock_guard.submodule_hashes.clear();
    rollup_lock_guard.cargo_toml_hashes.clear();
    rollup_lock_guard.rust_file_hashes.clear();
    rollup_lock_guard.project_root_hash = None;


    let real_file_system_stat = RealFileSystemStat::new(repo_mutex_arc.clone());

    let repo = repo_mutex_arc.lock().unwrap();
    let head_commit = repo.head()?.peel_to_commit()?;
    let tree = head_commit.tree()?;

    // Traverse the main repository and populate granular hashes
    let project_root_hash = traverse_git_tree(
        &repo,
        &tree,
        &PathBuf::from(""),
        &mut rollup_lock_guard,
        &real_file_system_stat,
        repo_path,
    )?;
    rollup_lock_guard.project_root_hash = Some(project_root_hash);

    // Calculate Crate Hashes (after all Cargo.toml and .rs files are hashed)
    let mut crate_paths_to_process: Vec<PathBuf> = rollup_lock_guard.cargo_toml_hashes.keys().cloned().collect();
    crate_paths_to_process.sort(); // Ensure consistent order

    for cargo_toml_path in crate_paths_to_process {
        let crate_root = cargo_toml_path.parent().unwrap_or(&cargo_toml_path);
        let mut crate_hasher = Sha256::new();

        // Include Cargo.toml's own hash
        if let Some(hash) = rollup_lock_guard.cargo_toml_hashes.get(&cargo_toml_path) {
            crate_hasher.update(hash);
        }

        // Collect all .rs file hashes within this crate's directory (Git-tracked)
        // This requires re-traversing or carefully collecting during the main traversal.
        // For simplicity here, we'll iterate through already collected rust_file_hashes
        // and check if they belong to this crate.
        let mut rs_file_hashes_for_crate = Vec::new();
        for (rs_path, rs_hash) in &rollup_lock_guard.rust_file_hashes {
            if rs_path.starts_with(crate_root) {
                rs_file_hashes_for_crate.push(rs_hash.clone());
            }
        }
        rs_file_hashes_for_crate.sort(); // Ensure consistent order
        for hash in rs_file_hashes_for_crate {
            crate_hasher.update(hash);
        }

        let crate_hash = hex::encode(crate_hasher.finalize());
        rollup_lock_guard.crate_hashes.insert(cargo_toml_path, crate_hash);
    }

    // 3. Handle untracked files (and other statuses) - existing logic, but now only for metadata
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true)
                  .recurse_untracked_dirs(true)
                  .exclude_submodules(false);

    let statuses = repo.statuses(Some(&mut status_options))?;

    for entry in statuses.iter() {
        if let Some(path_str) = entry.path() {
            let path = PathBuf::from(path_str);
            let full_path = repo_path.join(&path);

            if entry.status().is_wt_new() {
                if let Ok(metadata) = real_file_system_stat.get_metadata(&full_path) {
                    rollup_lock_guard.set_metadata(path.clone(), FileMetadata {
                        git_object_hash: None,
                        is_git_tracked: false,
                        ..metadata
                    });
                }
            }
        }
    }

    rollup_lock_guard.last_snapshot_time = Some(SystemTime::now());

    Ok(())
}
