use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use toml_edit::DocumentMut;
use regex::Regex; // Import Regex

pub trait WorkspaceRemover {
    fn remove_workspace_sections(&self, submodules_dir: &Path) -> Result<()>;
}

pub struct RealWorkspaceRemover;

impl WorkspaceRemover for RealWorkspaceRemover {
    fn remove_workspace_sections(&self, submodules_dir: &Path) -> Result<()> {
        println!("Scanning for Cargo.toml files in submodules under: {}", submodules_dir.display());

        // Regex to find the [workspace] header and any subsequent blank lines or comments
        let workspace_header_re = Regex::new(r"(?m)^\[workspace\]\s*(?:\n\s*(?:#.*)?)*")?;

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

            let mut changed = false;

            if doc.contains_key("workspace") {
                doc.remove("workspace");
                changed = true;
                println!("  Removed [workspace] table from {}", cargo_toml_path.display());
            }

            // Convert back to string and use regex to remove any remaining [workspace] header
            // and associated blank lines/comments that toml_edit might have left behind.
            let mut new_content = doc.to_string();
            if workspace_header_re.is_match(&new_content) {
                new_content = workspace_header_re.replace_all(&new_content, "").to_string();
                changed = true;
                println!("  Removed [workspace] header remnants from {}", cargo_toml_path.display());
            }

            if changed {
                std::fs::write(&cargo_toml_path, new_content.as_bytes())
                    .with_context(|| format!("Failed to write to {}", cargo_toml_path.display()))?;
                println!("  Successfully updated {}", cargo_toml_path.display());
            } else {
                println!("  No [workspace] section found or changes needed in {}", cargo_toml_path.display());
            }
        }
        Ok(())
    }
}