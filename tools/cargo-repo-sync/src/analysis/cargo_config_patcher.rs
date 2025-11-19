use anyhow::{Result, Context};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use regex::Regex;
use toml_edit::{DocumentMut, Item, Table, value};
use std::sync::Arc; // Import Arc
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider}; // Import the trait and its real implementation

pub trait CargoConfigPatcher {
    fn generate_patches(
        &self,
        tree_file: &Path,
        cargo_lock_file: &Path,
        config_file: &Path,
        project_root: &Path,
    ) -> Result<Vec<String>>;
}

pub struct RealCargoConfigPatcher {
    metadata_provider: Arc<dyn CargoMetadataProvider>,
}

impl RealCargoConfigPatcher {
    pub fn new(metadata_provider: Arc<dyn CargoMetadataProvider>) -> Self {
        RealCargoConfigPatcher { metadata_provider }
    }
}

impl CargoConfigPatcher for RealCargoConfigPatcher {
    fn generate_patches(
        &self,
        tree_file: &Path,
        cargo_lock_file: &Path,
        config_file: &Path,
        project_root: &Path,
    ) -> Result<Vec<String>> {
        // 1. Parse tree.txt to get submodule paths
        let mut submodule_paths: HashMap<String, PathBuf> = HashMap::new();
        let tree_content = std::fs::read_to_string(tree_file)
            .with_context(|| format!("Failed to read tree file: {}", tree_file.display()))?;
        
        let submodules_path_pattern = regex::escape(&project_root.join("submodules").to_string_lossy());
        let submodule_path_re = Regex::new(&format!(r"^\s*├──\s+(\S+)\s+v\S+\s+.*?(?P<path>{}[^\s)]+)", submodules_path_pattern))?;

        for line in tree_content.lines() {
            if let Some(captures) = submodule_path_re.captures(line) {
                let package_name = captures[1].to_string();
                let full_path = PathBuf::from(captures.name("path").unwrap().as_str());
                let relative_path = pathdiff::diff_paths(&full_path, project_root)
                    .context(format!("Failed to get relative path for {:?}", full_path))?;
                submodule_paths.insert(package_name, relative_path);
            }
        }

        // 2. Parse Cargo.lock to get all package names using CargoMetadataProvider
        let cargo_lock_packages = self.metadata_provider.get_package_names_from_lock_file(cargo_lock_file)?;

        // 3. Read existing config.toml to identify already patched/commented packages
        let mut existing_patched_packages = HashSet::new();
        let mut commented_out_packages: HashSet<String> = HashSet::new(); // This is not currently used, but kept for future consistency
        
        let mut config_doc = if config_file.exists() {
            let content = std::fs::read_to_string(config_file)
                .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;
            content.parse::<DocumentMut>().context("Failed to parse config.toml")?
        } else {
            DocumentMut::new()
        };

        if let Some(patch_table) = config_doc.get("patch").and_then(|item| item.as_table_like()) {
            if let Some(crates_io_table) = patch_table.get("crates-io").and_then(|item| item.as_table_like()) {
                for (key, value) in crates_io_table.iter() {
                    if value.is_inline_table() {
                        existing_patched_packages.insert(key.to_string());
                    }
                }
            }
        }

        // 4. Generate new patches
        let mut new_crates_io_patches_to_add: Vec<String> = Vec::new();
        for package_name in cargo_lock_packages {
            if let Some(relative_path) = submodule_paths.get(&package_name) {
                if !existing_patched_packages.contains(&package_name) && !commented_out_packages.contains(&package_name) {
                    new_crates_io_patches_to_add.push(format!(
                        "    {} = {{ path = \"{}\" }}",
                        package_name,
                        relative_path.display()
                    ));
                }
            }
        }
        Ok(new_crates_io_patches_to_add)
    }
}
