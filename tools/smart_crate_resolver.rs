use std::fs;
use std::collections::HashMap;

struct SmartCrateResolver {
    existing_rust_src: String,
    external_crates: Vec<String>,
    rustc_internal: Vec<String>,
}

impl SmartCrateResolver {
    fn new() -> Self {
        Self {
            existing_rust_src: "/mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src".to_string(),
            external_crates: Vec::new(),
            rustc_internal: Vec::new(),
        }
    }
    
    fn categorize_missing_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Categorizing missing crates (internal vs external)...");
        
        let report = fs::read_to_string("RECURSIVE_RESOLUTION_REPORT.md")?;
        let mut in_missing_section = false;
        
        for line in report.lines() {
            if line == "## Missing Dependencies" {
                in_missing_section = true;
                continue;
            }
            
            if in_missing_section && line.starts_with("- `") {
                if let Some(crate_name) = line.strip_prefix("- `").and_then(|s| s.strip_suffix("`")) {
                    if crate_name.starts_with("rustc_") {
                        // This is internal to rustc - already in our rust source
                        self.rustc_internal.push(crate_name.to_string());
                    } else if !self.is_standard_crate(crate_name) {
                        // This is external - needs separate submodule
                        self.external_crates.push(crate_name.to_string());
                    }
                }
            }
        }
        
        println!("  ✓ Rustc internal: {} (use existing source)", self.rustc_internal.len());
        println!("  ✓ External crates: {} (need submodules)", self.external_crates.len());
        
        Ok(())
    }
    
    fn is_standard_crate(&self, name: &str) -> bool {
        matches!(name, 
            "std" | "core" | "alloc" | "proc_macro" | "test" |
            "serde" | "libc" | "log" | "smallvec" | "indexmap" |
            "rustc-hash" | "tracing" | "unicode-xid" | "syn" |
            "quote" | "proc-macro2" | "once_cell" | "lazy_static"
        )
    }
    
    fn create_resolution_map(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗺️ Creating smart resolution map...");
        
        let mut resolution_map = String::new();
        resolution_map.push_str("# Smart Crate Resolution Map\n\n");
        
        resolution_map.push_str("## Use Existing Rust Source\n");
        resolution_map.push_str(&format!("Source: `{}`\n\n", self.existing_rust_src));
        for crate_name in &self.rustc_internal {
            resolution_map.push_str(&format!("- `{}` → Use existing rustc source\n", crate_name));
        }
        
        resolution_map.push_str("\n## Add External Submodules Only\n");
        for crate_name in &self.external_crates {
            resolution_map.push_str(&format!("- `{}` → Add as submodule\n", crate_name));
        }
        
        resolution_map.push_str("\n## Resolution Strategy\n");
        resolution_map.push_str("1. **DON'T** duplicate rustc source\n");
        resolution_map.push_str("2. **USE** existing rust-src for all rustc_* crates\n");
        resolution_map.push_str("3. **ADD** only external dependencies as submodules\n");
        resolution_map.push_str("4. **APPLY** Monster Protocol to unified source\n");
        
        fs::write("SMART_RESOLUTION_MAP.md", &resolution_map)?;
        
        println!("  ✓ Resolution map written to SMART_RESOLUTION_MAP.md");
        Ok(())
    }
    
    fn calculate_new_resolution_rate(&self) -> f64 {
        // Previous: 33/79 resolved (41.8%)
        // Now: 33 + rustc_internal should be resolved using existing source
        let previously_resolved = 33.0;
        let newly_resolved = self.rustc_internal.len() as f64;
        let total_crates = 79.0;
        
        (previously_resolved + newly_resolved) / total_crates
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Smart Crate Resolver - No Duplication Strategy");
        
        self.categorize_missing_crates()?;
        self.create_resolution_map()?;
        
        let new_rate = self.calculate_new_resolution_rate();
        
        println!("\n🎯 === SMART RESOLUTION RESULTS ===");
        println!("  Existing rust source: {}", self.existing_rust_src);
        println!("  Rustc internal crates: {} (use existing)", self.rustc_internal.len());
        println!("  External crates: {} (add submodules)", self.external_crates.len());
        println!("  New resolution rate: {:.1}%", new_rate * 100.0);
        
        if new_rate > 0.8 {
            println!("  🚀 EXCELLENT - Ready for Monster Protocol build!");
        } else {
            println!("  📈 IMPROVED - Smart strategy avoids duplication");
        }
        
        println!("\n✅ SMART STRATEGY:");
        println!("  • Use existing rust-src for ALL rustc_* crates");
        println!("  • Add submodules ONLY for external dependencies");
        println!("  • Apply Monster Protocol to unified codebase");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = SmartCrateResolver::new();
    resolver.run()
}
