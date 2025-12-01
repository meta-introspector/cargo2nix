use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileScanner;

impl FileScanner {
    pub fn new() -> Self {
        Self
    }

    pub fn find_rust_files(&self, root_path: &Path) -> Result<Vec<PathBuf>> {
        let mut rust_files = Vec::new();

        for entry in WalkDir::new(root_path).follow_links(false) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "rs" {
                        rust_files.push(entry.path().to_path_buf());
                    }
                }
            }
        }

        Ok(rust_files)
    }
}
