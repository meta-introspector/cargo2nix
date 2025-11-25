use std::fs;
use std::collections::HashMap;

struct FastMonsterAnalyzer {
    git_modules: HashMap<String, String>,
    cargo_crates: HashMap<String, String>, 
    ast_decls: HashMap<String, u8>, // decl -> monster index
}

impl FastMonsterAnalyzer {
    fn new() -> Self {
        Self {
            git_modules: HashMap::new(),
            cargo_crates: HashMap::new(),
            ast_decls: HashMap::new(),
        }
    }
    
    fn load_from_inventory(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading from existing inventory files...");
        
        // Use git_files_inventory2.txt from parent directory
        if let Ok(content) = fs::read_to_string("../git_files_inventory2.txt") {
            println!("Processing git inventory...");
            for line in content.lines().take(100) { // Limit for speed
                if line.contains(".gitmodules") {
                    self.git_modules.insert(line.to_string(), "submodule".to_string());
                }
            }
        }
        
        // Process a few key Cargo.toml files that we know exist
        let cargo_files = [
            "Cargo.toml",
            "rust-bootstrap-nix/Cargo.toml", 
            "monster_protocol/Cargo.toml"
        ];
        
        for cargo_file in &cargo_files {
            if let Ok(content) = fs::read_to_string(cargo_file) {
                if let Some(name) = self.extract_crate_name(&content) {
                    self.cargo_crates.insert(name, cargo_file.to_string());
                }
            }
        }
        
        // Process key Rust files for AST that we know exist
        let rust_files = [
            "simple_monster_traits.rs",
            "monster_triple_db_loader.rs", 
            "solana_submodule_driver.rs",
            "fast_monster_analyzer.rs"
        ];
        
        for rust_file in &rust_files {
            if let Ok(content) = fs::read_to_string(rust_file) {
                self.extract_decls(&content, rust_file);
            }
        }
        
        Ok(())
    }
    
    fn extract_crate_name(&self, content: &str) -> Option<String> {
        for line in content.lines() {
            if line.trim().starts_with("name = ") {
                return Some(line.split('"').nth(1)?.to_string());
            }
        }
        None
    }
    
    fn extract_decls(&mut self, content: &str, file: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("struct ") || line.starts_with("trait ") || line.starts_with("fn ") {
                let hash = self.simple_hash(line);
                let monster_index = (hash % 192) as u8;
                self.ast_decls.insert(format!("{}:{}", file, line), monster_index);
            }
        }
    }
    
    fn simple_hash(&self, content: &str) -> u64 {
        content.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
    }
    
    fn generate_report(&self) {
        println!("\n=== FAST MONSTER ANALYSIS REPORT ===");
        println!("Git modules found: {}", self.git_modules.len());
        println!("Cargo crates found: {}", self.cargo_crates.len()); 
        println!("AST declarations found: {}", self.ast_decls.len());
        
        // Show monster index distribution
        let mut monster_counts: HashMap<u8, u32> = HashMap::new();
        for &index in self.ast_decls.values() {
            *monster_counts.entry(index).or_insert(0) += 1;
        }
        
        println!("\nMonster Index Distribution:");
        for (index, count) in monster_counts.iter().take(10) {
            println!("  Index {}: {} declarations", index, count);
        }
        
        println!("\nSample declarations:");
        for (decl, index) in self.ast_decls.iter().take(5) {
            println!("  {} -> Monster Index {}", decl, index);
        }
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.load_from_inventory()?;
        self.generate_report();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = FastMonsterAnalyzer::new();
    analyzer.run()
}
