use std::fs;
use std::collections::HashMap;

struct GitForkTracker {
    modules: HashMap<String, GitModule>,
    fork_relationships: HashMap<String, String>, // fork_url -> original_url
}

#[derive(Debug)]
struct GitModule {
    url: String,
    path: String,
    is_fork: bool,
    original_url: Option<String>,
}

impl GitForkTracker {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            fork_relationships: HashMap::new(),
        }
    }
    
    fn detect_forks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Detecting git module fork relationships...");
        
        let content = fs::read_to_string("../git_files_inventory2.txt")?;
        
        for line in content.lines().take(100) {
            if line.ends_with("/.git") {
                let repo_path = line.replace("/.git", "");
                self.analyze_repo(&repo_path)?;
            }
        }
        
        self.find_fork_relationships();
        println!("  ✓ Found {} fork relationships", self.fork_relationships.len());
        Ok(())
    }
    
    fn analyze_repo(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = format!("{}/.git/config", path);
        if let Ok(config) = fs::read_to_string(&config_path) {
            let url = self.extract_url(&config);
            
            let module = GitModule {
                url: url.clone(),
                path: path.to_string(),
                is_fork: self.is_fork_url(&url),
                original_url: self.detect_original_url(&url),
            };
            
            self.modules.insert(path.to_string(), module);
        }
        Ok(())
    }
    
    fn extract_url(&self, config: &str) -> String {
        for line in config.lines() {
            if line.trim().starts_with("url = ") {
                return line.trim()[6..].to_string();
            }
        }
        String::new()
    }
    
    fn is_fork_url(&self, url: &str) -> bool {
        url.contains("meta-introspector") || url.contains("fork")
    }
    
    fn detect_original_url(&self, fork_url: &str) -> Option<String> {
        if fork_url.contains("meta-introspector") {
            // Convert meta-introspector fork back to original
            if let Some(repo_name) = fork_url.split('/').last() {
                let clean_name = repo_name.replace(".git", "");
                // Common original patterns
                if clean_name.contains("rust") {
                    return Some(format!("https://github.com/rust-lang/{}", clean_name));
                }
                if clean_name.contains("solana") {
                    return Some(format!("https://github.com/solana-labs/{}", clean_name));
                }
                // Generic original
                return Some(format!("https://github.com/original/{}", clean_name));
            }
        }
        None
    }
    
    fn find_fork_relationships(&mut self) {
        for module in self.modules.values() {
            if module.is_fork {
                if let Some(original) = &module.original_url {
                    self.fork_relationships.insert(module.url.clone(), original.clone());
                }
            }
        }
    }
    
    fn generate_fork_report(&self) {
        println!("\n🍴 === GIT FORK RELATIONSHIP REPORT ===");
        
        let total_modules = self.modules.len();
        let fork_count = self.modules.values().filter(|m| m.is_fork).count();
        
        println!("\n📊 STATISTICS:");
        println!("  Total git modules: {}", total_modules);
        println!("  Detected forks: {}", fork_count);
        println!("  Fork relationships: {}", self.fork_relationships.len());
        
        println!("\n🔗 FORK RELATIONSHIPS:");
        for (fork_url, original_url) in self.fork_relationships.iter().take(10) {
            let fork_name = fork_url.split('/').last().unwrap_or("unknown");
            let original_name = original_url.split('/').last().unwrap_or("unknown");
            println!("  {} → {}", fork_name, original_name);
        }
        
        println!("\n🎯 FORK MAPPING:");
        println!("  meta-introspector/* → original repositories");
        println!("  Enables bidirectional sync and upstream tracking");
        println!("  Fork graph: git_module → fork_of → git_module");
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.detect_forks()?;
        self.generate_fork_report();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker = GitForkTracker::new();
    tracker.run()
}
