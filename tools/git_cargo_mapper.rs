use std::fs;
use std::collections::HashMap;

struct GitCargoMapper {
    git_to_cargo: HashMap<String, Vec<String>>, // git_repo -> [cargo_crates]
}

impl GitCargoMapper {
    fn new() -> Self {
        Self {
            git_to_cargo: HashMap::new(),
        }
    }
    
    fn map_git_to_cargo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Mapping git modules to cargo crates...");
        
        // Scan for Cargo.toml files and map to git repos
        let cargo_files = [
            "Cargo.toml",
            "monster_protocol/Cargo.toml", 
            "rust-bootstrap-nix/Cargo.toml",
            "cargo-llm-bootstrap/Cargo.toml"
        ];
        
        for cargo_file in &cargo_files {
            if let Ok(content) = fs::read_to_string(cargo_file) {
                if let Some(crate_name) = self.extract_crate_name(&content) {
                    let git_repo = self.determine_git_repo(cargo_file);
                    self.git_to_cargo.entry(git_repo).or_insert_with(Vec::new).push(crate_name);
                }
            }
        }
        
        println!("  ✓ Mapped {} git repos to cargo crates", self.git_to_cargo.len());
        Ok(())
    }
    
    fn extract_crate_name(&self, content: &str) -> Option<String> {
        for line in content.lines() {
            if line.trim().starts_with("name = ") {
                return line.split('"').nth(1).map(|s| s.to_string());
            }
        }
        None
    }
    
    fn determine_git_repo(&self, cargo_path: &str) -> String {
        if cargo_path.contains("monster_protocol") {
            "https://github.com/meta-introspector/cargo2nix/monster_protocol".to_string()
        } else if cargo_path.contains("rust-bootstrap-nix") {
            "https://github.com/meta-introspector/cargo2nix/rust-bootstrap-nix".to_string()
        } else if cargo_path.contains("cargo-llm-bootstrap") {
            "https://github.com/meta-introspector/cargo2nix/cargo-llm-bootstrap".to_string()
        } else {
            "https://github.com/meta-introspector/cargo2nix".to_string()
        }
    }
    
    fn demonstrate_relationships(&self) {
        println!("\n🔗 === GIT MODULE → CARGO MODULE MAPPING ===");
        
        println!("\n📦 git_module - defined_in_repo_of -> cargo_module:");
        for (git_repo, cargo_crates) in &self.git_to_cargo {
            let repo_name = git_repo.split('/').last().unwrap_or("unknown");
            println!("  Git Repo: {}", repo_name);
            for crate_name in cargo_crates {
                println!("    {} - defined_in_repo_of -> {}", repo_name, crate_name);
            }
            println!();
        }
        
        println!("📊 MAPPING STATISTICS:");
        let total_crates: usize = self.git_to_cargo.values().map(|v| v.len()).sum();
        println!("  Git repositories: {}", self.git_to_cargo.len());
        println!("  Cargo crates: {}", total_crates);
        println!("  Average crates per repo: {:.1}", total_crates as f64 / self.git_to_cargo.len() as f64);
        
        println!("\n🎯 RELATIONSHIP BENEFITS:");
        println!("  ✓ Map git repositories to their cargo crates");
        println!("  ✓ Track which repo defines which crate");
        println!("  ✓ Enable repo-to-crate dependency analysis");
        println!("  ✓ Support Monster Protocol crate mapping");
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.map_git_to_cargo()?;
        self.demonstrate_relationships();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = GitCargoMapper::new();
    mapper.run()
}
