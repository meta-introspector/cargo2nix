use std::fs;
use std::process::Command;
use std::collections::HashMap;

struct CargoMetadataCollector {
    missing_crates: Vec<String>,
    cargo_db: HashMap<String, String>,
    collected_count: u32,
}

impl CargoMetadataCollector {
    fn new() -> Self {
        Self {
            missing_crates: Vec::new(),
            cargo_db: HashMap::new(),
            collected_count: 0,
        }
    }
    
    fn load_missing_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Loading missing crates from dry run report...");
        
        let report = fs::read_to_string("RUSTC_RECURSIVE_DRY_RUN.md")?;
        
        for line in report.lines() {
            if line.starts_with("- `") && line.contains("` → https://github.com/") {
                if let Some(crate_name) = line.strip_prefix("- `").and_then(|s| s.split('`').next()) {
                    self.missing_crates.push(crate_name.to_string());
                }
            }
        }
        
        println!("  ✓ Found {} missing crates", self.missing_crates.len());
        Ok(())
    }
    
    fn collect_all_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Collecting metadata for missing crates...");
        
        let crates_to_process = self.missing_crates.clone();
        
        for (i, crate_name) in crates_to_process.iter().enumerate() {
            if i % 20 == 0 {
                println!("  Progress: {}/{}", i, crates_to_process.len());
            }
            
            if self.collect_single_crate(crate_name) {
                self.collected_count += 1;
            }
        }
        
        println!("  ✓ Collected {} crate metadata entries", self.collected_count);
        Ok(())
    }
    
    fn collect_single_crate(&mut self, crate_name: &str) -> bool {
        // Try crates.io API
        let output = Command::new("curl")
            .args(&["-s", &format!("https://crates.io/api/v1/crates/{}", crate_name)])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                let metadata = String::from_utf8_lossy(&result.stdout);
                if !metadata.contains("Not Found") && metadata.len() > 50 {
                    self.cargo_db.insert(crate_name.to_string(), metadata.to_string());
                    return true;
                }
            }
            _ => {}
        }
        
        // Create minimal placeholder
        let placeholder = format!(r#"{{"name": "{}", "status": "not_found"}}"#, crate_name);
        self.cargo_db.insert(crate_name.to_string(), placeholder);
        false
    }
    
    fn save_cargo_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("💾 Saving cargo metadata database...");
        
        // Save as JSON
        let mut json_content = String::new();
        json_content.push_str("{\n");
        
        let entries: Vec<_> = self.cargo_db.iter().collect();
        for (i, (crate_name, metadata)) in entries.iter().enumerate() {
            json_content.push_str(&format!("  \"{}\": {}", crate_name, metadata));
            if i < entries.len() - 1 {
                json_content.push_str(",");
            }
            json_content.push_str("\n");
        }
        json_content.push_str("}\n");
        
        fs::write("cargo_metadata_db.json", &json_content)?;
        
        // Save summary
        let mut summary = String::new();
        summary.push_str("# Cargo Metadata Database Summary\n\n");
        summary.push_str(&format!("Total entries: {}\n", self.cargo_db.len()));
        summary.push_str(&format!("Successfully collected: {}\n", self.collected_count));
        summary.push_str("\n## Crates with metadata:\n");
        
        for crate_name in self.cargo_db.keys() {
            summary.push_str(&format!("- {}\n", crate_name));
        }
        
        fs::write("CARGO_DB_SUMMARY.md", &summary)?;
        
        println!("  ✓ Saved cargo_metadata_db.json and CARGO_DB_SUMMARY.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Cargo Metadata Collector");
        
        self.load_missing_crates()?;
        self.collect_all_metadata()?;
        self.save_cargo_db()?;
        
        println!("\n📊 COLLECTION RESULTS:");
        println!("  Missing crates: {}", self.missing_crates.len());
        println!("  Metadata collected: {}", self.collected_count);
        println!("  Database entries: {}", self.cargo_db.len());
        
        let rate = (self.collected_count as f64 / self.missing_crates.len() as f64) * 100.0;
        println!("  Success rate: {:.1}%", rate);
        
        println!("\n🎯 CARGO DATABASE READY for Monster Protocol!");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = CargoMetadataCollector::new();
    collector.run()
}
