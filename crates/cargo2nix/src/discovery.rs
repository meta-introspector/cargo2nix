use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;

pub fn find_cargo_manifests(root_path: &Path) -> Result<Vec<PathBuf>> {
    let mut manifests = Vec::new();

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && entry.file_name() == "Cargo.toml" {
            manifests.push(entry.into_path());
        }
    }

    Ok(manifests)
}

pub fn find_cargo_locks(root_path: &Path) -> Result<Vec<PathBuf>> {
    let mut locks = Vec::new();

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && entry.file_name() == "Cargo.lock" {
            locks.push(entry.into_path());
        }
    }

    Ok(locks)
}