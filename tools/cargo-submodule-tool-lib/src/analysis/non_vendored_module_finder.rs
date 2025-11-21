use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::Path;
#[cfg(feature = "nix_generation")] // Conditionally compile regex
use regex::Regex;

pub trait NonVendoredModuleFinder {
    fn find_and_count_non_vendored(&self, tree_file_path: &Path, project_root: &Path) -> Result<HashMap<String, u32>>;
}

#[cfg(feature = "nix_generation")] // Conditionally compile RealNonVendoredModuleFinder
pub struct RealNonVendoredModuleFinder;

#[cfg(feature = "nix_generation")] // Conditionally compile impl block
impl NonVendoredModuleFinder for RealNonVendoredModuleFinder {
    fn find_and_count_non_vendored(&self, tree_file_path: &Path, project_root: &Path) -> Result<HashMap<String, u32>> {
        let mut module_usage_counts = HashMap::new();

        let content = std::fs::read_to_string(tree_file_path)
            .with_context(|| format!("Failed to read tree file: {}", tree_file_path.display()))?;

        // Regex to extract package name and path from any dependency line in tree.txt
        let package_path_regex = Regex::new(r"^[│\s]*(?:├──|└──)?\s*(\w[\w-]*)\s+v\S+(?:\s+\(([^)]+)\))?")?;

        let submodules_path_str = project_root.join("submodules").to_string_lossy().to_string();

        for line in content.lines() {
            if let Some(captures) = package_path_regex.captures(line) {
                let package_name = captures[1].to_string();
                let package_path_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");

                let is_vendored = package_path_str.contains(&submodules_path_str);

                if !is_vendored {
                    *module_usage_counts.entry(package_name).or_insert(0) += 1;
                }
            }
        }
        Ok(module_usage_counts)
    }
}

#[cfg(not(feature = "nix_generation"))] // Dummy implementation when nix_generation is not enabled
pub struct RealNonVendoredModuleFinder;

#[cfg(not(feature = "nix_generation"))]
impl NonVendoredModuleFinder for RealNonVendoredModuleFinder {
    fn find_and_count_non_vendored(&self, _tree_file_path: &Path, _project_root: &Path) -> Result<HashMap<String, u32>> {
        anyhow::bail!("`NonVendoredModuleFinder` requires the `nix_generation` feature to be enabled.");
    }
}
