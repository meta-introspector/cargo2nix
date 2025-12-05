use std::collections::HashMap;

struct CargoUsesMapper {
    uses_relationships: HashMap<String, Vec<String>>, // dependent -> [dependencies]
}

impl CargoUsesMapper {
    fn new() -> Self {
        Self {
            uses_relationships: HashMap::new(),
        }
    }
    
    fn map_uses_relationships(&mut self) {
        println!("📦 Mapping cargo module uses relationships...");
        
        // cargo_module <- uses <- cargo_module
        self.uses_relationships.insert(
            "rustc_driver".to_string(),
            vec!["rustc_interface".to_string(), "rustc_middle".to_string()]
        );
        
        self.uses_relationships.insert(
            "monster_protocol".to_string(),
            vec!["serde".to_string(), "minizinc".to_string()]
        );
        
        self.uses_relationships.insert(
            "solana-program".to_string(),
            vec!["solana-sdk".to_string(), "borsh".to_string()]
        );
        
        println!("  ✓ Mapped {} cargo modules with dependencies", self.uses_relationships.len());
    }
    
    fn demonstrate_uses_relationships(&self) {
        println!("\n📦 === CARGO MODULE USES RELATIONSHIPS ===");
        
        println!("\n🔗 cargo_module <- uses <- cargo_module:");
        for (dependent, dependencies) in &self.uses_relationships {
            println!("  Dependent: {}", dependent);
            for dependency in dependencies {
                println!("    {} <- uses <- {}", dependency, dependent);
            }
        }
        
        println!("\n🎯 USES BENEFITS:");
        println!("  ✓ Track cargo crate dependency relationships");
        println!("  ✓ Map which crate uses which other crate");
        println!("  ✓ Enable dependency graph analysis");
        println!("  ✓ Support Monster Protocol dependency mapping");
    }
    
    fn run(&mut self) {
        self.map_uses_relationships();
        self.demonstrate_uses_relationships();
    }
}

fn main() {
    let mut mapper = CargoUsesMapper::new();
    mapper.run();
}
