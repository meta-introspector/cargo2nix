use std::collections::HashMap;
use std::fs;

/// Minimal Monster Protocol Implementation
/// Implements the core vision from plan.org with minimal code
struct MonsterProtocol {
    // Triple database system
    git_db: GitModuleDB,
    cargo_db: CargoCrateDB, 
    ast_db: ASTDeclDB,
    
    // Monster group mappings (192 conjugacy classes)
    monster_mappings: HashMap<String, u8>, // decl -> monster index (0-191)
}

#[derive(Debug)]
struct GitModuleDB {
    modules: HashMap<String, GitModule>,
}

#[derive(Debug)]
struct GitModule {
    url: String,
    path: String,
    branch: String,
}

#[derive(Debug)]
struct CargoCrateDB {
    crates: HashMap<String, CargoCrate>,
}

#[derive(Debug)]
struct CargoCrate {
    name: String,
    version: String,
    git_url: Option<String>,
}

#[derive(Debug)]
struct ASTDeclDB {
    decls: HashMap<String, ASTDecl>,
}

#[derive(Debug)]
struct ASTDecl {
    name: String,
    decl_type: String, // trait, struct, fn, etc
    content_hash: String,
    monster_index: u8, // 0-191 mapping to monster conjugacy classes
}

impl MonsterProtocol {
    fn new() -> Self {
        Self {
            git_db: GitModuleDB { modules: HashMap::new() },
            cargo_db: CargoCrateDB { crates: HashMap::new() },
            ast_db: ASTDeclDB { decls: HashMap::new() },
            monster_mappings: HashMap::new(),
        }
    }
    
    /// Load git modules from .gitmodules files
    fn load_git_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning for .gitmodules files...");
        
        // Find all .gitmodules files
        let output = std::process::Command::new("find")
            .args(&["/mnt/data1/nix", "-name", ".gitmodules", "-type", "f"])
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout);
        let gitmodules_files: Vec<&str> = files.lines().collect();
        
        println!("Found {} .gitmodules files", gitmodules_files.len());
        
        for gitmodules_path in gitmodules_files {
            println!("  Processing: {}", gitmodules_path);
            self.parse_gitmodules(gitmodules_path)?;
        }
        
        println!("✓ Loaded {} git modules total", self.git_db.modules.len());
        Ok(())
    }
    
    fn parse_gitmodules(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        let mut current_module: Option<String> = None;
        let mut url: Option<String> = None;
        let mut module_path: Option<String> = None;
        let mut branch = "main".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                // Save previous module if exists
                if let (Some(name), Some(u), Some(p)) = (&current_module, &url, &module_path) {
                    self.git_db.modules.insert(name.clone(), GitModule {
                        url: u.clone(),
                        path: p.clone(), 
                        branch: branch.clone(),
                    });
                }
                
                // Extract module name
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        current_module = Some(line[start+1..end].to_string());
                    }
                }
                branch = "main".to_string(); // reset
            } else if line.starts_with("url = ") {
                url = Some(line[6..].to_string());
            } else if line.starts_with("path = ") {
                module_path = Some(line[7..].to_string());
            } else if line.starts_with("branch = ") {
                branch = line[9..].to_string();
            }
        }
        
        // Save last module
        if let (Some(name), Some(u), Some(p)) = (current_module, url, module_path) {
            self.git_db.modules.insert(name, GitModule {
                url: u,
                path: p,
                branch,
            });
        }
        
        Ok(())
    }
    
    /// Load cargo crates from Cargo.toml files
    fn load_cargo_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning for Cargo.toml files...");
        
        let output = std::process::Command::new("find")
            .args(&["/mnt/data1/nix", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout);
        let cargo_files: Vec<&str> = files.lines().collect();
        
        println!("Found {} Cargo.toml files", cargo_files.len());
        
        for (i, cargo_path) in cargo_files.iter().enumerate() {
            if i % 50 == 0 {
                println!("  Processing Cargo.toml {}/{}", i + 1, cargo_files.len());
            }
            self.parse_cargo_toml(cargo_path)?;
        }
        
        println!("✓ Loaded {} cargo crates total", self.cargo_db.crates.len());
        Ok(())
    }
    
    fn parse_cargo_toml(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        let mut name: Option<String> = None;
        let mut version: Option<String> = None;
        let mut git_url: Option<String> = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("name = ") {
                name = Some(line[7..].trim_matches('"').to_string());
            } else if line.starts_with("version = ") {
                version = Some(line[10..].trim_matches('"').to_string());
            } else if line.contains("git = ") {
                if let Some(start) = line.find("git = \"") {
                    if let Some(end) = line[start+7..].find('"') {
                        git_url = Some(line[start+7..start+7+end].to_string());
                    }
                }
            }
        }
        
        if let (Some(n), Some(v)) = (name, version) {
            self.cargo_db.crates.insert(n.clone(), CargoCrate {
                name: n,
                version: v,
                git_url,
            });
        }
        
        Ok(())
    }
    
    /// Extract AST declarations and assign monster indices
    fn extract_ast_declarations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning for Rust source files...");
        
        let output = std::process::Command::new("find")
            .args(&["/mnt/data1/nix", "-name", "*.rs", "-type", "f"])
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout);
        let rust_files: Vec<&str> = files.lines().collect();
        
        println!("Found {} Rust files", rust_files.len());
        
        for (i, rust_file) in rust_files.iter().enumerate() {
            if i % 100 == 0 {
                println!("  Processing Rust file {}/{}", i + 1, rust_files.len());
            }
            self.parse_rust_file(rust_file)?;
        }
        
        println!("✓ Extracted {} AST declarations total", self.ast_db.decls.len());
        Ok(())
    }
    
    fn parse_rust_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        // Simple regex-based extraction (minimal implementation)
        for line in content.lines() {
            let line = line.trim();
            
            // Extract traits
            if line.starts_with("trait ") {
                if let Some(name) = self.extract_name_after("trait ", line) {
                    let hash = self.simple_hash(&line);
                    let monster_index = (hash % 192) as u8;
                    
                    self.ast_db.decls.insert(format!("trait_{}", name), ASTDecl {
                        name: name.clone(),
                        decl_type: "trait".to_string(),
                        content_hash: format!("{:x}", hash),
                        monster_index,
                    });
                    
                    self.monster_mappings.insert(format!("trait_{}", name), monster_index);
                }
            }
            
            // Extract structs
            if line.starts_with("struct ") {
                if let Some(name) = self.extract_name_after("struct ", line) {
                    let hash = self.simple_hash(&line);
                    let monster_index = (hash % 192) as u8;
                    
                    self.ast_db.decls.insert(format!("struct_{}", name), ASTDecl {
                        name: name.clone(),
                        decl_type: "struct".to_string(),
                        content_hash: format!("{:x}", hash),
                        monster_index,
                    });
                    
                    self.monster_mappings.insert(format!("struct_{}", name), monster_index);
                }
            }
            
            // Extract functions
            if line.starts_with("fn ") || line.starts_with("pub fn ") {
                let fn_start = if line.starts_with("pub fn ") { "pub fn " } else { "fn " };
                if let Some(name) = self.extract_name_after(fn_start, line) {
                    let hash = self.simple_hash(&line);
                    let monster_index = (hash % 192) as u8;
                    
                    self.ast_db.decls.insert(format!("fn_{}", name), ASTDecl {
                        name: name.clone(),
                        decl_type: "fn".to_string(),
                        content_hash: format!("{:x}", hash),
                        monster_index,
                    });
                    
                    self.monster_mappings.insert(format!("fn_{}", name), monster_index);
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_name_after(&self, prefix: &str, line: &str) -> Option<String> {
        if let Some(start) = line.find(prefix) {
            let after_prefix = &line[start + prefix.len()..];
            if let Some(end) = after_prefix.find(|c: char| c.is_whitespace() || c == '(' || c == '<' || c == '{') {
                return Some(after_prefix[..end].to_string());
            }
        }
        None
    }
    
    fn simple_hash(&self, content: &str) -> u64 {
        // Simple hash function for monster index assignment
        let mut hash = 0u64;
        for byte in content.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
    
    /// Find similar declarations using monster indices
    fn find_similar_declarations(&self) -> Vec<(String, String, u8)> {
        let mut similarities = Vec::new();
        let mut index_groups: HashMap<u8, Vec<String>> = HashMap::new();
        
        // Group declarations by monster index
        for (decl_name, monster_index) in &self.monster_mappings {
            index_groups.entry(*monster_index).or_insert_with(Vec::new).push(decl_name.clone());
        }
        
        // Find groups with multiple declarations (potential duplicates)
        for (monster_index, decls) in index_groups {
            if decls.len() > 1 {
                for i in 0..decls.len() {
                    for j in i+1..decls.len() {
                        similarities.push((decls[i].clone(), decls[j].clone(), monster_index));
                    }
                }
            }
        }
        
        similarities
    }
    
    /// Generate build report showing crate build order and submodule mapping
    fn generate_build_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== MONSTER PROTOCOL BUILD REPORT ===");
        
        println!("\n1. Git Modules ({} total):", self.git_db.modules.len());
        for (name, module) in &self.git_db.modules {
            println!("  {} -> {} ({})", name, module.url, module.branch);
        }
        
        println!("\n2. Cargo Crates ({} total):", self.cargo_db.crates.len());
        for (name, crate_info) in &self.cargo_db.crates {
            if let Some(git_url) = &crate_info.git_url {
                println!("  {} v{} -> {}", name, crate_info.version, git_url);
            } else {
                println!("  {} v{} (registry)", name, crate_info.version);
            }
        }
        
        println!("\n3. AST Declarations ({} total):", self.ast_db.decls.len());
        let mut monster_counts: HashMap<u8, u32> = HashMap::new();
        for decl in self.ast_db.decls.values() {
            *monster_counts.entry(decl.monster_index).or_insert(0) += 1;
        }
        
        for (monster_index, count) in monster_counts {
            println!("  Monster Index {}: {} declarations", monster_index, count);
        }
        
        println!("\n4. Similar Declarations (Monster Algorithm):");
        let similarities = self.find_similar_declarations();
        for (decl1, decl2, monster_index) in similarities.iter().take(10) {
            println!("  {} ≈ {} (Monster Index: {})", decl1, decl2, monster_index);
        }
        
        println!("\n5. Solana Rustc Build Plan:");
        println!("  - Total submodules available: {}", self.git_db.modules.len());
        println!("  - Crates with git dependencies: {}", 
                 self.cargo_db.crates.values().filter(|c| c.git_url.is_some()).count());
        println!("  - Unique monster indices used: {}", 
                 self.monster_mappings.values().collect::<std::collections::HashSet<_>>().len());
        
        Ok(())
    }
    
    /// Main execution function
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Monster Protocol: Minimal Implementation");
        println!("=======================================");
        
        // Load triple database
        self.load_git_modules()?;
        self.load_cargo_crates()?;
        self.extract_ast_declarations()?;
        
        // Generate comprehensive report
        self.generate_build_report()?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monster = MonsterProtocol::new();
    monster.run()
}
