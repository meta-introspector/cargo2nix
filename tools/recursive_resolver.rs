use std::fs;
use std::collections::{HashMap, HashSet};
use std::process::Command;

struct RecursiveResolver {
    submodules: HashMap<String, String>,        // crate_name -> repo_url
    rustc_crates: HashMap<String, Vec<String>>, // crate_name -> dependencies
    resolution_graph: HashMap<String, bool>,    // crate_name -> resolved
    missing_deps: Vec<String>,
}

impl RecursiveResolver {
    fn new() -> Self {
        Self {
            submodules: HashMap::new(),
            rustc_crates: HashMap::new(),
            resolution_graph: HashMap::new(),
            missing_deps: Vec::new(),
        }
    }
    
    fn read_git_inventory_to_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📂 Reading 12k git files into database...");
        
        let content = fs::read_to_string("../git_files_inventory2.txt")?;
        let mut repo_count = 0;
        
        for line in content.lines() {
            if line.ends_with("/.git") {
                let repo_path = line.replace("/.git", "");
                let repo_name = repo_path.split('/').last().unwrap_or("unknown");
                
                // Simulate git remote URL lookup
                let repo_url = format!("https://github.com/meta-introspector/{}", repo_name);
                self.submodules.insert(repo_name.to_string(), repo_url);
                repo_count += 1;
            }
        }
        
        println!("  ✓ Loaded {} repositories into database", repo_count);
        Ok(())
    }
    
    fn extract_rustc_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 Extracting rustc crates and dependencies...");
        
        let rust_src = "/mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
        
        let output = Command::new("find")
            .args(&[rust_src, "-name", "Cargo.toml"])
            .output()?;
        
        if !output.status.success() {
            println!("  ✗ Failed to find Cargo.toml files");
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for toml_path in cargo_files.lines() {
            if toml_path.contains("rustc_") {
                if let Ok(content) = fs::read_to_string(toml_path) {
                    if let Some(crate_name) = self.extract_crate_name(&content) {
                        let deps = self.extract_dependencies(&content);
                        self.rustc_crates.insert(crate_name, deps);
                    }
                }
            }
        }
        
        println!("  ✓ Found {} rustc crates", self.rustc_crates.len());
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
        let mut in_deps = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line == "[dependencies]" {
                in_deps = true;
                continue;
            }
            
            if line.starts_with('[') && line != "[dependencies]" {
                in_deps = false;
            }
            
            if in_deps && line.contains('=') && !line.starts_with('#') {
                if let Some(dep_name) = line.split('=').next() {
                    let clean_dep = dep_name.trim().to_string();
                    if !clean_dep.is_empty() {
                        deps.push(clean_dep);
                    }
                }
            }
        }
        
        deps
    }
    
    fn check_recursive_resolution(&mut self) -> bool {
        println!("🔄 Checking recursive resolution of rustc crates...");
        
        let mut resolved_count = 0;
        let mut unresolved_count = 0;
        
        for (crate_name, dependencies) in &self.rustc_crates {
            let mut can_resolve = true;
            let mut missing_for_crate = Vec::new();
            
            for dep in dependencies {
                // Check if dependency exists in our submodules or is a standard crate
                if !self.submodules.contains_key(dep) && 
                   !self.is_standard_crate(dep) &&
                   !self.rustc_crates.contains_key(dep) {
                    can_resolve = false;
                    missing_for_crate.push(dep.clone());
                }
            }
            
            self.resolution_graph.insert(crate_name.clone(), can_resolve);
            
            if can_resolve {
                resolved_count += 1;
            } else {
                unresolved_count += 1;
                self.missing_deps.extend(missing_for_crate);
            }
        }
        
        println!("  ✓ Resolved: {} crates", resolved_count);
        println!("  ✗ Unresolved: {} crates", unresolved_count);
        
        resolved_count > 0
    }
    
    fn is_standard_crate(&self, crate_name: &str) -> bool {
        // Standard Rust crates that don't need git repos
        matches!(crate_name, 
            "std" | "core" | "alloc" | "proc_macro" | 
            "serde" | "libc" | "log" | "smallvec" | 
            "indexmap" | "rustc-hash" | "tracing"
        )
    }
    
    fn generate_resolution_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🎯 === RECURSIVE RESOLUTION REPORT ===");
        
        let mut report = String::new();
        report.push_str("# Rustc Recursive Resolution Analysis\n\n");
        
        report.push_str("## Database Statistics\n");
        report.push_str(&format!("- Submodules in database: {}\n", self.submodules.len()));
        report.push_str(&format!("- Rustc crates found: {}\n", self.rustc_crates.len()));
        report.push_str(&format!("- Resolution attempts: {}\n", self.resolution_graph.len()));
        
        let resolved = self.resolution_graph.values().filter(|&&v| v).count();
        let unresolved = self.resolution_graph.len() - resolved;
        
        report.push_str(&format!("- Successfully resolved: {}\n", resolved));
        report.push_str(&format!("- Failed to resolve: {}\n", unresolved));
        
        report.push_str("\n## Resolved Crates\n");
        for (crate_name, &resolved) in &self.resolution_graph {
            if resolved {
                let dep_count = self.rustc_crates.get(crate_name).map(|d| d.len()).unwrap_or(0);
                report.push_str(&format!("- `{}` ({} dependencies)\n", crate_name, dep_count));
            }
        }
        
        report.push_str("\n## Unresolved Crates\n");
        for (crate_name, &resolved) in &self.resolution_graph {
            if !resolved {
                report.push_str(&format!("- `{}` (missing dependencies)\n", crate_name));
            }
        }
        
        report.push_str("\n## Missing Dependencies\n");
        let mut unique_missing: Vec<_> = self.missing_deps.iter().collect::<HashSet<_>>().into_iter().collect();
        unique_missing.sort();
        for dep in unique_missing {
            report.push_str(&format!("- `{}`\n", dep));
        }
        
        fs::write("RECURSIVE_RESOLUTION_REPORT.md", &report)?;
        
        println!("\n📊 RESOLUTION STATISTICS:");
        println!("  Submodules in DB: {}", self.submodules.len());
        println!("  Rustc crates: {}", self.rustc_crates.len());
        println!("  Resolved: {}", resolved);
        println!("  Unresolved: {}", unresolved);
        println!("  Missing deps: {}", self.missing_deps.len());
        
        let resolution_rate = if self.rustc_crates.len() > 0 {
            (resolved as f64 / self.rustc_crates.len() as f64) * 100.0
        } else { 0.0 };
        
        println!("  Resolution rate: {:.1}%", resolution_rate);
        
        if resolution_rate > 80.0 {
            println!("  🎉 HIGH RESOLUTION RATE - Ready for Monster Protocol build!");
        } else if resolution_rate > 50.0 {
            println!("  ⚠️ MODERATE RESOLUTION - Need more submodules");
        } else {
            println!("  ❌ LOW RESOLUTION - Major dependencies missing");
        }
        
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Recursive Resolver - Database Ingestion & Resolution Check");
        
        self.read_git_inventory_to_db()?;
        self.extract_rustc_crates()?;
        self.check_recursive_resolution();
        self.generate_resolution_report()?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = RecursiveResolver::new();
    resolver.run()
}
