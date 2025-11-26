// crates/tool-refactorings/src/package_extractor.rs

use gemini_utils::gemini_eprintln;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Trait for extracting package information (name and path) from Cargo.toml files.
pub trait PackageExtractor {
    /// Extracts package names and their paths from Cargo.toml files within a given root directory,
    /// excluding those that match specified patterns.
    ///
    /// `root_dir`: The root directory to start scanning from.
    /// `exclude_keywords`: A regex pattern used to exclude package paths or names.
    ///
    /// Returns a vector of tuples `(package_name, package_path)`.
    fn extract_packages(&self, root_dir: &Path, exclude_keywords: &str) -> Vec<(String, PathBuf)>;
}

/// Dummy implementation of `PackageExtractor` for testing and simulation.
pub struct DefaultPackageExtractor {
    // Simulate a file system for testing with pre-defined Cargo.toml contents
    pub file_system: HashMap<PathBuf, String>,
}

impl DefaultPackageExtractor {
    pub fn new(file_system: HashMap<PathBuf, String>) -> Self {
        Self { file_system }
    }

    // Helper to simulate reading a file
    fn mock_read_file(&self, path: &Path) -> Option<&String> {
        self.file_system.get(path)
    }
}

impl PackageExtractor for DefaultPackageExtractor {
    fn extract_packages(&self, root_dir: &Path, exclude_keywords: &str) -> Vec<(String, PathBuf)> {
        let mut extracted_packages = Vec::new;
        let exclude_regex = Regex::new(exclude_keywords).expect("Invalid regex for exclude keywords");

        gemini_eprintln!("Extracting packages from :root_dir: with exclude keywords: :keywords:", root_dir = root_dir.display(), keywords = exclude_keywords);

        // Simulate finding Cargo.toml files
        for (path, content) in &self.file_system {
            if path.file_name().map_or(false, |f| f == "Cargo.toml") && path.starts_with(root_dir) {
                let cargo_toml_dir = path.parent().unwrap_or(path);

                // Check if the directory path contains any exclude keywords
                if exclude_regex.is_match(&cargo_toml_dir.to_string_lossy()) {
                    gemini_eprintln!("Skipping path :path: due to exclude keywords.", path = cargo_toml_dir.display());
                    continue;
                }

                // Extract package name
                if let Some(name_line) = content.lines().find(|line| line.trim().starts_with("name = ")) {
                    let package_name = name_line.split('=').nth(1).unwrap_or("").trim().trim_matches('"').to_string();

                    if !package_name.is_empty() {
                        // Check if the package name itself contains any exclude keywords
                        if exclude_regex.is_match(&package_name) {
                            gemini_eprintln!("Skipping package :name: due to exclude keywords.", name = package_name);
                            continue;
                        }
                        extracted_packages.push((package_name, cargo_toml_dir.to_path_buf()));
                    }
                }
            }
        }
        extracted_packages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_mock_file_system() -> HashMap<PathBuf, String> {
        let mut fs = HashMap::new();
        fs.insert(PathBuf::from("/project/Cargo.toml"), "[package]\nname = \"project-root\"".to_string());
        fs.insert(PathBuf::from("/project/src/main.rs"), "fn main() {}".to_string());
        fs.insert(PathBuf::from("/project/submodules/my-crate/Cargo.toml"), "[package]\nname = \"my-crate\"".to_string());
        fs.insert(PathBuf::from("/project/submodules/test-crate/Cargo.toml"), "[package]\nname = \"test-crate\"".to_string());
        fs.insert(PathBuf::from("/project/examples/example-app/Cargo.toml"), "[package]\nname = \"example-app\"".to_string());
        fs.insert(PathBuf::from("/project/bench/bench-utils/Cargo.toml"), "[package]\nname = \"bench-utils\"".to_string());
        fs.insert(PathBuf::from("/project/my-lib/Cargo.toml"), "[package]\nname = \"my-lib\"".to_string());
        fs
    }

    #[test]
    fn test_extract_packages_basic() {
        let root_dir = PathBuf::from("/project");
        let exclude_keywords = "bench|test|example";
        let extractor = DefaultPackageExtractor::new(create_mock_file_system());
        let packages = extractor.extract_packages(&root_dir, exclude_keywords);

        assert_eq!(packages.len(), 3);
        assert!(packages.iter().any(|(name, _)| name == "project-root"));
        assert!(packages.iter().any(|(name, _)| name == "my-crate"));
        assert!(packages.iter().any(|(name, _)| name == "my-lib"));
        assert!(!packages.iter().any(|(name, _)| name == "test-crate"));
        assert!(!packages.iter().any(|(name, _)| name == "example-app"));
        assert!(!packages.iter().any(|(name, _)| name == "bench-utils"));
    }

    #[test]
    fn test_extract_packages_no_exclude() {
        let root_dir = PathBuf::from("/project");
        let exclude_keywords = ""; // No exclusions
        let extractor = DefaultPackageExtractor::new(create_mock_file_system());
        let packages = extractor.extract_packages(&root_dir, exclude_keywords);

        assert_eq!(packages.len(), 6); // All packages should be included
    }

    #[test]
    fn test_extract_packages_path_exclusion() {
        let root_dir = PathBuf::from("/project");
        let exclude_keywords = "submodules"; // Exclude paths containing "submodules"
        let extractor = DefaultPackageExtractor::new(create_mock_file_system());
        let packages = extractor.extract_packages(&root_dir, exclude_keywords);

        assert_eq!(packages.len(), 4);
        assert!(packages.iter().any(|(name, _)| name == "project-root"));
        assert!(packages.iter().any(|(name, _)| name == "example-app"));
        assert!(packages.iter().any(|(name, _)| name == "bench-utils"));
        assert!(packages.iter().any(|(name, _)| name == "my-lib"));
        assert!(!packages.iter().any(|(name, _)| name == "my-crate"));
        assert!(!packages.iter().any(|(name, _)| name == "test-crate"));
    }

    #[test]
    fn test_extract_packages_name_exclusion() {
        let root_dir = PathBuf::from("/project");
        let exclude_keywords = "bench-utils|test-crate"; // Exclude names
        let extractor = DefaultPackageExtractor::new(create_mock_file_system());
        let packages = extractor.extract_packages(&root_dir, exclude_keywords);

        assert_eq!(packages.len(), 4);
        assert!(packages.iter().any(|(name, _)| name == "project-root"));
        assert!(packages.iter().any(|(name, _)| name == "my-crate"));
        assert!(packages.iter().any(|(name, _)| name == "example-app"));
        assert!(packages.iter().any(|(name, _)| name == "my-lib"));
        assert!(!packages.iter().any(|(name, _)| name == "test-crate"));
        assert!(!packages.iter().any(|(name, _)| name == "bench-utils"));
    }
}
