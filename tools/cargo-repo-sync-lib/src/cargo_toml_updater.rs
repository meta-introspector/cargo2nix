use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use toml_edit::{DocumentMut, Item, Table, value};

pub trait CargoTomlUpdater {
    fn generate_deps_from_names_txt(&self, names_txt_path: &Path, project_root: &Path) -> Result<Vec<String>>;
    fn update_cargo_toml(&self, cargo_toml_path: &Path, generated_deps_content: &[String]) -> Result<()>;
}

pub struct RealCargoTomlUpdater;

impl CargoTomlUpdater for RealCargoTomlUpdater {
    fn generate_deps_from_names_txt(&self, names_txt_path: &Path, project_root: &Path) -> Result<Vec<String>> {
        let mut candidate_entries: HashMap<String, (PathBuf, usize)> = HashMap::new(); // { crate_name: (path_buf, path_length) }

        let content = fs::read_to_string(names_txt_path)
            .with_context(|| format!("Failed to read names.txt file: {}", names_txt_path.display()))?;

        let line_re = Regex::new(r"(.+?)/Cargo.toml:name = \"(.+?)\"")?;

        for line in content.lines() {
            if let Some(captures) = line_re.captures(line) {
                let cargo_toml_path_relative_to_project_root = PathBuf::from(captures[1].to_string());
                let crate_name = captures[2].to_string();

                // 1. Filter by 'submodules/' prefix
                if !cargo_toml_path_relative_to_project_root.starts_with("submodules") {
                    continue;
                }

                // The path to use for the 'path' attribute in Cargo.toml
                let crate_path_for_toml = cargo_toml_path_relative_to_project_root.clone();

                // 2. Calculate Depth relative to 'submodules/'
                let path_segments_count = crate_path_for_toml.iter().count();
                // Assuming "submodules" is the first segment, depth is segments after that.
                // e.g., "submodules/foo" -> 1, "submodules/foo/bar" -> 2
                let depth = if path_segments_count > 1 { path_segments_count - 1 } else { 0 };

                // 3. Apply Max Depth Filter (max 4 directories after 'submodules/')
                if depth > 4 {
                    continue;
                }

                let current_path_length = crate_path_for_toml.to_string_lossy().len();

                // 4. Duplicate Resolution: choose the shorter path
                if let Some((_existing_path, existing_path_length)) = candidate_entries.get(&crate_name) {
                    if current_path_length < *existing_path_length {
                        candidate_entries.insert(crate_name, (crate_path_for_toml, current_path_length));
                    }
                } else {
                    candidate_entries.insert(crate_name, (crate_path_for_toml, current_path_length));
                }
            }
        }

        let mut sorted_crate_names: Vec<String> = candidate_entries.keys().cloned().collect();
        sorted_crate_names.sort_unstable();
        
        let mut generated_deps = Vec::new();
        for crate_name in sorted_crate_names {
            let (path_buf, _) = candidate_entries.get(&crate_name).unwrap();
            generated_deps.push(format!(
                "{} = {{ path = \"{}\" }}",
                crate_name,
                path_buf.display()
            ));
        }

        Ok(generated_deps)
    }

    fn update_cargo_toml(&self, cargo_toml_path: &Path, generated_deps_content: &[String]) -> Result<()> {
        let content = fs::read_to_string(cargo_toml_path)
            .with_context(|| format!("Failed to read Cargo.toml at {}", cargo_toml_path.display()))?;
        
        let mut doc = content.parse::<DocumentMut>()
            .with_context(|| format!("Failed to parse Cargo.toml at {}", cargo_toml_path.display()))?;

        // Ensure [workspace.dependencies] table exists
        let workspace_deps_table = doc
            .entry("workspace")
            .or_insert_auto(Item::Table(Table::new()))
            .as_table_mut()
            .context("Expected 'workspace' to be a table")?
            .entry("dependencies")
            .or_insert_auto(Item::Table(Table::new()))
            .as_table_mut()
            .context("Expected 'workspace.dependencies' to be a table")?;

        // Clear existing dependencies and add new ones
        workspace_deps_table.clear();
        for dep_line in generated_deps_content {
            // Parse the dependency line to extract crate name and path
            let dep_re = Regex::new(r"(\S+)\s*=\s*\{\s*path\s*=\s*\"(.+)\"\s*\}")?;
            if let Some(captures) = dep_re.captures(dep_line) {
                let crate_name = captures[1].to_string();
                let path_str = captures[2].to_string();
                
                let mut dep_table = Table::new();
                dep_table.insert("path", Item::Value(value(path_str)));
                workspace_deps_table.insert(&crate_name, Item::Table(dep_table));
            } else {
                eprintln!("Warning: Could not parse dependency line: {}", dep_line);
            }
        }

        fs::write(cargo_toml_path, doc.to_string().as_bytes())
            .with_context(|| format!("Failed to write updated Cargo.toml to {}", cargo_toml_path.display()))?;

        Ok(())
    }
}
