use std::fs;
use std::process::Command;
use std::collections::{HashMap, HashSet, VecDeque};

struct RecursiveGitUrlChecker {
    rustc_crates: HashMap<String, Vec<String>>, // crate -> dependencies
    git_urls: HashMap<String, String>,          // crate -> git_url
    submodule_urls: HashSet<String>,           // existing submodule urls
    missing_urls: Vec<String>,                 // git urls not in submodules
    processed: HashSet<String>,                // avoid cycles
}

impl RecursiveGitUrlChecker {
    fn new() -> Self {
        Self {
            rustc_crates: HashMap::new(),
            git_urls: HashMap::new(),
            submodule_urls: HashSet::new(),
            missing_urls: Vec::new(),
            processed: HashSet::new(),
        }
    }
    
    fn load_rustc_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 Loading rustc dependencies...");
        
        let rust_src = "/mnt/data1/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
        let output = Command::new("find")
            .args(&[rust_src, "-name", "Cargo.toml"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        for toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(toml_path) {
                if let Some(crate_name) = self.extract_crate_name(&content) {
                    let deps = self.extract_dependencies(&content);
                    self.rustc_crates.insert(crate_name, deps);
                }
            }
        }
        
        println!("  ✓ Loaded {} rustc crates", self.rustc_crates.len());
        Ok(())
    }
    
    fn load_cargo_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Loading cargo metadata for git URLs...");
        
        let metadata = fs::read_to_string("cargo_metadata_db.json")?;
        let mut git_count = 0;
        
        // Simple JSON parsing for repository URLs
        for line in metadata.lines() {
            if line.contains("\"repository\":") {
                if let Some(url) = self.extract_git_url_from_json(line) {
                    // Extract crate name from context (previous lines)
                    if let Some(crate_name) = self.find_crate_name_in_context(&metadata, line) {
                        self.git_urls.insert(crate_name, url);
                        git_count += 1;
                    }
                }
            }
        }
        
        println!("  ✓ Found {} git URLs from metadata", git_count);
        Ok(())
    }
    
    fn load_existing_submodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📂 Loading existing submodule URLs...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        for line in gitmodules.lines() {
            if line.trim().starts_with("url = ") {
                if let Some(url) = line.split('=').nth(1) {
                    let clean_url = url.trim().replace(".git", "");
                    self.submodule_urls.insert(clean_url);
                }
            }
        }
        
        println!("  ✓ Loaded {} existing submodule URLs", self.submodule_urls.len());
        Ok(())
    }
    
    fn recursive_dependency_check(&mut self) {
        println!("🔄 Recursively checking all rustc dependencies...");
        
        let mut queue = VecDeque::new();
        
        // Start with all rustc crates
        for crate_name in self.rustc_crates.keys() {
            queue.push_back(crate_name.clone());
        }
        
        while let Some(crate_name) = queue.pop_front() {
            if self.processed.contains(&crate_name) {
                continue;
            }
            
            self.processed.insert(crate_name.clone());
            
            // Check if this crate has a git URL
            if let Some(git_url) = self.git_urls.get(&crate_name) {
                if !self.is_url_in_submodules(git_url) {
                    self.missing_urls.push(git_url.clone());
                }
            }
            
            // Add dependencies to queue
            if let Some(deps) = self.rustc_crates.get(&crate_name) {
                for dep in deps {
                    if !self.processed.contains(dep) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
        
        println!("  ✓ Processed {} crates recursively", self.processed.len());
        println!("  ✗ Found {} missing git URLs", self.missing_urls.len());
    }
    
    fn is_url_in_submodules(&self, git_url: &str) -> bool {
        let clean_url = git_url.replace(".git", "");
        
        for submodule_url in &self.submodule_urls {
            if submodule_url.contains(&clean_url) || clean_url.contains(submodule_url) {
                return true;
            }
        }
        false
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
            
            if line.starts_with("[dependencies") {
                in_deps = true;
                continue;
            }
            
            if line.starts_with('[') && in_deps {
                in_deps = false;
            }
            
            if in_deps && line.contains('=') && !line.starts_with('#') {
                if let Some(dep_name) = line.split('=').next() {
                    let clean_dep = dep_name.trim().replace('"', "");
                    if !clean_dep.is_empty() {
                        deps.push(clean_dep);
                    }
                }
            }
        }
        
        deps
    }
    
    fn extract_git_url_from_json(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("\"repository\":\"") {
            let start = start + 14; // Length of "repository":"
            if let Some(end) = line[start..].find('"') {
                let url = &line[start..start + end];
                if url.contains("github.com") {
                    return Some(url.to_string());
                }
            }
        }
        None
    }
    
    fn find_crate_name_in_context(&self, metadata: &str, target_line: &str) -> Option<String> {
        let lines: Vec<&str> = metadata.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if *line == target_line && i > 0 {
                // Look backwards for crate name
                for j in (0..i).rev() {
                    if lines[j].contains("\"name\":\"") {
                        if let Some(start) = lines[j].find("\"name\":\"") {
                            let start = start + 8;
                            if let Some(end) = lines[j][start..].find('"') {
                                return Some(lines[j][start..start + end].to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    
    fn generate_missing_urls_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Generating missing URLs report...");
        
        let mut report = String::new();
        report.push_str("# Recursive Git URL Analysis Report\n\n");
        
        report.push_str("## Summary\n");
        report.push_str(&format!("- Rustc crates analyzed: {}\n", self.rustc_crates.len()));
        report.push_str(&format!("- Git URLs found: {}\n", self.git_urls.len()));
        report.push_str(&format!("- Existing submodules: {}\n", self.submodule_urls.len()));
        report.push_str(&format!("- Missing git URLs: {}\n", self.missing_urls.len()));
        report.push_str(&format!("- Crates processed recursively: {}\n", self.processed.len()));
        
        let coverage = if self.git_urls.len() > 0 {
            ((self.git_urls.len() - self.missing_urls.len()) as f64 / self.git_urls.len() as f64) * 100.0
        } else { 0.0 };
        report.push_str(&format!("- Submodule coverage: {:.1}%\n", coverage));
        
        report.push_str("\n## Missing Git URLs\n");
        report.push_str("These repositories need to be added as submodules:\n\n");
        
        for (i, url) in self.missing_urls.iter().enumerate() {
            if i < 20 { // Show first 20
                report.push_str(&format!("- {}\n", url));
            }
        }
        
        if self.missing_urls.len() > 20 {
            report.push_str(&format!("- ... and {} more\n", self.missing_urls.len() - 20));
        }
        
        report.push_str("\n## Next Steps\n");
        report.push_str("1. Add missing repositories as git submodules\n");
        report.push_str("2. Update Monster Protocol to use complete dependency tree\n");
        report.push_str("3. Re-run recursive analysis to verify coverage\n");
        
        fs::write("RECURSIVE_GIT_URL_ANALYSIS.md", &report)?;
        
        println!("  ✓ Report written to RECURSIVE_GIT_URL_ANALYSIS.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Recursive Git URL Checker for Rustc Dependencies");
        
        self.load_rustc_dependencies()?;
        self.load_cargo_metadata()?;
        self.load_existing_submodules()?;
        self.recursive_dependency_check();
        self.generate_missing_urls_report()?;
        
        println!("\n🎯 === RECURSIVE ANALYSIS COMPLETE ===");
        println!("  Rustc crates: {}", self.rustc_crates.len());
        println!("  Git URLs found: {}", self.git_urls.len());
        println!("  Existing submodules: {}", self.submodule_urls.len());
        println!("  Missing URLs: {}", self.missing_urls.len());
        
        let coverage = if self.git_urls.len() > 0 {
            ((self.git_urls.len() - self.missing_urls.len()) as f64 / self.git_urls.len() as f64) * 100.0
        } else { 0.0 };
        
        println!("  Coverage: {:.1}%", coverage);
        
        if coverage > 90.0 {
            println!("  🎉 EXCELLENT coverage - Monster Protocol ready!");
        } else if coverage > 70.0 {
            println!("  ✅ GOOD coverage - few missing submodules");
        } else {
            println!("  ⚠️ MODERATE coverage - need more submodules");
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut checker = RecursiveGitUrlChecker::new();
    checker.run()
}
