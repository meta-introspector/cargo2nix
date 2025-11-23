use std::path::{Path, PathBuf};
use std::fs;
use toml::Value;
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use sha2::{Digest, Sha256};
use hex;

use crate::error::AppError; // Assuming AppError is defined in rust-src-scanner

pub struct CargoParser {
    cache: HashMap<PathBuf, Value>,
    graph: DiGraph<String, String>, // Node weight: crate name, Edge weight: dependency type/version
    package_name_to_node_index: HashMap<String, NodeIndex>,
    cache_dir: Option<PathBuf>,
}

impl CargoParser {
    pub fn new(cache_dir: Option<PathBuf>) -> Self {
        CargoParser {
            cache: HashMap::new(),
            graph: DiGraph::new(),
            package_name_to_node_index: HashMap::new(),
            cache_dir,
        }
    }

    pub fn find_cargo_toml_files(path: &Path) -> Result<Vec<PathBuf>, AppError> {
        let mut cargo_toml_files = Vec::new();
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
                cargo_toml_files.push(path.to_path_buf());
            }
        }
        Ok(cargo_toml_files)
    }

    pub fn parse_cargo_toml(&mut self, path: &Path) -> Result<Value, AppError> {
        // Check in-memory cache first
        if let Some(cached_value) = self.cache.get(path) {
            return Ok(cached_value.clone());
        }

        let mut cache_file_path: Option<PathBuf> = None;
        if let Some(cd) = &self.cache_dir {
            let mut hasher = Sha256::new();
            hasher.update(path.to_string_lossy().as_bytes());
            let hash = hex::encode(hasher.finalize());
            let cf_path = cd.join(format!("{}.json", hash));
            cache_file_path = Some(cf_path);

            // Check file system cache
            if let Some(cfp) = &cache_file_path {
                if cfp.exists() {
                    let cached_content = fs::read_to_string(cfp).map_err(AppError::Io)?;
                    let cached_value: Value = serde_json::from_str(&cached_content).map_err(AppError::Json)?;
                    self.add_package_to_graph(&cached_value)?;
                    self.cache.insert(path.to_path_buf(), cached_value.clone());
                    return Ok(cached_value);
                }
            }
        }

        // If not in cache, read and parse
        let content = fs::read_to_string(path).map_err(AppError::Io)?;
        let value: Value = toml::from_str(&content).map_err(AppError::TomlDe)?;

        self.add_package_to_graph(&value)?;
        self.cache.insert(path.to_path_buf(), value.clone());

        // Write to file system cache
        if let Some(cfp) = &cache_file_path {
            fs::create_dir_all(cfp.parent().unwrap()).map_err(AppError::Io)?;
            let serialized_value = serde_json::to_string_pretty(&value).map_err(AppError::Json)?;
            fs::write(cfp, serialized_value).map_err(AppError::Io)?;
        }

        Ok(value)
    }

    fn add_package_to_graph(&mut self, cargo_toml_value: &Value) -> Result<(), AppError> {
        let package_name = cargo_toml_value
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .ok_or_else(|| AppError::Other("Could not find package name in Cargo.toml".to_string()))?
            .to_string();

        let package_node_index = *self.package_name_to_node_index.entry(package_name.clone()).or_insert_with(|| {
            self.graph.add_node(package_name.clone())
        });

        // Helper to add dependencies
        let mut add_deps = |deps_table: &Value, dep_type: &str| -> Result<(), AppError> {
            if let Some(deps) = deps_table.as_table() {
                for (dep_name, _dep_info) in deps {
                    let dep_node_index = *self.package_name_to_node_index.entry(dep_name.clone()).or_insert_with(|| {
                        self.graph.add_node(dep_name.clone())
                    });
                    self.graph.add_edge(package_node_index, dep_node_index, dep_type.to_string());
                }
            }
            Ok(())
        };

        if let Some(dependencies) = cargo_toml_value.get("dependencies") {
            add_deps(dependencies, "dependency")?;
        }
        if let Some(dev_dependencies) = cargo_toml_value.get("dev-dependencies") {
            add_deps(dev_dependencies, "dev-dependency")?;
        }
        if let Some(build_dependencies) = cargo_toml_value.get("build-dependencies") {
            add_deps(build_dependencies, "build-dependency")?;
        }

        Ok(())
    }

    pub fn get_dependency_graph(&self) -> &DiGraph<String, String> {
        &self.graph
    }
}
