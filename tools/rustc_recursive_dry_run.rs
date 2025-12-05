use std::fs;
use std::process::Command;
use std::collections::{HashMap, HashSet};

struct RustcRecursiveDryRun {
    rust_src_path: String,
    rustc_crates: HashMap<String, Vec<String>>, // crate -> dependencies
    existing_submodules: HashSet<String>,
    needed_submodules: Vec<String>,
    missing_count: u32,
}

impl RustcRecursiveDryRun {
    fn new() -> Self {
        Self {
            rust_src_path: "/mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src".to_string(),
            rustc_crates: HashMap::new(),
            existing_submodules: HashSet::new(),
            needed_submodules: Vec::new(),
            missing_count: 0,
        }
    }
    
    fn scan_existing_submodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning existing submodules...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        for line in gitmodules.lines() {
            if line.contains("url = https://github.com/") {
                if let Some(repo_name) = line.split('/').last() {
                    let clean_name = repo_name.replace(".git", "");
                    self.existing_submodules.insert(clean_name);
                }
            }
        }
        
        println!("  ✓ Found {} existing submodules", self.existing_submodules.len());
        Ok(())
    }
    
    fn analyze_rustc_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 Analyzing entire rustc dependency tree...");
        
        let output = Command::new("find")
            .args(&[&self.rust_src_path, "-name", "Cargo.toml"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        let mut total_deps = 0;
        
        for toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(toml_path) {
                if let Some(crate_name) = self.extract_crate_name(&content) {
                    let deps = self.extract_all_dependencies(&content);
                    total_deps += deps.len();
                    self.rustc_crates.insert(crate_name, deps);
                }
            }
        }
        
        println!("  ✓ Analyzed {} rustc crates", self.rustc_crates.len());
        println!("  ✓ Found {} total dependencies", total_deps);
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
    
    fn extract_all_dependencies(&self, content: &str) -> Vec<String> {
        let mut deps = Vec::new();
        let mut in_deps_section = false;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("[dependencies") || line.starts_with("[dev-dependencies") || line.starts_with("[build-dependencies") {
                in_deps_section = true;
                continue;
            }
            
            if line.starts_with('[') && in_deps_section {
                in_deps_section = false;
            }
            
            if in_deps_section && line.contains('=') && !line.starts_with('#') {
                if let Some(dep_name) = line.split('=').next() {
                    let clean_dep = dep_name.trim().replace('"', "");
                    if !clean_dep.is_empty() && !self.is_standard_crate(&clean_dep) {
                        deps.push(clean_dep);
                    }
                }
            }
        }
        
        deps
    }
    
    fn is_standard_crate(&self, name: &str) -> bool {
        matches!(name, 
            "std" | "core" | "alloc" | "proc_macro" | "test" |
            "serde" | "libc" | "log" | "smallvec" | "indexmap" |
            "rustc-hash" | "tracing" | "unicode-xid" | "syn" |
            "quote" | "proc-macro2" | "once_cell" | "lazy_static"
        ) || name.starts_with("rustc_")
    }
    
    fn identify_missing_submodules(&mut self) {
        println!("🔍 Identifying missing submodules for complete rustc build...");
        
        let mut all_external_deps = HashSet::new();
        
        for (_, deps) in &self.rustc_crates {
            for dep in deps {
                if !dep.starts_with("rustc_") && !self.is_standard_crate(dep) {
                    all_external_deps.insert(dep.clone());
                }
            }
        }
        
        for dep in &all_external_deps {
            if !self.existing_submodules.contains(dep) {
                self.needed_submodules.push(dep.clone());
                self.missing_count += 1;
            }
        }
        
        self.needed_submodules.sort();
        
        println!("  ✓ Total external dependencies: {}", all_external_deps.len());
        println!("  ✓ Already have submodules: {}", all_external_deps.len() - self.missing_count as usize);
        println!("  ✗ Missing submodules: {}", self.missing_count);
    }
    
    fn generate_dry_run_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Generating dry run report...");
        
        let mut report = String::new();
        report.push_str("# Rustc Recursive Dependency Dry Run Report\n\n");
        
        report.push_str("## Analysis Summary\n");
        report.push_str(&format!("- Rustc source path: `{}`\n", self.rust_src_path));
        report.push_str(&format!("- Rustc crates analyzed: {}\n", self.rustc_crates.len()));
        report.push_str(&format!("- Existing submodules: {}\n", self.existing_submodules.len()));
        report.push_str(&format!("- Missing submodules needed: {}\n", self.missing_count));
        
        let coverage = if self.missing_count > 0 {
            let total_needed = self.needed_submodules.len() + (self.existing_submodules.len() - self.missing_count as usize);
            ((self.existing_submodules.len() as f64 / total_needed as f64) * 100.0)
        } else { 100.0 };
        
        report.push_str(&format!("- Coverage: {:.1}%\n", coverage));
        
        report.push_str("\n## Missing Submodules (Dry Run)\n");
        report.push_str("These submodules would need to be added for complete rustc build:\n\n");
        
        for (i, submodule) in self.needed_submodules.iter().enumerate() {
            if i < 20 { // Show first 20
                report.push_str(&format!("- `{}` → https://github.com/rust-lang/{}\n", submodule, submodule));
            }
        }
        
        if self.needed_submodules.len() > 20 {
            report.push_str(&format!("- ... and {} more\n", self.needed_submodules.len() - 20));
        }
        
        report.push_str("\n## Monster Protocol Strategy\n");
        report.push_str("1. **Use existing rust-src** for all rustc_* crates (no duplication)\n");
        report.push_str("2. **Current submodules** cover most external dependencies\n");
        report.push_str(&format!("3. **Add {} missing submodules** for 100% coverage\n", self.missing_count));
        report.push_str("4. **Apply Monster Protocol** to unified dependency tree\n");
        
        fs::write("RUSTC_RECURSIVE_DRY_RUN.md", &report)?;
        
        println!("  ✓ Report written to RUSTC_RECURSIVE_DRY_RUN.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Rustc Recursive Dependency Dry Run Analysis");
        
        self.scan_existing_submodules()?;
        self.analyze_rustc_dependencies()?;
        self.identify_missing_submodules();
        self.generate_dry_run_report()?;
        
        println!("\n🎯 === DRY RUN RESULTS ===");
        println!("  Rustc crates: {}", self.rustc_crates.len());
        println!("  Existing submodules: {}", self.existing_submodules.len());
        println!("  Missing submodules: {}", self.missing_count);
        
        let coverage = if self.missing_count > 0 {
            let total = self.existing_submodules.len() + self.missing_count as usize;
            (self.existing_submodules.len() as f64 / total as f64) * 100.0
        } else { 100.0 };
        
        println!("  Coverage: {:.1}%", coverage);
        
        if coverage > 95.0 {
            println!("  🎉 EXCELLENT coverage - ready for Monster Protocol!");
        } else if coverage > 80.0 {
            println!("  ✅ GOOD coverage - few missing submodules");
        } else {
            println!("  ⚠️ MODERATE coverage - need more submodules");
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = RustcRecursiveDryRun::new();
    analyzer.run()
}
