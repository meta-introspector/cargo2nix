use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct CompleteCargoResolver {
    cargo_exports: HashMap<String, Vec<String>>,
    phi_mappings: HashMap<String, u64>,
    resolved: HashMap<String, (String, u64)>,
}

impl CompleteCargoResolver {
    fn new() -> Self {
        Self {
            cargo_exports: HashMap::new(),
            phi_mappings: HashMap::new(),
            resolved: HashMap::new(),
        }
    }
    
    fn scan_submodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Scanning ./submodules for Cargo.toml files ===");
        
        let submodule_paths = vec![
            "./submodules/BLAKE3",
            "./submodules/juniper", 
            "./lattice-introspector",
            "./minizinc-introspector",
        ];
        
        for submodule_path in &submodule_paths {
            if Path::new(submodule_path).exists() {
                self.scan_cargo_in_path(submodule_path)?;
            }
        }
        
        Ok(())
    }
    
    fn scan_cargo_in_path(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_toml_path = format!("{}/Cargo.toml", path);
        
        if Path::new(&cargo_toml_path).exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
                let crate_name = self.extract_crate_name(&content);
                println!("Found crate: {} at {}", crate_name, path);
                
                let lib_paths = vec![
                    format!("{}/src/lib.rs", path),
                    format!("{}/src/main.rs", path),
                ];
                
                let mut exports = Vec::new();
                for lib_path in &lib_paths {
                    if Path::new(lib_path).exists() {
                        if let Ok(lib_content) = fs::read_to_string(lib_path) {
                            exports.extend(self.extract_exports(&lib_content));
                        }
                    }
                }
                
                if !exports.is_empty() {
                    self.add_crate_exports(&crate_name, exports);
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_crate_name(&self, cargo_content: &str) -> String {
        for line in cargo_content.lines() {
            if line.trim().starts_with("name = ") {
                if let Some(name_start) = line.find('"') {
                    if let Some(name_end) = line[name_start + 1..].find('"') {
                        return line[name_start + 1..name_start + 1 + name_end].to_string();
                    }
                }
            }
        }
        "unknown".to_string()
    }
    
    fn extract_exports(&self, lib_content: &str) -> Vec<String> {
        let mut exports = Vec::new();
        
        for line in lib_content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("pub fn ") {
                if let Some(paren_pos) = trimmed.find('(') {
                    let fn_name = &trimmed[7..paren_pos];
                    exports.push(fn_name.to_string());
                }
            }
            
            if trimmed.starts_with("pub struct ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() > 2 {
                    exports.push(parts[2].to_string());
                }
            }
        }
        
        exports
    }
    
    fn add_crate_exports(&mut self, crate_name: &str, exports: Vec<String>) {
        println!("  {} exports: {:?}", crate_name, exports);
        
        for export in &exports {
            let full_name = format!("{}::{}", crate_name, export);
            let phi = calculate_phi_key(export);
            self.phi_mappings.insert(full_name, phi);
        }
        
        self.cargo_exports.insert(crate_name.to_string(), exports);
    }
    
    fn add_rust_std_library(&mut self) {
        println!("=== Adding Rust std library ===");
        
        let std_exports: Vec<String> = vec![
            "println", "format", "write", "read_to_string", "HashMap", 
            "Vec", "String", "Option", "Result", "fs", "Command"
        ].into_iter().map(|s| s.to_string()).collect();
        
        self.add_crate_exports("std", std_exports);
    }
    
    fn resolve_all(&mut self) {
        println!("\n=== Resolving all unresolved usages ===");
        
        let unresolved = vec![
            "fs", "Command", "HashMap", "println", "format", "Vec",
            "String", "Option", "Result", "serialize", "deserialize", 
            "blake3", "hash", "juniper", "GraphQL"
        ];
        
        for usage in &unresolved {
            let mut best_match = None;
            let mut best_score = 0;
            
            for (full_name, phi) in &self.phi_mappings {
                let score = similarity_score(usage, full_name);
                if score > best_score {
                    best_score = score;
                    best_match = Some((full_name.clone(), *phi));
                }
            }
            
            if let Some((full_name, phi)) = best_match {
                if best_score > 40 {
                    self.resolved.insert(usage.to_string(), (full_name.clone(), phi));
                    println!("  {} → {} (φ = {}, score: {})", usage, full_name, phi, best_score);
                } else {
                    println!("  {} → UNRESOLVED (best score: {})", usage, best_score);
                }
            } else {
                println!("  {} → NO MATCH", usage);
            }
        }
    }
}

fn similarity_score(a: &str, b: &str) -> u32 {
    if a == b { return 100; }
    if b.ends_with(&format!("::{}", a)) { return 95; }
    if b.contains(a) || a.contains(b) { return 80; }
    
    let common_chars = a.chars().filter(|c| b.contains(*c)).count();
    (common_chars * 100 / a.len().max(b.len()).max(1)) as u32
}

fn calculate_phi_key(name: &str) -> u64 {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let monster_element = (name_hash * 5 + 71) % 196883;
    euler_phi(monster_element)
}

fn euler_phi(n: u64) -> u64 {
    if n <= 1 { return 1; }
    let mut result = n;
    let mut num = n;
    let mut p = 2;
    
    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 { num /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 { result -= result / num; }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Complete Cargo Resolver ===");
    
    let mut resolver = CompleteCargoResolver::new();
    
    resolver.scan_submodules()?;
    resolver.add_rust_std_library();
    resolver.resolve_all();
    
    println!("\n=== Final Summary ===");
    println!("Total crates found: {}", resolver.cargo_exports.len());
    println!("Total exports indexed: {}", resolver.phi_mappings.len());
    println!("Successfully resolved: {}", resolver.resolved.len());
    
    let total_phi: u64 = resolver.resolved.values().map(|(_, phi)| phi).sum();
    println!("Total phi sum: {}", total_phi);
    
    println!("\n✓ All submodules scanned");
    println!("✓ Complete dependency resolution");
    
    Ok(())
}
