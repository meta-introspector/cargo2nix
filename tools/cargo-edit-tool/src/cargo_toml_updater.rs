use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml_edit::{value, DocumentMut, Item, Table};
use walkdir::WalkDir;

#[cfg(feature = "nix_generation")]
use crate::analysis::cargo_metadata_provider::{
    CargoMetadataProvider, DummyCargoMetadataProvider, RealCargoMetadataProvider,
};
use cargo_edit_lib::CargoMetadataProvider;
use cargo_toml_editor_lib::api::CargoTomlPatch;
use cargo_toml_editor_lib::executor::CargoEditExecutor;
#[cfg(not(feature = "real_toml_edit"))]
use cargo_toml_editor_lib::executor::DummyCargoEditExecutor;
#[cfg(feature = "real_toml_edit")]
use cargo_toml_editor_lib::executor::RealCargoEditExecutor;

pub trait CargoTomlUpdater {
    fn generate_deps_from_submodules(&self, project_root: &Path) -> Result<Vec<String>>;
    fn update_cargo_toml(
        &self,
        cargo_toml_path: &Path,
        generated_deps_content: &[String],
    ) -> Result<()>;
}

pub struct RealCargoTomlUpdater {
    cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync>,
    cargo_edit_executor: Box<dyn CargoEditExecutor + Send + Sync>,
}

impl RealCargoTomlUpdater {
    pub fn new(
        cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync>,
        cargo_edit_executor: Box<dyn CargoEditExecutor + Send + Sync>,
    ) -> Self {
        RealCargoTomlUpdater {
            cargo_metadata_provider,
            cargo_edit_executor,
        }
    }
}

impl CargoTomlUpdater for RealCargoTomlUpdater {
    fn generate_deps_from_submodules(&self, project_root: &Path) -> Result<Vec<String>> {
        let mut candidate_entries: HashMap<String, (PathBuf, usize)> = HashMap::new(); // { crate_name: (path_buf, path_length) }
        let submodules_dir = project_root.join("submodules");

        for entry in WalkDir::new(&submodules_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
        {
            let cargo_toml_path = entry.path();
            let crate_dir = cargo_toml_path.parent().unwrap();
            let crate_path_for_toml = crate_dir.strip_prefix(project_root).unwrap().to_path_buf();

            // 2. Calculate Depth relative to 'submodules/'
            let path_segments_count = crate_path_for_toml.iter().count();
            let depth = if path_segments_count > 1 {
                path_segments_count - 1
            } else {
                0
            };

            // 3. Apply Max Depth Filter (max 4 directories after 'submodules/')
            if depth > 4 {
                continue;
            }

            let metadata = self
                .cargo_metadata_provider
                .provide_metadata(cargo_toml_path)?;

            if let Some(package) = metadata.packages.get(0) {
                let crate_name = package.name.to_string();
                let current_path_length = crate_path_for_toml.to_string_lossy().len();

                // 4. Duplicate Resolution: choose the shorter path
                if let Some((_existing_path, existing_path_length)) =
                    candidate_entries.get(&crate_name)
                {
                    if current_path_length < *existing_path_length {
                        candidate_entries.insert(
                            crate_name,
                            (crate_path_for_toml.clone(), current_path_length),
                        );
                    }
                } else {
                    candidate_entries.insert(
                        crate_name,
                        (crate_path_for_toml.clone(), current_path_length),
                    );
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

    fn update_cargo_toml(
        &self,
        cargo_toml_path: &Path,
        generated_deps_content: &[String],
    ) -> Result<()> {
        let content = self.cargo_edit_executor.read_cargo_toml(cargo_toml_path)?;

        let mut doc = content.parse::<DocumentMut>().with_context(|| {
            format!(
                "Failed to parse Cargo.toml at {}",
                cargo_toml_path.display()
            )
        })?;

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
        let mut patches = Vec::new();
        for dep_line in generated_deps_content {
            // Example dep_line: `crate-name = { path = "path/to/crate" }`
            let parts: Vec<&str> = dep_line.splitn(2, '=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let crate_name = parts[0].to_string();
                let path_part = parts[1]
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .trim();
                let path_parts: Vec<&str> = path_part.splitn(2, '=').map(|s| s.trim()).collect();
                if path_parts.len() == 2 && path_parts[0] == "path" {
                    let path_str = path_parts[1].trim_matches('"').to_string();
                    patches.push(CargoTomlPatch {
                        section: "workspace.dependencies".to_string(),
                        key: crate_name,
                        value: format!("{{ path = \"{}\" }}", path_str),
                    });
                }
            }
        }

        self.cargo_edit_executor
            .apply_patches(cargo_toml_path, patches)?;

        Ok(())
    }
}
