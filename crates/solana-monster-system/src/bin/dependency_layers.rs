use std::process::Command;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct DependencyLayer {
    layer: usize,
    crates: Vec<String>,
}

struct Database;

impl Database {
    fn analyze_dependency_layers(&self) -> Vec<DependencyLayer> {
        let mut layers = Vec::new();
        let solana_root = "/home/mdupont/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
        
        // Parse main Cargo.lock for dependency graph
        let cargo_lock_path = format!("{}/Cargo.lock", solana_root);
        let dependencies = self.parse_cargo_lock(&cargo_lock_path);
        
        // Build 31 layers based on dependency depth
        let layer_map = self.calculate_dependency_layers(&dependencies);
        
        for layer_num in 1..=31 {
            let crates_in_layer: Vec<String> = layer_map.iter()
                .filter(|(_, &layer)| layer == layer_num)
                .map(|(name, _)| name.clone())
                .collect();
                
            if !crates_in_layer.is_empty() {
                layers.push(DependencyLayer {
                    layer: layer_num,
                    crates: crates_in_layer,
                });
            }
        }
        
        layers
    }
    
    fn parse_cargo_lock(&self, path: &str) -> HashMap<String, Vec<String>> {
        let mut dependencies = HashMap::new();
        
        if let Ok(output) = Command::new("grep").args(&["-A", "5", "\\[\\[package\\]\\]", path]).output() {
            let content = String::from_utf8_lossy(&output.stdout);
            let mut current_package = String::new();
            
            for line in content.lines() {
                if line.starts_with("name = ") {
                    current_package = line.replace("name = ", "").trim_matches('"').to_string();
                } else if line.starts_with("dependencies = [") && !current_package.is_empty() {
                    let deps: Vec<String> = line
                        .replace("dependencies = [", "")
                        .replace("]", "")
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    dependencies.insert(current_package.clone(), deps);
                }
            }
        }
        
        dependencies
    }
    
    fn calculate_dependency_layers(&self, dependencies: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
        let mut layer_map = HashMap::new();
        let mut visited = HashSet::new();
        
        // Start with root crates (no dependencies) at layer 1
        for (name, deps) in dependencies {
            if deps.is_empty() {
                layer_map.insert(name.clone(), 1);
                visited.insert(name.clone());
            }
        }
        
        // Build layers 2-31 based on dependency depth
        for layer in 2..=31 {
            let mut new_crates = Vec::new();
            
            for (name, deps) in dependencies {
                if !visited.contains(name) {
                    let max_dep_layer = deps.iter()
                        .filter_map(|dep| layer_map.get(dep))
                        .max()
                        .unwrap_or(&0);
                        
                    if *max_dep_layer == layer - 1 {
                        new_crates.push(name.clone());
                    }
                }
            }
            
            for name in new_crates {
                layer_map.insert(name.clone(), layer);
                visited.insert(name);
            }
        }
        
        layer_map
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 31-Layer Dependency Analysis ===");
    
    let db = Database;
    let layers = db.analyze_dependency_layers();
    let layer_count = layers.len();
    
    println!("query SolanaRustcDependencyLayers {{");
    
    for layer in &layers {
        println!("  layer{}: [", layer.layer);
        for (i, crate_name) in layer.crates.iter().enumerate() {
            println!("    {}. {}", i + 1, crate_name);
        }
        println!("  ] ({} crates)", layer.crates.len());
    }
    
    println!("}}");
    println!("\nTotal layers: {} (target: 31)", layer_count);
    
    Ok(())
}
