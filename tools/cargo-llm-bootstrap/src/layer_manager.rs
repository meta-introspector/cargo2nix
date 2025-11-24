use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::AppError;

pub trait LayerManager {
    fn load_layers(&self, layered_graph_path: &Path) -> Result<HashMap<String, u32>, AppError>;
    fn get_crates_for_layer(&self, layer: u32, layered_crates: &HashMap<String, u32>, 
                           crate_name_to_root_map: &HashMap<String, PathBuf>) -> Vec<PathBuf>;
    fn get_max_layer(&self, layered_crates: &HashMap<String, u32>) -> u32;
}

pub struct FileSystemLayerManager;

impl LayerManager for FileSystemLayerManager {
    fn load_layers(&self, layered_graph_path: &Path) -> Result<HashMap<String, u32>, AppError> {
        if layered_graph_path.exists() {
            let content = fs::read_to_string(layered_graph_path)
                .map_err(|e| AppError::Custom(format!("Failed to read layered_graph.json: {}", e)))?;
            
            serde_json::from_str(&content)
                .map_err(|e| AppError::Custom(format!("Failed to parse layered_graph.json: {}", e)))
        } else {
            println!("Warning: layered_graph.json not found at {:?}. Level filtering will not be applied.", 
                    layered_graph_path);
            Ok(HashMap::new())
        }
    }

    fn get_crates_for_layer(&self, layer: u32, layered_crates: &HashMap<String, u32>, 
                           crate_name_to_root_map: &HashMap<String, PathBuf>) -> Vec<PathBuf> {
        let mut crates_in_layer: Vec<PathBuf> = Vec::new();
        let mut processed_crate_roots: HashSet<PathBuf> = HashSet::new();

        for (crate_name, &crate_layer) in layered_crates {
            if crate_layer == layer {
                if let Some(crate_root_path) = crate_name_to_root_map.get(crate_name) {
                    if processed_crate_roots.insert(crate_root_path.clone()) {
                        crates_in_layer.push(crate_root_path.clone());
                    }
                } else {
                    println!("Warning: Could not find crate root path in map for crate: {}", crate_name);
                }
            }
        }

        // Sort crates within the layer for consistent processing order
        crates_in_layer.sort_by(|a_path, b_path| {
            let a_name = a_path.file_name().unwrap().to_string_lossy().to_string();
            let b_name = b_path.file_name().unwrap().to_string_lossy().to_string();

            let a_layer = layered_crates.get(&a_name).unwrap_or(&0);
            let b_layer = layered_crates.get(&b_name).unwrap_or(&0);

            a_layer.cmp(b_layer)
        });

        crates_in_layer
    }

    fn get_max_layer(&self, layered_crates: &HashMap<String, u32>) -> u32 {
        layered_crates.values().max().cloned().unwrap_or(0)
    }
}

pub fn apply_limit(crates: Vec<PathBuf>, limit: Option<u32>) -> Vec<PathBuf> {
    match limit {
        Some(limit) => crates.into_iter().take(limit as usize).collect(),
        None => crates,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_layer() {
        let manager = FileSystemLayerManager;
        let mut layers = HashMap::new();
        layers.insert("crate1".to_string(), 0);
        layers.insert("crate2".to_string(), 5);
        layers.insert("crate3".to_string(), 3);
        
        assert_eq!(manager.get_max_layer(&layers), 5);
    }

    #[test]
    fn test_apply_limit() {
        let crates = vec![
            PathBuf::from("crate1"),
            PathBuf::from("crate2"),
            PathBuf::from("crate3"),
        ];
        
        let limited = apply_limit(crates.clone(), Some(2));
        assert_eq!(limited.len(), 2);
        
        let unlimited = apply_limit(crates.clone(), None);
        assert_eq!(unlimited.len(), 3);
    }
}
