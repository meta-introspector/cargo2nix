use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir;
use crate::error::AppError;

pub trait CrateDiscovery {
    fn scan_crates(&self, rust_src_path: &Path) -> Result<HashMap<String, PathBuf>, AppError>;
}

pub struct FileSystemCrateDiscovery;

impl CrateDiscovery for FileSystemCrateDiscovery {
    fn scan_crates(&self, rust_src_path: &Path) -> Result<HashMap<String, PathBuf>, AppError> {
        let mut crate_name_to_root_map: HashMap<String, PathBuf> = HashMap::new();
        
        println!("Scanning for Cargo.toml files to build crate map...");
        
        for entry in walkdir::WalkDir::new(rust_src_path) {
            let entry = entry.map_err(AppError::Walkdir)?;
            if entry.file_name() == "Cargo.toml" {
                let cargo_toml_path = entry.path();
                
                match parse_cargo_toml(cargo_toml_path) {
                    Ok(Some((name, root_path))) => {
                        crate_name_to_root_map.insert(name, root_path);
                    }
                    Ok(None) => {
                        // No package name found, skip
                    }
                    Err(e) => {
                        println!("Warning: Failed to parse Cargo.toml at {:?}: {}. Skipping this file.", 
                                cargo_toml_path, e);
                    }
                }
            }
        }
        
        println!("Crate map built with {} entries.", crate_name_to_root_map.len());
        Ok(crate_name_to_root_map)
    }
}

fn parse_cargo_toml(cargo_toml_path: &Path) -> Result<Option<(String, PathBuf)>, AppError> {
    let content = fs::read_to_string(cargo_toml_path).map_err(AppError::Io)?;
    let cargo_toml: toml::Value = content.parse()
        .map_err(|e| AppError::Custom(format!("Failed to parse TOML: {}", e)))?;

    if let Some(package) = cargo_toml.get("package") {
        if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
            if let Some(parent) = cargo_toml_path.parent() {
                return Ok(Some((name.to_string(), parent.to_path_buf())));
            }
        }
    }
    
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        let content = r#"
[package]
name = "test-crate"
version = "0.1.0"
"#;
        fs::write(&cargo_toml_path, content).unwrap();
        
        let result = parse_cargo_toml(&cargo_toml_path).unwrap();
        assert!(result.is_some());
        
        let (name, path) = result.unwrap();
        assert_eq!(name, "test-crate");
        assert_eq!(path, temp_dir.path());
    }
}
