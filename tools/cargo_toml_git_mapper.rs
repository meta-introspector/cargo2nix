use std::collections::HashMap;

struct CargoTomlGitMapper {
    toml_in_git: HashMap<String, Vec<String>>, // git_module -> [cargo_toml_files]
}

impl CargoTomlGitMapper {
    fn new() -> Self {
        Self {
            toml_in_git: HashMap::new(),
        }
    }
    
    fn map_toml_in_git_relationships(&mut self) {
        println!("📄 Mapping Cargo.toml files in git modules...");
        
        // cargo_toml - in_file_in - git_module
        self.toml_in_git.insert(
            "https://github.com/rust-lang/rust".to_string(),
            vec!["Cargo.toml".to_string(), "compiler/rustc_driver/Cargo.toml".to_string(), "compiler/rustc_interface/Cargo.toml".to_string()]
        );
        
        self.toml_in_git.insert(
            "https://github.com/meta-introspector/cargo2nix".to_string(),
            vec!["Cargo.toml".to_string(), "tools/monster_protocol/Cargo.toml".to_string()]
        );
        
        self.toml_in_git.insert(
            "https://github.com/solana-labs/solana".to_string(),
            vec!["Cargo.toml".to_string(), "program/Cargo.toml".to_string(), "sdk/Cargo.toml".to_string()]
        );
        
        println!("  ✓ Mapped {} git modules with Cargo.toml files", self.toml_in_git.len());
    }
    
    fn demonstrate_toml_file_relationships(&self) {
        println!("\n📄 === CARGO.TOML IN GIT MODULE MAPPING ===");
        
        println!("\n📦 cargo_toml - in_file_in - git_module:");
        for (git_module, toml_files) in &self.toml_in_git {
            let repo_name = git_module.split('/').last().unwrap_or("unknown");
            println!("  Git Module: {}", repo_name);
            for toml_file in toml_files {
                println!("    {} - in_file_in - {}", toml_file, repo_name);
            }
        }
        
        println!("\n🎯 FILE LOCATION BENEFITS:");
        println!("  ✓ Track Cargo.toml file locations in git repos");
        println!("  ✓ Map file paths to their containing repositories");
        println!("  ✓ Enable file-based dependency analysis");
        println!("  ✓ Support Monster Protocol file->repo mapping");
    }
    
    fn run(&mut self) {
        self.map_toml_in_git_relationships();
        self.demonstrate_toml_file_relationships();
    }
}

fn main() {
    let mut mapper = CargoTomlGitMapper::new();
    mapper.run();
}
