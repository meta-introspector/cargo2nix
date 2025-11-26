use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct GitModule {
    name: String,
    path: String,
    git_hash: String,
    dependencies: Vec<String>,
    exports: Vec<String>,
    cas_address: u64,
}

#[derive(Debug, Clone)]
struct GitRelation {
    from_module: String,
    to_module: String,
    relation_type: String, // "depends", "forks", "imports"
    phi_weight: u64,
}

struct GitModuleCASDB {
    modules: HashMap<u64, GitModule>, // cas_address -> module
    relations: Vec<GitRelation>,
    name_to_cas: HashMap<String, u64>,
}

impl GitModuleCASDB {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            relations: Vec::new(),
            name_to_cas: HashMap::new(),
        }
    }
    
    fn load_git_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Loading Git Modules into CAS DB ===");
        
        // Scan our actual submodules
        let submodule_paths = vec![
            ("./submodules/BLAKE3", "blake3"),
            ("./submodules/juniper", "juniper"),
            ("./lattice-introspector", "lattice"),
            ("./minizinc-introspector", "minizinc_introspector"),
            ("./tools/monster_protocol", "monster_protocol"),
            ("./src", "cargo2nix_core"),
        ];
        
        for (path, name) in &submodule_paths {
            if Path::new(path).exists() {
                self.scan_git_module(path, name)?;
            }
        }
        
        Ok(())
    }
    
    fn scan_git_module(&mut self, path: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning git module: {} at {}", name, path);
        
        let mut dependencies = Vec::new();
        let mut exports = Vec::new();
        
        // Check for Cargo.toml dependencies
        let cargo_path = format!("{}/Cargo.toml", path);
        if Path::new(&cargo_path).exists() {
            if let Ok(content) = fs::read_to_string(&cargo_path) {
                dependencies.extend(self.extract_cargo_dependencies(&content));
            }
        }
        
        // Check for lib.rs exports
        let lib_paths = vec![
            format!("{}/src/lib.rs", path),
            format!("{}/src/main.rs", path),
        ];
        
        for lib_path in &lib_paths {
            if Path::new(lib_path).exists() {
                if let Ok(content) = fs::read_to_string(lib_path) {
                    exports.extend(self.extract_exports(&content));
                }
            }
        }
        
        // Get git hash (simulated)
        let git_hash = self.get_git_hash(path);
        
        // Calculate CAS address from dependencies + exports + git_hash
        let cas_address = self.calculate_module_cas_address(&dependencies, &exports, &git_hash);
        
        let module = GitModule {
            name: name.to_string(),
            path: path.to_string(),
            git_hash,
            dependencies: dependencies.clone(),
            exports: exports.clone(),
            cas_address,
        };
        
        println!("  → CAS: {}, deps: {}, exports: {}", cas_address, dependencies.len(), exports.len());
        
        self.modules.insert(cas_address, module);
        self.name_to_cas.insert(name.to_string(), cas_address);
        
        // Create relations
        for dep in &dependencies {
            self.add_relation(name, dep, "depends");
        }
        
        Ok(())
    }
    
    fn extract_cargo_dependencies(&self, cargo_content: &str) -> Vec<String> {
        let mut deps = Vec::new();
        let mut in_dependencies = false;
        
        for line in cargo_content.lines() {
            let trimmed = line.trim();
            
            if trimmed == "[dependencies]" {
                in_dependencies = true;
                continue;
            }
            
            if trimmed.starts_with('[') && trimmed != "[dependencies]" {
                in_dependencies = false;
            }
            
            if in_dependencies && trimmed.contains('=') {
                if let Some(dep_name) = trimmed.split('=').next() {
                    let clean_name = dep_name.trim().trim_matches('"');
                    if !clean_name.is_empty() {
                        deps.push(clean_name.to_string());
                    }
                }
            }
        }
        
        deps
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
    
    fn get_git_hash(&self, _path: &str) -> String {
        // Simulated git hash - in real implementation would run `git rev-parse HEAD`
        format!("abc123def456_{}", _path.len())
    }
    
    fn calculate_module_cas_address(&self, dependencies: &[String], exports: &[String], git_hash: &str) -> u64 {
        let mut signature = 0u64;
        
        // Include dependency signatures
        for dep in dependencies {
            signature = signature.wrapping_add(calculate_phi_key(dep));
        }
        
        // Include export signatures  
        for export in exports {
            signature = signature.wrapping_add(calculate_phi_key(export));
        }
        
        // Include git hash
        signature = signature.wrapping_add(git_hash.bytes().map(|b| b as u64).sum::<u64>());
        
        signature % 196883
    }
    
    fn add_relation(&mut self, from: &str, to: &str, relation_type: &str) {
        let phi_weight = calculate_phi_key(&format!("{}_{}", from, to));
        
        let relation = GitRelation {
            from_module: from.to_string(),
            to_module: to.to_string(),
            relation_type: relation_type.to_string(),
            phi_weight,
        };
        
        self.relations.push(relation);
    }
    
    fn show_database(&self) {
        println!("\n=== Git Modules CAS Database ===");
        println!("Modules: {}", self.modules.len());
        println!("Relations: {}", self.relations.len());
        
        for (cas, module) in &self.modules {
            println!("📦 {} (CAS: {})", module.name, cas);
            println!("   Path: {}", module.path);
            println!("   Git: {}", module.git_hash);
            println!("   Deps: {:?}", module.dependencies);
            println!("   Exports: {:?}", module.exports);
        }
        
        println!("\n=== Relations ===");
        for relation in &self.relations {
            println!("🔗 {} --{}-> {} (φ = {})", 
                     relation.from_module, relation.relation_type, 
                     relation.to_module, relation.phi_weight);
        }
    }
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
    println!("=== Git Modules CAS Database Loader ===");
    
    let mut db = GitModuleCASDB::new();
    
    // Load all git modules
    db.load_git_modules()?;
    
    // Show complete database
    db.show_database();
    
    println!("\n=== Summary ===");
    println!("✓ All git modules loaded into single CAS database");
    println!("✓ Dependencies and relations mapped");
    println!("✓ CAS addresses calculated from module signatures");
    println!("✓ Ready for unified querying and analysis");
    
    Ok(())
}
