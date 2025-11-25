use std::collections::HashMap;

struct GitSameNameMapper {
    same_name_groups: HashMap<String, Vec<String>>, // name -> [git_modules]
}

impl GitSameNameMapper {
    fn new() -> Self {
        Self {
            same_name_groups: HashMap::new(),
        }
    }
    
    fn map_same_name_relationships(&mut self) {
        println!("📛 Mapping same name git modules...");
        
        // git module - same_name_as - git module
        self.add_same_name_group("rust", vec![
            "https://github.com/rust-lang/rust",
            "https://github.com/meta-introspector/rust"
        ]);
        
        self.add_same_name_group("cargo2nix", vec![
            "https://github.com/cargo2nix/cargo2nix",
            "https://github.com/meta-introspector/cargo2nix"
        ]);
        
        self.add_same_name_group("solana", vec![
            "https://github.com/solana-labs/solana",
            "https://github.com/meta-introspector/solana"
        ]);
        
        println!("  ✓ Mapped {} same name groups", self.same_name_groups.len());
    }
    
    fn add_same_name_group(&mut self, name: &str, modules: Vec<&str>) {
        self.same_name_groups.insert(
            name.to_string(),
            modules.into_iter().map(|s| s.to_string()).collect()
        );
    }
    
    fn demonstrate_same_name_relationships(&self) {
        println!("\n📛 === GIT MODULE SAME NAME GRAPH ===");
        
        println!("\n🔗 git_module - same_name_as - git_module:");
        for (name, modules) in &self.same_name_groups {
            println!("  Name: '{}'", name);
            for (i, module) in modules.iter().enumerate() {
                let owner = module.split('/').nth(3).unwrap_or("unknown");
                if i == 0 {
                    println!("    {} (original)", owner);
                } else {
                    println!("    {} - same_name_as - {}", owner, modules[0].split('/').nth(3).unwrap_or("unknown"));
                }
            }
        }
        
        println!("\n🎯 SAME NAME BENEFITS:");
        println!("  ✓ Group repositories by identical names");
        println!("  ✓ Enable name-based discovery");
        println!("  ✓ Support automated sync by name matching");
        println!("  ✓ Track original vs fork by same name");
    }
    
    fn run(&mut self) {
        self.map_same_name_relationships();
        self.demonstrate_same_name_relationships();
    }
}

fn main() {
    let mut mapper = GitSameNameMapper::new();
    mapper.run();
}
