use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CargoModule {
    name: String,
    cargo_toml_path: String,
    version: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct GitToCargoMapping {
    git_module_path: String,
    git_url: String,
    git_object: String,
    cargo_modules: Vec<CargoModule>,
}

struct GitToCargoMapper {
    mappings: HashMap<String, GitToCargoMapping>,
}

impl GitToCargoMapper {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }
    
    fn scan_git_modules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Scanning git modules for cargo definitions...");
        
        // Parse .gitmodules to get git module info
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut current_path = String::new();
        let mut current_url = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                // Process previous entry
                if !current_path.is_empty() && !current_url.is_empty() {
                    self.process_git_module(&current_path, &current_url)?;
                }
                current_path.clear();
                current_url.clear();
            } else if line.starts_with("path = ") {
                current_path = line.replace("path = ", "");
            } else if line.starts_with("url = ") {
                current_url = line.replace("url = ", "");
            }
        }
        
        // Don't forget the last entry
        if !current_path.is_empty() && !current_url.is_empty() {
            self.process_git_module(&current_path, &current_url)?;
        }
        
        println!("  ✓ Processed {} git modules", self.mappings.len());
        Ok(())
    }
    
    fn process_git_module(&mut self, path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let git_object = self.get_git_object(path);
        let cargo_modules = self.find_cargo_modules(path)?;
        
        if !cargo_modules.is_empty() {
            self.mappings.insert(path.to_string(), GitToCargoMapping {
                git_module_path: path.to_string(),
                git_url: url.to_string(),
                git_object,
                cargo_modules,
            });
        }
        
        Ok(())
    }
    
    fn find_cargo_modules(&self, git_module_path: &str) -> Result<Vec<CargoModule>, Box<dyn std::error::Error>> {
        let full_path = format!("../{}", git_module_path);
        
        // Find all Cargo.toml files in this git module
        let output = Command::new("find")
            .args(&[&full_path, "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(Vec::new());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        let mut cargo_modules = Vec::new();
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                if let Some(cargo_module) = self.parse_cargo_toml(&content, cargo_toml_path) {
                    cargo_modules.push(cargo_module);
                }
            }
        }
        
        Ok(cargo_modules)
    }
    
    fn parse_cargo_toml(&self, content: &str, path: &str) -> Option<CargoModule> {
        let mut name = None;
        let mut version = None;
        let mut description = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("name = ") {
                name = line.split('"').nth(1).map(|s| s.to_string());
            } else if line.starts_with("version = ") {
                version = line.split('"').nth(1).map(|s| s.to_string());
            } else if line.starts_with("description = ") {
                description = line.split('"').nth(1).map(|s| s.to_string());
            }
        }
        
        if let Some(cargo_name) = name {
            Some(CargoModule {
                name: cargo_name,
                cargo_toml_path: path.to_string(),
                version,
                description,
            })
        } else {
            None
        }
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
        println!("📊 Generating git-to-cargo mapping report...");
        
        let mut report = String::new();
        report.push_str("# Git Module to Cargo Module Mapping Report\n\n");
        
        report.push_str("## Mapping Relationship Flow\n");
        report.push_str("git module - defined in repo of -> cargo module\n\n");
        
        report.push_str("## Git Module to Cargo Module Mappings\n\n");
        
        // Sort by number of cargo modules (most productive repos first)
        let mut sorted_mappings: Vec<_> = self.mappings.iter().collect();
        sorted_mappings.sort_by(|a, b| b.1.cargo_modules.len().cmp(&a.1.cargo_modules.len()));
        
        for (git_path, mapping) in &sorted_mappings {
            let git_name = git_path.replace("submodules/", "");
            
            report.push_str(&format!("### {} ({} cargo modules)\n", git_name, mapping.cargo_modules.len()));
            report.push_str(&format!("- **Git Module Path**: `{}`\n", mapping.git_module_path));
            report.push_str(&format!("- **Git URL**: `{}`\n", mapping.git_url));
            report.push_str(&format!("- **Git Object**: `{}`\n", mapping.git_object));
            
            report.push_str("- **Defined Cargo Modules**:\n");
            for cargo_module in &mapping.cargo_modules {
                report.push_str(&format!("  - **{}**\n", cargo_module.name));
                
                if let Some(version) = &cargo_module.version {
                    report.push_str(&format!("    - Version: `{}`\n", version));
                }
                
                if let Some(description) = &cargo_module.description {
                    report.push_str(&format!("    - Description: {}\n", description));
                }
                
                let short_path = cargo_module.cargo_toml_path.replace("../", "");
                report.push_str(&format!("    - Cargo.toml: `{}`\n", short_path));
                
                report.push_str(&format!("    - **Relationship**: `{}` - **defined in repo of** -> `{}`\n", 
                    git_name, cargo_module.name));
            }
            
            report.push_str("\n");
        }
        
        // Statistics
        let total_cargo_modules: usize = self.mappings.values()
            .map(|m| m.cargo_modules.len())
            .sum();
        
        report.push_str("## Mapping Statistics\n");
        report.push_str(&format!("- Git modules with cargo definitions: {}\n", self.mappings.len()));
        report.push_str(&format!("- Total cargo modules defined: {}\n", total_cargo_modules));
        
        if self.mappings.len() > 0 {
            let avg_cargo_per_git = total_cargo_modules as f64 / self.mappings.len() as f64;
            report.push_str(&format!("- Average cargo modules per git repo: {:.1}\n", avg_cargo_per_git));
        }
        
        // Top productive repositories
        report.push_str("\n## Most Productive Git Repositories\n");
        for (i, (git_path, mapping)) in sorted_mappings.iter().enumerate() {
            if i < 10 {
                let git_name = git_path.replace("submodules/", "");
                report.push_str(&format!("{}. **{}** - {} cargo modules\n", 
                    i + 1, git_name, mapping.cargo_modules.len()));
            }
        }
        
        report.push_str("\n## RocksDB Git-to-Cargo Schema\n");
        report.push_str("```\n");
        report.push_str("Key: git_module_path:git_object\n");
        report.push_str("Value: {\n");
        report.push_str("  git_url: string,\n");
        report.push_str("  git_object: string,\n");
        report.push_str("  cargo_modules: [{\n");
        report.push_str("    name: string,\n");
        report.push_str("    version: string,\n");
        report.push_str("    description: string,\n");
        report.push_str("    cargo_toml_path: string\n");
        report.push_str("  }],\n");
        report.push_str("  relationship: \"defined_in_repo_of\"\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("GIT_TO_CARGO_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to GIT_TO_CARGO_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Git-to-Cargo Mapper - git module - defined in repo of -> cargo module");
        
        self.scan_git_modules()?;
        self.generate_mapping_report()?;
        
        let total_cargo_modules: usize = self.mappings.values()
            .map(|m| m.cargo_modules.len())
            .sum();
        
        println!("\n🎯 === GIT-TO-CARGO MAPPING COMPLETE ===");
        println!("  Git modules with cargo: {}", self.mappings.len());
        println!("  Total cargo modules: {}", total_cargo_modules);
        
        if self.mappings.len() > 0 {
            let avg = total_cargo_modules as f64 / self.mappings.len() as f64;
            println!("  Average cargo per git: {:.1}", avg);
        }
        
        println!("\n🔗 DEFINITION RELATIONSHIP: git module - defined in repo of -> cargo module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = GitToCargoMapper::new();
    mapper.run()
}
