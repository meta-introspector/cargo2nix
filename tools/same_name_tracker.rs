use std::collections::HashMap;

struct SameNameTracker {
    name_groups: HashMap<String, Vec<String>>, // name -> [urls]
}

impl SameNameTracker {
    fn new() -> Self {
        Self {
            name_groups: HashMap::new(),
        }
    }
    
    fn build_same_name_relationships(&mut self) {
        println!("📛 Building same name relationships...");
        
        // git module - same_name_as - git module
        self.add_same_name("rust", vec![
            "https://github.com/rust-lang/rust",
            "https://github.com/meta-introspector/rust",
        ]);
        
        self.add_same_name("solana", vec![
            "https://github.com/solana-labs/solana", 
            "https://github.com/meta-introspector/solana",
        ]);
        
        self.add_same_name("cargo2nix", vec![
            "https://github.com/cargo2nix/cargo2nix",
            "https://github.com/meta-introspector/cargo2nix",
        ]);
        
        println!("  ✓ Built {} same name groups", self.name_groups.len());
    }
    
    fn add_same_name(&mut self, name: &str, urls: Vec<&str>) {
        self.name_groups.insert(
            name.to_string(), 
            urls.into_iter().map(|s| s.to_string()).collect()
        );
    }
    
    fn demonstrate_same_name_relationships(&self) {
        println!("\n📛 === SAME NAME RELATIONSHIP GRAPH ===");
        
        println!("\n🔗 git_module - same_name_as - git_module:");
        for (name, urls) in &self.name_groups {
            println!("  Name: '{}'", name);
            for (i, url) in urls.iter().enumerate() {
                let repo_path = url.split('/').collect::<Vec<_>>();
                let owner = repo_path.get(repo_path.len()-2).unwrap_or(&"unknown");
                if i == 0 {
                    println!("    {} (original)", owner);
                } else {
                    println!("    {} - same_name_as - {}", owner, 
                             repo_path.get(repo_path.len()-2).unwrap_or(&"unknown"));
                }
            }
            println!();
        }
        
        println!("📊 SAME NAME STATISTICS:");
        let total_repos: usize = self.name_groups.values().map(|v| v.len()).sum();
        println!("  Total repositories: {}", total_repos);
        println!("  Name groups: {}", self.name_groups.len());
        println!("  Average repos per name: {:.1}", total_repos as f64 / self.name_groups.len() as f64);
        
        println!("\n🎯 SAME NAME BENEFITS:");
        println!("  ✓ Identify repositories with identical names");
        println!("  ✓ Track original vs fork relationships");
        println!("  ✓ Enable name-based repository discovery");
        println!("  ✓ Support automated sync by name matching");
    }
    
    fn run(&mut self) {
        self.build_same_name_relationships();
        self.demonstrate_same_name_relationships();
    }
}

fn main() {
    let mut tracker = SameNameTracker::new();
    tracker.run();
}
