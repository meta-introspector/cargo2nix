use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use toml_edit::DocumentMut;

pub trait WorkspaceRemover {
    fn remove_workspace_sections(&self, submodules_dir: &Path) -> Result<()>;
}

pub struct RealWorkspaceRemover;

impl WorkspaceRemover for RealWorkspaceRemover {
    fn remove_workspace_sections(&self, submodules_dir: &Path) -> Result<()> {
        println!("Scanning for Cargo.toml files in submodules under: {}", submodules_dir.display());

        for entry in WalkDir::new(submodules_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
        {
            let cargo_toml_path = entry.path();
            println!("Processing {}...", cargo_toml_path.display());

            let content = std::fs::read_to_string(&cargo_toml_path)
                .with_context(|| format!("Failed to read Cargo.toml at {}", cargo_toml_path.display()))?;
            
            let mut doc = content.parse::<DocumentMut>()
                .with_context(|| format!("Failed to parse Cargo.toml at {}", cargo_toml_path.display()))?;

            if doc.contains_key("workspace") {
                doc.remove("workspace");
                std::fs::write(&cargo_toml_path, doc.to_string().as_bytes())
                    .with_context(|| format!("Failed to write to {}", cargo_toml_path.display()))?;
                println!("  Successfully removed [workspace] section from {}", cargo_toml_path.display());
            } else {
                println!("  No [workspace] section found in {}", cargo_toml_path.display());
            }
        }
        Ok(())
    }
}