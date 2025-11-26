use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CargoTomlInfo {
    cargo_toml_path: String,
    crate_name: String,
    version: Option<String>,
    git_module_path: String,
    git_url: String,
    git_object: String,
}

struct CargoToGitMapper {
    mappings: Vec<CargoTomlInfo>,
    git_modules: HashMap<String, (String, String)>, // path -> (url, git_object)
}

impl CargoToGitMapper {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
            git_modules: HashMap::new(),
        }
    }
    
    fn load_git_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Loading git module information...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut current_path = String::new();
        let mut current_url = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                if !current_path.is_empty() && !current_url.is_empty() {
                    let git_object = self.get_git_object(&current_path);
                    self.git_modules.insert(current_path.clone(), (current_url.clone(), git_object));
                }
                current_path.clear();
                current_url.clear();
            } else if line.starts_with("path = ") {
                current_path = line.replace("path = ", "");
            } else if line.starts_with("url = ") {
                current_url = line.replace("url = ", "");
            }
        }
        
        if !current_path.is_empty() && !current_url.is_empty() {
            let git_object = self.get_git_object(&current_path);
            self.git_modules.insert(current_path, (current_url, git_object));
        }
        
        println!("  ✓ Loaded {} git modules", self.git_modules.len());
        Ok(())
    }
    
    fn scan_cargo_tomls(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning all Cargo.toml files...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                if let Some(mapping) = self.create_mapping(cargo_toml_path, &content) {
                    self.mappings.push(mapping);
                }
            }
        }
        
        println!("  ✓ Found {} Cargo.toml files", self.mappings.len());
        Ok(())
    }
    
    fn create_mapping(&self, cargo_toml_path: &str, content: &str) -> Option<CargoTomlInfo> {
        // Extract crate name and version
        let mut crate_name = None;
        let mut version = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("name = ") {
                crate_name = line.split('"').nth(1).map(|s| s.to_string());
            } else if line.starts_with("version = ") {
                version = line.split('"').nth(1).map(|s| s.to_string());
            }
        }
        
        // Find which git module contains this Cargo.toml
        let git_module_path = self.find_containing_git_module(cargo_toml_path)?;
        
        if let Some((git_url, git_object)) = self.git_modules.get(&git_module_path) {
            if let Some(name) = crate_name {
                return Some(CargoTomlInfo {
                    cargo_toml_path: cargo_toml_path.to_string(),
                    crate_name: name,
                    version,
                    git_module_path: git_module_path.clone(),
                    git_url: git_url.clone(),
                    git_object: git_object.clone(),
                });
            }
        }
        
        None
    }
    
    fn find_containing_git_module(&self, cargo_toml_path: &str) -> Option<String> {
        // Find the git module that contains this Cargo.toml file
        for git_module_path in self.git_modules.keys() {
            if cargo_toml_path.starts_with(&format!("../{}/", git_module_path)) {
                return Some(git_module_path.clone());
            }
        }
        None
    }
    
    fn get_git_object(&self, submodule_path: &str) -> String {
        let output = Command::new("git")
            .args(&["submodule", "status", submodule_path])
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
    
    fn generate_mapping_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating cargo-to-git mapping report...");
        
        let mut report = String::new();
        report.push_str("# Cargo.toml to Git Module Mapping Report\n\n");
        
        report.push_str("## Mapping Relationship Flow\n");
        report.push_str("cargo toml - in file in - git module\n\n");
        
        // Group by git module
        let mut by_git_module: HashMap<String, Vec<&CargoTomlInfo>> = HashMap::new();
        
        for mapping in &self.mappings {
            by_git_module
                .entry(mapping.git_module_path.clone())
                .or_insert_with(Vec::new)
                .push(mapping);
        }
        
        // Sort by number of Cargo.toml files
        let mut sorted_modules: Vec<_> = by_git_module.iter().collect();
        sorted_modules.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        
        report.push_str("## Cargo.toml Files by Git Module\n\n");
        
        for (git_module_path, cargo_tomls) in &sorted_modules {
            let git_name = git_module_path.replace("submodules/", "");
            
            report.push_str(&format!("### {} ({} Cargo.toml files)\n", git_name, cargo_tomls.len()));
            
            if let Some(first_mapping) = cargo_tomls.first() {
                report.push_str(&format!("- **Git Module Path**: `{}`\n", first_mapping.git_module_path));
                report.push_str(&format!("- **Git URL**: `{}`\n", first_mapping.git_url));
                report.push_str(&format!("- **Git Object**: `{}`\n", first_mapping.git_object));
            }
            
            report.push_str("- **Cargo.toml Files**:\n");
            
            for (i, mapping) in cargo_tomls.iter().enumerate() {
                if i < 10 { // Show first 10
                    let short_path = mapping.cargo_toml_path.replace("../", "");
                    report.push_str(&format!("  - **{}**\n", mapping.crate_name));
                    
                    if let Some(version) = &mapping.version {
                        report.push_str(&format!("    - Version: `{}`\n", version));
                    }
                    
                    report.push_str(&format!("    - File: `{}`\n", short_path));
                    report.push_str(&format!("    - **Relationship**: `{}` - **in file in** - `{}`\n", 
                        short_path, git_name));
                }
            }
            
            if cargo_tomls.len() > 10 {
                report.push_str(&format!("  - ... and {} more Cargo.toml files\n", cargo_tomls.len() - 10));
            }
            
            report.push_str("\n");
        }
        
        // Statistics
        report.push_str("## Mapping Statistics\n");
        report.push_str(&format!("- Total Cargo.toml files: {}\n", self.mappings.len()));
        report.push_str(&format!("- Git modules containing Cargo.toml: {}\n", by_git_module.len()));
        
        if by_git_module.len() > 0 {
            let avg_cargo_per_git = self.mappings.len() as f64 / by_git_module.len() as f64;
            report.push_str(&format!("- Average Cargo.toml files per git module: {:.1}\n", avg_cargo_per_git));
        }
        
        // Top git modules by Cargo.toml count
        report.push_str("\n## Git Modules with Most Cargo.toml Files\n");
        for (i, (git_module_path, cargo_tomls)) in sorted_modules.iter().enumerate() {
            if i < 10 {
                let git_name = git_module_path.replace("submodules/", "");
                report.push_str(&format!("{}. **{}** - {} Cargo.toml files\n", 
                    i + 1, git_name, cargo_tomls.len()));
            }
        }
        
        report.push_str("\n## RocksDB Cargo-to-Git Schema\n");
        report.push_str("```\n");
        report.push_str("Key: cargo_toml_path\n");
        report.push_str("Value: {\n");
        report.push_str("  crate_name: string,\n");
        report.push_str("  version: string,\n");
        report.push_str("  git_module_path: string,\n");
        report.push_str("  git_url: string,\n");
        report.push_str("  git_object: string,\n");
        report.push_str("  relationship: \"in_file_in\"\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("CARGO_TO_GIT_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to CARGO_TO_GIT_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Cargo-to-Git Mapper - cargo toml - in file in - git module");
        
        self.load_git_modules()?;
        self.scan_cargo_tomls()?;
        self.generate_mapping_report()?;
        
        // Group by git module for stats
        let mut by_git_module: HashMap<String, Vec<&CargoTomlInfo>> = HashMap::new();
        for mapping in &self.mappings {
            by_git_module
                .entry(mapping.git_module_path.clone())
                .or_insert_with(Vec::new)
                .push(mapping);
        }
        
        println!("\n🎯 === CARGO-TO-GIT MAPPING COMPLETE ===");
        println!("  Total Cargo.toml files: {}", self.mappings.len());
        println!("  Git modules with Cargo.toml: {}", by_git_module.len());
        
        if by_git_module.len() > 0 {
            let avg = self.mappings.len() as f64 / by_git_module.len() as f64;
            println!("  Average Cargo.toml per git: {:.1}", avg);
        }
        
        println!("\n🔗 FILE CONTAINMENT RELATIONSHIP: cargo toml - in file in - git module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = CargoToGitMapper::new();
    mapper.run()
}
