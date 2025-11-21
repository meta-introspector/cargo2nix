use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use toml_edit::{DocumentMut, Item, Table, value};
use walkdir::WalkDir;
use cargo_metadata::MetadataCommand;

pub trait CargoTomlUpdater {
    fn generate_deps_from_submodules(&self, project_root: &Path) -> Result<Vec<String>>;
    fn update_cargo_toml(&self, cargo_toml_path: &Path, generated_deps_content: &[String]) -> Result<()>;
}

pub struct RealCargoTomlUpdater;

impl CargoTomlUpdater for RealCargoTomlUpdater {
    fn generate_deps_from_submodules(&self, project_root: &Path) -> Result<Vec<String>> {
        let mut candidate_entries: HashMap<String, (PathBuf, usize)> = HashMap::new(); // { crate_name: (path_buf, path_length) }
        let submodules_dir = project_root.join("submodules");

        for entry in WalkDir::new(&submodules_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_str() == Some("Cargo.toml"))
        {
            let cargo_toml_path = entry.path();
            let crate_dir = cargo_toml_path.parent().unwrap();
            let crate_path_for_toml = crate_dir.strip_prefix(project_root).unwrap().to_path_buf();

            // 2. Calculate Depth relative to 'submodules/'
            let path_segments_count = crate_path_for_toml.iter().count();
            let depth = if path_segments_count > 1 { path_segments_count - 1 } else { 0 };

            // 3. Apply Max Depth Filter (max 4 directories after 'submodules/')
            if depth > 4 {
                continue;
            }

            let metadata = MetadataCommand::new()
                .manifest_path(cargo_toml_path)
                .no_deps()
                .exec()?;

            if let Some(package) = metadata.packages.get(0) {
                let crate_name = package.name.to_string();
                let current_path_length = crate_path_for_toml.to_string_lossy().len();

                // 4. Duplicate Resolution: choose the shorter path
                if let Some((_existing_path, existing_path_length)) = candidate_entries.get(&crate_name) {
                    if current_path_length < *existing_path_length {
                        candidate_entries.insert(crate_name, (crate_path_for_toml.clone(), current_path_length));
                    }
                } else {
                    candidate_entries.insert(crate_name, (crate_path_for_toml.clone(), current_path_length));
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
            .or_insert(Item::Table(Table::new()))
            .as_table_mut()
            .context("Expected 'workspace' to be a table")?
            .entry("dependencies")
            .or_insert(Item::Table(Table::new()))
            .as_table_mut()
            .context("Expected 'workspace.dependencies' to be a table")?;

        // Clear existing dependencies and add new ones
        workspace_deps_table.clear();
        for dep_line in generated_deps_content {
            // Example dep_line: `crate-name = { path = "path/to/crate" }`
            let parts: Vec<&str> = dep_line.splitn(2, '=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let crate_name = parts[0].to_string();
                let path_part = parts[1].trim_start_matches('{').trim_end_matches('}').trim();
                let path_parts: Vec<&str> = path_part.splitn(2, '=').map(|s| s.trim()).collect();
                if path_parts.len() == 2 && path_parts[0] == "path" {
                    let path_str = path_parts[1].trim_matches('"').to_string();
                    let mut dep_table = Table::new();
                    dep_table.insert("path", value(path_str));
                    workspace_deps_table.insert(&crate_name, Item::Table(dep_table));
                }
            }
        }

        fs::write(cargo_toml_path, doc.to_string().as_bytes())
            .with_context(|| format!("Failed to write updated Cargo.toml to {}", cargo_toml_path.display()))?;

        Ok(())
    }
}