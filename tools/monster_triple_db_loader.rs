use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ContentID(String);

impl ContentID {
    fn new(content: &str) -> Self {
        Self(format!("{:x}", simple_hash(content)))
    }
}

// DB 1: Git Modules and Relations
#[derive(Debug)]
struct GitModuleDB {
    modules: HashMap<ContentID, GitModule>,
    relations: HashMap<ContentID, Vec<ContentID>>,
}

#[derive(Debug, Clone)]
struct GitModule {
    id: ContentID,
    name: String,
    url: String,
    path: String,
    branch: String,
}

// DB 2: Cargo Crates
#[derive(Debug)]
struct CargoCrateDB {
    crates: HashMap<ContentID, CargoCrate>,
    dependencies: HashMap<ContentID, Vec<ContentID>>,
}

#[derive(Debug, Clone)]
struct CargoCrate {
    id: ContentID,
    name: String,
    version: String,
    path: String,
    git_module_id: Option<ContentID>,
}

// DB 3: AST Declarations
#[derive(Debug)]
struct ASTDeclDB {
    decls: HashMap<ContentID, ASTDecl>,
    similarities: HashMap<ContentID, Vec<(ContentID, f64)>>,
}

#[derive(Debug, Clone)]
struct ASTDecl {
    id: ContentID,
    decl_type: String, // fn, struct, impl, etc
    name: String,
    signature: String,
    crate_id: ContentID,
    file_path: String,
}

struct MonsterTripleDB {
    git_db: GitModuleDB,
    cargo_db: CargoCrateDB,
    ast_db: ASTDeclDB,
}

impl MonsterTripleDB {
    fn new() -> Self {
        Self {
            git_db: GitModuleDB {
                modules: HashMap::new(),
                relations: HashMap::new(),
            },
            cargo_db: CargoCrateDB {
                crates: HashMap::new(),
                dependencies: HashMap::new(),
            },
            ast_db: ASTDeclDB {
                decls: HashMap::new(),
                similarities: HashMap::new(),
            },
        }
    }
    
    fn load_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading Monster Triple DB...");
        
        // 1. Load git modules
        self.load_git_modules()?;
        
        // 2. Load cargo crates
        self.load_cargo_crates()?;
        
        // 3. Load AST declarations
        self.load_ast_declarations()?;
        
        // 4. Calculate similarities
        self.calculate_similarities()?;
        
        Ok(())
    }
    
    fn load_git_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading git modules into DB1...");
        
        // Read .gitmodules
        if let Ok(content) = fs::read_to_string(".gitmodules") {
            let mut current_module: Option<GitModule> = None;
            
            for line in content.lines() {
                let line = line.trim();
                
                if line.starts_with("[submodule \"") {
                    if let Some(module) = current_module.take() {
                        self.git_db.modules.insert(module.id.clone(), module);
                    }
                    
                    let name = &line[12..line.len()-2];
                    let id = ContentID::new(&format!("git_module_{}", name));
                    
                    current_module = Some(GitModule {
                        id,
                        name: name.to_string(),
                        url: String::new(),
                        path: String::new(),
                        branch: "main".to_string(),
                    });
                } else if let Some(ref mut module) = current_module {
                    if line.starts_with("url = ") {
                        module.url = line[6..].to_string();
                    } else if line.starts_with("path = ") {
                        module.path = line[7..].to_string();
                    } else if line.starts_with("branch = ") {
                        module.branch = line[9..].to_string();
                    }
                }
            }
            
            if let Some(module) = current_module {
                self.git_db.modules.insert(module.id.clone(), module);
            }
        }
        
        println!("Loaded {} git modules", self.git_db.modules.len());
        Ok(())
    }
    
    fn load_cargo_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading cargo crates into DB2...");
        
        // Scan submodules for Cargo.toml files
        if let Ok(entries) = fs::read_dir("submodules") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let cargo_toml = path.join("Cargo.toml");
                        if cargo_toml.exists() {
                            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                                let crate_name = extract_crate_name(&content)
                                    .unwrap_or_else(|| path.file_name().unwrap().to_string_lossy().to_string());
                                
                                let id = ContentID::new(&format!("cargo_crate_{}", crate_name));
                                
                                // Find corresponding git module
                                let git_module_id = self.find_git_module_for_path(&path.to_string_lossy());
                                
                                let crate_obj = CargoCrate {
                                    id: id.clone(),
                                    name: crate_name,
                                    version: "0.1.0".to_string(),
                                    path: path.to_string_lossy().to_string(),
                                    git_module_id,
                                };
                                
                                self.cargo_db.crates.insert(id, crate_obj);
                            }
                        }
                    }
                }
            }
        }
        
        println!("Loaded {} cargo crates", self.cargo_db.crates.len());
        Ok(())
    }
    
    fn load_ast_declarations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading AST declarations into DB3...");
        
        // Find all Rust files and extract declarations
        let output = std::process::Command::new("find")
            .args(&["submodules", "-name", "*.rs", "-type", "f"])
            .output()?;
        
        let files = String::from_utf8_lossy(&output.stdout);
        
        for file_path in files.lines() {
            if let Ok(content) = fs::read_to_string(file_path) {
                let decls = extract_declarations(&content, file_path);
                
                for decl in decls {
                    // Find corresponding crate
                    let crate_id = self.find_crate_for_file(file_path);
                    
                    let mut ast_decl = decl;
                    ast_decl.crate_id = crate_id.unwrap_or_else(|| ContentID::new("unknown"));
                    
                    self.ast_db.decls.insert(ast_decl.id.clone(), ast_decl);
                }
            }
        }
        
        println!("Loaded {} AST declarations", self.ast_db.decls.len());
        Ok(())
    }
    
    fn calculate_similarities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Calculating Monster similarities...");
        
        let decl_ids: Vec<ContentID> = self.ast_db.decls.keys().cloned().collect();
        
        for i in 0..decl_ids.len() {
            let mut similarities = Vec::new();
            
            for j in (i+1)..decl_ids.len() {
                let decl1 = &self.ast_db.decls[&decl_ids[i]];
                let decl2 = &self.ast_db.decls[&decl_ids[j]];
                
                let similarity = monster_similarity(&decl1.signature, &decl2.signature);
                
                if similarity > 0.8 {
                    similarities.push((decl_ids[j].clone(), similarity));
                }
            }
            
            if !similarities.is_empty() {
                self.ast_db.similarities.insert(decl_ids[i].clone(), similarities);
            }
        }
        
        println!("Found {} declarations with high similarities", self.ast_db.similarities.len());
        Ok(())
    }
    
    fn find_git_module_for_path(&self, path: &str) -> Option<ContentID> {
        for module in self.git_db.modules.values() {
            if path.contains(&module.path) {
                return Some(module.id.clone());
            }
        }
        None
    }
    
    fn find_crate_for_file(&self, file_path: &str) -> Option<ContentID> {
        for crate_obj in self.cargo_db.crates.values() {
            if file_path.starts_with(&crate_obj.path) {
                return Some(crate_obj.id.clone());
            }
        }
        None
    }
    
    fn generate_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::from("# Monster Triple DB Analysis\n\n");
        
        report.push_str(&format!("## Database Summary\n"));
        report.push_str(&format!("- **Git Modules (DB1)**: {}\n", self.git_db.modules.len()));
        report.push_str(&format!("- **Cargo Crates (DB2)**: {}\n", self.cargo_db.crates.len()));
        report.push_str(&format!("- **AST Declarations (DB3)**: {}\n", self.ast_db.decls.len()));
        report.push_str(&format!("- **Similar Declarations**: {}\n\n", self.ast_db.similarities.len()));
        
        // Show linkages
        report.push_str("## Linkages\n");
        for (crate_id, crate_obj) in &self.cargo_db.crates {
            if let Some(git_id) = &crate_obj.git_module_id {
                if let Some(git_module) = self.git_db.modules.get(git_id) {
                    report.push_str(&format!("- **{}** → **{}**\n", crate_obj.name, git_module.name));
                }
            }
        }
        
        fs::write("monster_triple_db_report.md", report)?;
        println!("Report saved to monster_triple_db_report.md");
        
        Ok(())
    }
}

fn extract_crate_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("name = \"") {
            if let Some(end) = line[8..].find("\"") {
                return Some(line[8..8+end].to_string());
            }
        }
    }
    None
}

fn extract_declarations(content: &str, file_path: &str) -> Vec<ASTDecl> {
    let mut decls = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        
        if line.starts_with("fn ") || line.starts_with("pub fn ") {
            let name = extract_fn_name(line).unwrap_or_else(|| format!("fn_{}", line_num));
            let id = ContentID::new(&format!("{}:{}:{}", file_path, line_num, name));
            
            decls.push(ASTDecl {
                id,
                decl_type: "fn".to_string(),
                name,
                signature: line.to_string(),
                crate_id: ContentID::new("unknown"), // Will be set later
                file_path: file_path.to_string(),
            });
        } else if line.starts_with("struct ") || line.starts_with("pub struct ") {
            let name = extract_struct_name(line).unwrap_or_else(|| format!("struct_{}", line_num));
            let id = ContentID::new(&format!("{}:{}:{}", file_path, line_num, name));
            
            decls.push(ASTDecl {
                id,
                decl_type: "struct".to_string(),
                name,
                signature: line.to_string(),
                crate_id: ContentID::new("unknown"),
                file_path: file_path.to_string(),
            });
        }
    }
    
    decls
}

fn extract_fn_name(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for i in 0..parts.len() {
        if parts[i] == "fn" && i + 1 < parts.len() {
            let name = parts[i + 1];
            if let Some(paren_pos) = name.find('(') {
                return Some(name[..paren_pos].to_string());
            }
            return Some(name.to_string());
        }
    }
    None
}

fn extract_struct_name(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for i in 0..parts.len() {
        if parts[i] == "struct" && i + 1 < parts.len() {
            let name = parts[i + 1];
            if let Some(brace_pos) = name.find('{') {
                return Some(name[..brace_pos].to_string());
            }
            return Some(name.to_string());
        }
    }
    None
}

fn monster_similarity(sig1: &str, sig2: &str) -> f64 {
    // Simple token-based similarity
    let tokens1: std::collections::HashSet<&str> = sig1.split_whitespace().collect();
    let tokens2: std::collections::HashSet<&str> = sig2.split_whitespace().collect();
    
    let intersection = tokens1.intersection(&tokens2).count();
    let union = tokens1.union(&tokens2).count();
    
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

fn simple_hash(content: &str) -> u64 {
    let mut hash = 0u64;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = MonsterTripleDB::new();
    
    println!("Monster Triple Database Loader");
    println!("=============================");
    
    db.load_all()?;
    db.generate_report()?;
    
    println!("Monster Triple DB loaded successfully!");
    
    Ok(())
}
