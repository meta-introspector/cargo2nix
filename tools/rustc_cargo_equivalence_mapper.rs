use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct EquivalenceMapping {
    rustc_crate: String,
    cargo_module: String,
    git_repo: String,
    cargo_toml_path: String,
    git_object: String,
    confidence: f64, // 0.0 to 1.0
}

struct RustcCargoEquivalenceMapper {
    rustc_crates: Vec<String>,
    cargo_modules: HashMap<String, (String, String, String)>, // name -> (git_repo, path, git_object)
    equivalences: Vec<EquivalenceMapping>,
}

impl RustcCargoEquivalenceMapper {
    fn new() -> Self {
        Self {
            rustc_crates: Vec::new(),
            cargo_modules: HashMap::new(),
            equivalences: Vec::new(),
        }
    }
    
    fn load_rustc_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 Loading rustc crates...");
        
        // Load from dry run report
        if let Ok(report) = fs::read_to_string("RUSTC_RECURSIVE_DRY_RUN.md") {
            for line in report.lines() {
                if line.starts_with("- `") && line.contains("` → https://github.com/") {
                    if let Some(crate_name) = line.strip_prefix("- `").and_then(|s| s.split('`').next()) {
                        self.rustc_crates.push(crate_name.to_string());
                    }
                }
            }
        }
        
        println!("  ✓ Loaded {} rustc crates", self.rustc_crates.len());
        Ok(())
    }
    
    fn load_cargo_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Loading cargo modules from git repos...");
        
        // Load from cargo dependency mapping
        if let Ok(report) = fs::read_to_string("CARGO_DEPENDENCY_MAPPING.md") {
            let mut current_git_repo = String::new();
            
            for line in report.lines() {
                // Extract git module from headers like "### bench (juniper)"
                if line.starts_with("### ") && line.contains(" (") && line.contains(")") {
                    if let Some(start) = line.find(" (") {
                        if let Some(end) = line.find(")") {
                            current_git_repo = line[start + 2..end].to_string();
                        }
                    }
                }
                
                // Extract cargo module name and path
                if line.contains("**Cargo.toml**: `") {
                    if let Some(start) = line.find("`") {
                        if let Some(end) = line[start + 1..].find("`") {
                            let cargo_toml_path = line[start + 1..start + 1 + end].to_string();
                            
                            // Extract module name from the header
                            if let Some(header_line) = self.find_previous_header(&report, line) {
                                if let Some(module_name) = self.extract_module_name(&header_line) {
                                    let git_object = self.get_git_object(&current_git_repo);
                                    self.cargo_modules.insert(
                                        module_name,
                                        (current_git_repo.clone(), cargo_toml_path, git_object)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("  ✓ Loaded {} cargo modules", self.cargo_modules.len());
        Ok(())
    }
    
    fn find_previous_header(&self, report: &str, target_line: &str) -> Option<String> {
        let lines: Vec<&str> = report.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if *line == target_line && i > 0 {
                // Look backwards for header
                for j in (0..i).rev() {
                    if lines[j].starts_with("### ") {
                        return Some(lines[j].to_string());
                    }
                }
            }
        }
        None
    }
    
    fn extract_module_name(&self, header: &str) -> Option<String> {
        // Extract from "### bench (juniper)"
        if let Some(start) = header.find("### ") {
            let after_hash = &header[start + 4..];
            if let Some(end) = after_hash.find(" (") {
                return Some(after_hash[..end].to_string());
            }
        }
        None
    }
    
    fn get_git_object(&self, git_repo: &str) -> String {
        use std::process::Command;
        
        let submodule_path = format!("submodules/{}", git_repo);
        
        let output = Command::new("git")
            .args(&["submodule", "status", &submodule_path])
            .current_dir("..")
            .output();
        
        if let Ok(result) = output {
            if result.status.success() {
                let status_line = String::from_utf8_lossy(&result.stdout);
                if let Some(hash) = status_line.split_whitespace().next() {
                    return hash.trim_start_matches(' ').to_string();
                }
            }
        }
        
        "unknown".to_string()
    }
    
    fn find_equivalences(&mut self) {
        println!("🔍 Finding rustc = cargo module equivalences...");
        
        for rustc_crate in &self.rustc_crates {
            for (cargo_module, (git_repo, cargo_toml_path, git_object)) in &self.cargo_modules {
                let confidence = self.calculate_confidence(rustc_crate, cargo_module);
                
                if confidence > 0.5 { // Only include likely matches
                    self.equivalences.push(EquivalenceMapping {
                        rustc_crate: rustc_crate.clone(),
                        cargo_module: cargo_module.clone(),
                        git_repo: git_repo.clone(),
                        cargo_toml_path: cargo_toml_path.clone(),
                        git_object: git_object.clone(),
                        confidence,
                    });
                }
            }
        }
        
        // Sort by confidence
        self.equivalences.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        println!("  ✓ Found {} equivalence mappings", self.equivalences.len());
    }
    
    fn calculate_confidence(&self, rustc_crate: &str, cargo_module: &str) -> f64 {
        // Exact match
        if rustc_crate == cargo_module {
            return 1.0;
        }
        
        // Normalize names for comparison
        let rustc_norm = rustc_crate.to_lowercase().replace("-", "_");
        let cargo_norm = cargo_module.to_lowercase().replace("-", "_");
        
        if rustc_norm == cargo_norm {
            return 0.95;
        }
        
        // Substring matches
        if rustc_norm.contains(&cargo_norm) || cargo_norm.contains(&rustc_norm) {
            return 0.8;
        }
        
        // Common prefixes/suffixes
        if rustc_norm.starts_with(&cargo_norm) || cargo_norm.starts_with(&rustc_norm) {
            return 0.7;
        }
        
        if rustc_norm.ends_with(&cargo_norm) || cargo_norm.ends_with(&rustc_norm) {
            return 0.7;
        }
        
        // Levenshtein-like similarity
        let similarity = self.string_similarity(&rustc_norm, &cargo_norm);
        if similarity > 0.7 {
            return similarity * 0.6;
        }
        
        0.0
    }
    
    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let len1 = s1.len();
        let len2 = s2.len();
        
        if len1 == 0 || len2 == 0 {
            return 0.0;
        }
        
        let max_len = len1.max(len2);
        let common_chars = s1.chars()
            .filter(|c| s2.contains(*c))
            .count();
        
        common_chars as f64 / max_len as f64
    }
    
    fn generate_equivalence_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating rustc = cargo module equivalence report...");
        
        let mut report = String::new();
        report.push_str("# Rustc = Cargo Module Equivalence Mapping Report\n\n");
        
        report.push_str("## Equivalence Relationship Flow\n");
        report.push_str("rustc = cargo module in git repos\n\n");
        
        report.push_str("## Rustc to Cargo Module Equivalences\n\n");
        
        for equivalence in &self.equivalences {
            let confidence_percent = (equivalence.confidence * 100.0) as u32;
            
            report.push_str(&format!("### {} = {} ({confidence_percent}% confidence)\n", 
                equivalence.rustc_crate, equivalence.cargo_module));
            
            report.push_str(&format!("- **Rustc Crate**: `{}`\n", equivalence.rustc_crate));
            report.push_str(&format!("- **Cargo Module**: `{}`\n", equivalence.cargo_module));
            report.push_str(&format!("- **Git Repository**: `{}`\n", equivalence.git_repo));
            report.push_str(&format!("- **Cargo.toml Path**: `{}`\n", equivalence.cargo_toml_path));
            report.push_str(&format!("- **Git Object**: `{}`\n", equivalence.git_object));
            report.push_str(&format!("- **Confidence**: {:.1}%\n", equivalence.confidence * 100.0));
            
            report.push_str(&format!("- **Equivalence**: `{}` = `{}` in `{}`\n", 
                equivalence.rustc_crate, equivalence.cargo_module, equivalence.git_repo));
            
            report.push_str("\n");
        }
        
        // Statistics
        let high_confidence = self.equivalences.iter().filter(|e| e.confidence > 0.9).count();
        let medium_confidence = self.equivalences.iter().filter(|e| e.confidence > 0.7 && e.confidence <= 0.9).count();
        let low_confidence = self.equivalences.iter().filter(|e| e.confidence <= 0.7).count();
        
        report.push_str("## Equivalence Statistics\n");
        report.push_str(&format!("- Total rustc crates: {}\n", self.rustc_crates.len()));
        report.push_str(&format!("- Total cargo modules: {}\n", self.cargo_modules.len()));
        report.push_str(&format!("- Total equivalences found: {}\n", self.equivalences.len()));
        report.push_str(&format!("- High confidence (>90%): {}\n", high_confidence));
        report.push_str(&format!("- Medium confidence (70-90%): {}\n", medium_confidence));
        report.push_str(&format!("- Low confidence (<70%): {}\n", low_confidence));
        
        let coverage = if self.rustc_crates.len() > 0 {
            (self.equivalences.len() as f64 / self.rustc_crates.len() as f64) * 100.0
        } else { 0.0 };
        
        report.push_str(&format!("- Coverage: {:.1}%\n", coverage));
        
        // Top confidence matches
        report.push_str("\n## Highest Confidence Equivalences\n");
        for (i, equivalence) in self.equivalences.iter().enumerate() {
            if i < 10 {
                report.push_str(&format!("{}. **{}** = **{}** ({:.1}%)\n", 
                    i + 1, equivalence.rustc_crate, equivalence.cargo_module, 
                    equivalence.confidence * 100.0));
            }
        }
        
        fs::write("RUSTC_CARGO_EQUIVALENCE.md", &report)?;
        
        println!("  ✓ Report written to RUSTC_CARGO_EQUIVALENCE.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Rustc = Cargo Module Equivalence Mapper");
        
        self.load_rustc_crates()?;
        self.load_cargo_modules()?;
        self.find_equivalences();
        self.generate_equivalence_report()?;
        
        let high_confidence = self.equivalences.iter().filter(|e| e.confidence > 0.9).count();
        
        println!("\n🎯 === RUSTC = CARGO EQUIVALENCE MAPPING COMPLETE ===");
        println!("  Rustc crates: {}", self.rustc_crates.len());
        println!("  Cargo modules: {}", self.cargo_modules.len());
        println!("  Equivalences found: {}", self.equivalences.len());
        println!("  High confidence matches: {}", high_confidence);
        
        let coverage = if self.rustc_crates.len() > 0 {
            (self.equivalences.len() as f64 / self.rustc_crates.len() as f64) * 100.0
        } else { 0.0 };
        
        println!("  Coverage: {:.1}%", coverage);
        
        println!("\n🔗 EQUIVALENCE RELATIONSHIP: rustc = cargo module in git repos");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = RustcCargoEquivalenceMapper::new();
    mapper.run()
}
