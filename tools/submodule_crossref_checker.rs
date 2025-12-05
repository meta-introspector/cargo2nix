use std::fs;
use std::process::Command;
use std::collections::HashMap;

struct SubmoduleCrossrefChecker {
    submodule_crates: HashMap<String, String>, // crate_name -> submodule_path
    rustc_dependencies: Vec<String>,           // needed by rustc
    found_matches: Vec<(String, String)>,      // (crate_name, submodule_path)
    missing_crates: Vec<String>,               // still missing
}

impl SubmoduleCrossrefChecker {
    fn new() -> Self {
        Self {
            submodule_crates: HashMap::new(),
            rustc_dependencies: Vec::new(),
            found_matches: Vec::new(),
            missing_crates: Vec::new(),
        }
    }
    
    fn scan_submodule_cargo_tomls(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning submodules for Cargo.toml files...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(toml_path) {
                if let Some(crate_name) = self.extract_crate_name(&content) {
                    let submodule_path = toml_path.replace("/Cargo.toml", "");
                    self.submodule_crates.insert(crate_name, submodule_path);
                }
            }
        }
        
        println!("  ✓ Found {} crates in submodules", self.submodule_crates.len());
        Ok(())
    }
    
    fn load_rustc_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 Loading rustc dependencies from analysis...");
        
        // Load from previous dry run report
        if let Ok(report) = fs::read_to_string("RUSTC_RECURSIVE_DRY_RUN.md") {
            for line in report.lines() {
                if line.starts_with("- `") && line.contains("` → https://github.com/") {
                    if let Some(crate_name) = line.strip_prefix("- `").and_then(|s| s.split('`').next()) {
                        self.rustc_dependencies.push(crate_name.to_string());
                    }
                }
            }
        }
        
        // Also load from cargo metadata
        if let Ok(metadata) = fs::read_to_string("CARGO_DB_SUMMARY.md") {
            for line in metadata.lines() {
                if line.starts_with("- ") && !line.contains("entries:") {
                    let crate_name = line.strip_prefix("- ").unwrap_or("").to_string();
                    if !crate_name.is_empty() && !self.rustc_dependencies.contains(&crate_name) {
                        self.rustc_dependencies.push(crate_name);
                    }
                }
            }
        }
        
        println!("  ✓ Loaded {} rustc dependencies", self.rustc_dependencies.len());
        Ok(())
    }
    
    fn crossref_dependencies(&mut self) {
        println!("🔍 Cross-referencing rustc dependencies with submodules...");
        
        for dep in &self.rustc_dependencies {
            if let Some(submodule_path) = self.submodule_crates.get(dep) {
                self.found_matches.push((dep.clone(), submodule_path.clone()));
            } else {
                // Check for similar names (e.g., rust-base64 vs base64)
                let mut found = false;
                for (crate_name, submodule_path) in &self.submodule_crates {
                    if self.names_match(dep, crate_name) {
                        self.found_matches.push((dep.clone(), submodule_path.clone()));
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    self.missing_crates.push(dep.clone());
                }
            }
        }
        
        println!("  ✓ Found {} matches in existing submodules", self.found_matches.len());
        println!("  ✗ Still missing {} crates", self.missing_crates.len());
    }
    
    fn names_match(&self, dep: &str, crate_name: &str) -> bool {
        // Check various name patterns
        dep == crate_name ||
        dep.replace("-", "_") == crate_name.replace("-", "_") ||
        dep.contains(crate_name) ||
        crate_name.contains(dep) ||
        format!("rust-{}", dep) == *crate_name ||
        dep == crate_name.strip_prefix("rust-").unwrap_or(crate_name)
    }
    
    fn extract_crate_name(&self, content: &str) -> Option<String> {
        for line in content.lines() {
            if line.trim().starts_with("name = ") {
                return line.split('"').nth(1).map(|s| s.to_string());
            }
        }
        None
    }
    
    fn generate_crossref_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Generating cross-reference report...");
        
        let mut report = String::new();
        report.push_str("# Submodule Cross-Reference Report\n\n");
        
        report.push_str("## Summary\n");
        report.push_str(&format!("- Submodule crates found: {}\n", self.submodule_crates.len()));
        report.push_str(&format!("- Rustc dependencies: {}\n", self.rustc_dependencies.len()));
        report.push_str(&format!("- Matched in submodules: {}\n", self.found_matches.len()));
        report.push_str(&format!("- Still missing: {}\n", self.missing_crates.len()));
        
        let coverage = if self.rustc_dependencies.len() > 0 {
            (self.found_matches.len() as f64 / self.rustc_dependencies.len() as f64) * 100.0
        } else { 0.0 };
        report.push_str(&format!("- Coverage: {:.1}%\n", coverage));
        
        report.push_str("\n## Found Matches\n");
        report.push_str("Rustc dependencies available in our submodules:\n\n");
        
        for (crate_name, submodule_path) in &self.found_matches {
            let short_path = submodule_path.replace("../submodules/", "");
            report.push_str(&format!("- `{}` → `{}`\n", crate_name, short_path));
        }
        
        report.push_str("\n## Still Missing\n");
        report.push_str("These crates need to be added as submodules:\n\n");
        
        for crate_name in &self.missing_crates {
            report.push_str(&format!("- `{}`\n", crate_name));
        }
        
        report.push_str("\n## Monster Protocol Status\n");
        if coverage > 80.0 {
            report.push_str("🎉 EXCELLENT coverage - ready for Monster Protocol build!\n");
        } else if coverage > 60.0 {
            report.push_str("✅ GOOD coverage - few missing dependencies\n");
        } else {
            report.push_str("⚠️ MODERATE coverage - need more submodules\n");
        }
        
        fs::write("SUBMODULE_CROSSREF_REPORT.md", &report)?;
        
        println!("  ✓ Report written to SUBMODULE_CROSSREF_REPORT.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Submodule Cross-Reference Checker");
        
        self.scan_submodule_cargo_tomls()?;
        self.load_rustc_dependencies()?;
        self.crossref_dependencies();
        self.generate_crossref_report()?;
        
        println!("\n🎯 === CROSS-REFERENCE COMPLETE ===");
        println!("  Submodule crates: {}", self.submodule_crates.len());
        println!("  Rustc dependencies: {}", self.rustc_dependencies.len());
        println!("  Found matches: {}", self.found_matches.len());
        println!("  Still missing: {}", self.missing_crates.len());
        
        let coverage = if self.rustc_dependencies.len() > 0 {
            (self.found_matches.len() as f64 / self.rustc_dependencies.len() as f64) * 100.0
        } else { 0.0 };
        
        println!("  Coverage: {:.1}%", coverage);
        
        println!("\n🎉 CROSS-REFERENCE ANALYSIS COMPLETE!");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut checker = SubmoduleCrossrefChecker::new();
    checker.run()
}
