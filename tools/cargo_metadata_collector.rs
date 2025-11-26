use std::fs;
use std::process::Command;
use std::collections::HashMap;

struct CargoMetadataCollector {
    missing_crates: Vec<String>,
    cargo_db: HashMap<String, String>, // crate_name -> metadata_json
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
        
        println!("  ✓ Found {} missing crates to collect metadata for", self.missing_crates.len());
        Ok(())
    }
    
    fn collect_crate_metadata(&mut self, crate_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Try cargo metadata for the crate
        let output = Command::new("cargo")
            .args(&["metadata", "--format-version", "1", "--no-deps"])
            .env("CARGO_TARGET_DIR", "/tmp/cargo_metadata")
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                let metadata = String::from_utf8_lossy(&result.stdout);
                self.cargo_db.insert(crate_name.to_string(), metadata.to_string());
                Ok(true)
            }
            _ => {
                // Try crates.io API as fallback
                self.collect_from_crates_io(crate_name)
            }
        }
    }
    
    fn collect_from_crates_io(&mut self, crate_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let output = Command::new("curl")
            .args(&["-s", &format!("https://crates.io/api/v1/crates/{}", crate_name)])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                let metadata = String::from_utf8_lossy(&result.stdout);
                if !metadata.contains("Not Found") {
                    self.cargo_db.insert(crate_name.to_string(), metadata.to_string());
                    return Ok(true);
                }
            }
            _ => {}
        }
        
        // Create placeholder metadata
        let placeholder = format!(r#"{{"name": "{}", "version": "unknown", "source": "placeholder"}}"#, crate_name);
        self.cargo_db.insert(crate_name.to_string(), placeholder);
        Ok(false)
    }
    
    fn collect_all_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Collecting metadata for {} missing crates...", self.missing_crates.len());
        
        for (i, crate_name) in self.missing_crates.iter().enumerate() {
            if i % 10 == 0 {
                println!("  Progress: {}/{}", i, self.missing_crates.len());
            }
            
            if self.collect_crate_metadata(crate_name)? {
                self.collected_count += 1;
            }
        }
        
        println!("  ✓ Collected metadata for {} crates", self.collected_count);
        Ok(())
    }
    
    fn save_cargo_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("💾 Saving cargo metadata database...");
        
        let mut db_content = String::new();
        db_content.push_str("# Cargo Metadata Database\n");
        db_content.push_str("# Generated from missing rustc dependencies\n\n");
        
        for (crate_name, metadata) in &self.cargo_db {
            db_content.push_str(&format!("## {}\n", crate_name));
            db_content.push_str("```json\n");
            db_content.push_str(metadata);
            db_content.push_str("\n```\n\n");
        }
        
        fs::write("CARGO_METADATA_DB.md", &db_content)?;
        
        // Also save as JSON
        let mut json_db = String::new();
        json_db.push_str("{\n");
        for (i, (crate_name, metadata)) in self.cargo_db.iter().enumerate() {
            json_db.push_str(&format!("  \"{}\": {}", crate_name, metadata));
            if i < self.cargo_db.len() - 1 {
                json_db.push_str(",");
            }
            json_db.push_str("\n");
        }
        json_db.push_str("}\n");
        
        fs::write("cargo_metadata_db.json", &json_db)?;
        
        println!("  ✓ Saved to CARGO_METADATA_DB.md and cargo_metadata_db.json");
        Ok(())
    }
    
    fn generate_summary_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::new();
        report.push_str("# Cargo Metadata Collection Summary\n\n");
        
        report.push_str("## Collection Results\n");
        report.push_str(&format!("- Missing crates identified: {}\n", self.missing_crates.len()));
        report.push_str(&format!("- Metadata collected: {}\n", self.collected_count));
        report.push_str(&format!("- Success rate: {:.1}%\n", 
            (self.collected_count as f64 / self.missing_crates.len() as f64) * 100.0));
        
        report.push_str("\n## Database Files\n");
        report.push_str("- `CARGO_METADATA_DB.md` - Human readable format\n");
        report.push_str("- `cargo_metadata_db.json` - Machine readable format\n");
        
        report.push_str("\n## Next Steps\n");
        report.push_str("1. Review collected metadata\n");
        report.push_str("2. Identify critical dependencies\n");
        report.push_str("3. Add priority submodules to Monster Protocol\n");
        
        fs::write("CARGO_COLLECTION_SUMMARY.md", &report)?;
        
        println!("  ✓ Summary written to CARGO_COLLECTION_SUMMARY.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Cargo Metadata Collector for Missing Dependencies");
        
        self.load_missing_crates()?;
        self.collect_all_metadata()?;
        self.save_cargo_db()?;
        self.generate_summary_report()?;
        
        println!("\n🎯 === COLLECTION COMPLETE ===");
        println!("  Missing crates: {}", self.missing_crates.len());
        println!("  Metadata collected: {}", self.collected_count);
        println!("  Database entries: {}", self.cargo_db.len());
        
        let success_rate = (self.collected_count as f64 / self.missing_crates.len() as f64) * 100.0;
        println!("  Success rate: {:.1}%", success_rate);
        
        if success_rate > 80.0 {
            println!("  🎉 EXCELLENT - comprehensive metadata collected!");
        } else if success_rate > 60.0 {
            println!("  ✅ GOOD - substantial metadata available");
        } else {
            println!("  ⚠️ PARTIAL - some metadata missing");
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = CargoMetadataCollector::new();
    collector.run()
}
