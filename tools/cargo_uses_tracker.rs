use std::fs;
use std::collections::HashMap;

struct CargoUsesTracker {
    uses_graph: HashMap<String, Vec<String>>, // crate -> [dependencies]
}

impl CargoUsesTracker {
    fn new() -> Self {
        Self {
            uses_graph: HashMap::new(),
        }
    }
    
    fn build_uses_relationships(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Building cargo uses relationships...");
        
        let cargo_files = [
            "Cargo.toml",
            "monster_protocol/Cargo.toml",
            "cargo-llm-bootstrap/Cargo.toml"
        ];
        
        for cargo_file in &cargo_files {
            if let Ok(content) = fs::read_to_string(cargo_file) {
                if let Some(crate_name) = self.extract_crate_name(&content) {
                    let deps = self.extract_dependencies(&content);
                    self.uses_graph.insert(crate_name, deps);
                }
            }
        }
        
        println!("  ✓ Built uses graph for {} crates", self.uses_graph.len());
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
    
    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        let mut deps = Vec::new();
        let mut in_deps_section = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line == "[dependencies]" {
                in_deps_section = true;
                continue;
            }
            
            if line.starts_with('[') && line != "[dependencies]" {
                in_deps_section = false;
            }
            
            if in_deps_section && line.contains('=') {
                if let Some(dep_name) = line.split('=').next() {
                    deps.push(dep_name.trim().to_string());
                }
            }
        }
        
        deps
    }
    
    fn demonstrate_uses_relationships(&self) {
        println!("\n📦 === CARGO USES RELATIONSHIP GRAPH ===");
        
        println!("\n🔗 cargo_module <- uses <- cargo_module:");
        for (crate_name, deps) in &self.uses_graph {
            if !deps.is_empty() {
                println!("  Crate: {}", crate_name);
                for dep in deps {
                    println!("    {} <- uses <- {}", dep, crate_name);
                }
                println!();
            }
        }
        
        println!("📊 USES STATISTICS:");
        let total_deps: usize = self.uses_graph.values().map(|v| v.len()).sum();
        println!("  Crates analyzed: {}", self.uses_graph.len());
        println!("  Total dependencies: {}", total_deps);
        println!("  Average deps per crate: {:.1}", total_deps as f64 / self.uses_graph.len() as f64);
        
        println!("\n🎯 USES BENEFITS:");
        println!("  ✓ Track cargo crate dependencies");
        println!("  ✓ Map which crate uses which other crate");
        println!("  ✓ Enable dependency graph analysis");
        println!("  ✓ Support Monster Protocol dependency mapping");
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.build_uses_relationships()?;
        self.demonstrate_uses_relationships();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker = CargoUsesTracker::new();
    tracker.run()
}
