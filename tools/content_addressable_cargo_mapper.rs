use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CargoTomlContent {
    content_hash: String,
    name: String,
    version: String,
    locations: Vec<CargoLocation>,
}

#[derive(Debug, Clone)]
struct CargoLocation {
    git_repo: String,
    git_object: String,
    cargo_toml_path: String,
    git_log: Vec<String>,
}

struct ContentAddressableCargoMapper {
    content_map: HashMap<String, CargoTomlContent>, // content_hash -> content info
}

impl ContentAddressableCargoMapper {
    fn new() -> Self {
        Self {
            content_map: HashMap::new(),
        }
    }
    
    fn scan_all_cargo_tomls(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Scanning all Cargo.toml files for content addressing...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                self.process_cargo_toml(cargo_toml_path, &content)?;
            }
        }
        
        println!("  ✓ Processed {} unique content hashes", self.content_map.len());
        Ok(())
    }
    
    fn process_cargo_toml(&mut self, cargo_toml_path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content_hash = self.calculate_content_hash(content);
        let (name, version) = self.extract_name_version(content);
        let git_repo = self.extract_git_repo(cargo_toml_path);
        let git_object = self.get_git_object(&git_repo);
        let git_log = self.get_git_log(cargo_toml_path)?;
        
        let location = CargoLocation {
            git_repo,
            git_object,
            cargo_toml_path: cargo_toml_path.to_string(),
            git_log,
        };
        
        self.content_map
            .entry(content_hash.clone())
            .or_insert_with(|| CargoTomlContent {
                content_hash: content_hash.clone(),
                name,
                version,
                locations: Vec::new(),
            })
            .locations
            .push(location);
        
        Ok(())
    }
    
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Normalize content for consistent hashing
        let normalized = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        
        let mut hasher = DefaultHasher::new();
        normalized.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    fn extract_name_version(&self, content: &str) -> (String, String) {
        let mut name = "unknown".to_string();
        let mut version = "unknown".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("name = ") {
                if let Some(n) = line.split('"').nth(1) {
                    name = n.to_string();
                }
            } else if line.starts_with("version = ") {
                if let Some(v) = line.split('"').nth(1) {
                    version = v.to_string();
                }
            }
        }
        
        (name, version)
    }
    
    fn extract_git_repo(&self, cargo_toml_path: &str) -> String {
        if let Some(submodules_pos) = cargo_toml_path.find("submodules/") {
            let after_submodules = &cargo_toml_path[submodules_pos + 11..];
            if let Some(slash_pos) = after_submodules.find('/') {
                return after_submodules[..slash_pos].to_string();
            }
        }
        "unknown".to_string()
    }
    
    fn get_git_object(&self, git_repo: &str) -> String {
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
    
    fn get_git_log(&self, cargo_toml_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["log", "--oneline", "-5", cargo_toml_path])
            .output();
        
        if let Ok(result) = output {
            if result.status.success() {
                let log_output = String::from_utf8_lossy(&result.stdout);
                return Ok(log_output.lines().map(|s| s.to_string()).collect());
            }
        }
        
        Ok(vec!["No git log available".to_string()])
    }
    
    fn find_duplicate_content(&self) -> Vec<&CargoTomlContent> {
        self.content_map.values()
            .filter(|content| content.locations.len() > 1)
            .collect()
    }
    
    fn generate_content_addressable_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating content-addressable Cargo.toml report...");
        
        let duplicates = self.find_duplicate_content();
        
        let mut report = String::new();
        report.push_str("# Content-Addressable Cargo.toml Mapping Report\n\n");
        
        report.push_str("## Content Addressing Flow\n");
        report.push_str("git obj -> cargo.toml -> name and version -> content hash -> identical content across repos\n\n");
        
        report.push_str("## Identical Content Across Repositories\n\n");
        
        for content in &duplicates {
            report.push_str(&format!("### {} v{} (Content Hash: {})\n", 
                content.name, content.version, &content.content_hash[..8]));
            
            report.push_str(&format!("- **Content Hash**: `{}`\n", content.content_hash));
            report.push_str(&format!("- **Package**: `{}` v`{}`\n", content.name, content.version));
            report.push_str(&format!("- **Found in {} repositories**:\n", content.locations.len()));
            
            for (i, location) in content.locations.iter().enumerate() {
                report.push_str(&format!("  {}. **{}**\n", i + 1, location.git_repo));
                report.push_str(&format!("     - Git Object: `{}`\n", location.git_object));
                
                let short_path = location.cargo_toml_path.replace("../", "");
                report.push_str(&format!("     - Path: `{}`\n", short_path));
                
                if !location.git_log.is_empty() {
                    report.push_str("     - Recent Git Log:\n");
                    for (j, log_entry) in location.git_log.iter().enumerate() {
                        if j < 3 { // Show first 3 log entries
                            report.push_str(&format!("       - `{}`\n", log_entry));
                        }
                    }
                }
            }
            
            report.push_str(&format!("- **Content Identity**: Same `{}` v`{}` exists in {} different git objects\n", 
                content.name, content.version, content.locations.len()));
            
            report.push_str("\n");
        }
        
        // Unique content
        let unique_content: Vec<_> = self.content_map.values()
            .filter(|content| content.locations.len() == 1)
            .collect();
        
        report.push_str("## Content Statistics\n");
        report.push_str(&format!("- Total unique content hashes: {}\n", self.content_map.len()));
        report.push_str(&format!("- Identical content across repos: {}\n", duplicates.len()));
        report.push_str(&format!("- Unique content (single repo): {}\n", unique_content.len()));
        
        let total_locations: usize = self.content_map.values()
            .map(|c| c.locations.len())
            .sum();
        
        report.push_str(&format!("- Total Cargo.toml files: {}\n", total_locations));
        
        if duplicates.len() > 0 {
            let duplicate_instances: usize = duplicates.iter()
                .map(|c| c.locations.len())
                .sum();
            
            let deduplication_savings = duplicate_instances - duplicates.len();
            report.push_str(&format!("- Deduplication savings: {} files\n", deduplication_savings));
        }
        
        // Top duplicated content
        let mut sorted_duplicates = duplicates;
        sorted_duplicates.sort_by(|a, b| b.locations.len().cmp(&a.locations.len()));
        
        report.push_str("\n## Most Duplicated Content\n");
        for (i, content) in sorted_duplicates.iter().enumerate() {
            if i < 10 {
                report.push_str(&format!("{}. **{}** v{} - {} copies\n", 
                    i + 1, content.name, content.version, content.locations.len()));
            }
        }
        
        report.push_str("\n## Content-Addressable Storage Schema\n");
        report.push_str("```\n");
        report.push_str("Key: content_hash\n");
        report.push_str("Value: {\n");
        report.push_str("  name: string,\n");
        report.push_str("  version: string,\n");
        report.push_str("  locations: [{\n");
        report.push_str("    git_repo: string,\n");
        report.push_str("    git_object: string,\n");
        report.push_str("    cargo_toml_path: string,\n");
        report.push_str("    git_log: [string]\n");
        report.push_str("  }]\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("CONTENT_ADDRESSABLE_CARGO.md", &report)?;
        
        println!("  ✓ Report written to CONTENT_ADDRESSABLE_CARGO.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Content-Addressable Cargo.toml Mapper");
        
        self.scan_all_cargo_tomls()?;
        self.generate_content_addressable_report()?;
        
        let duplicates = self.find_duplicate_content();
        let total_files: usize = self.content_map.values()
            .map(|c| c.locations.len())
            .sum();
        
        println!("\n🎯 === CONTENT-ADDRESSABLE MAPPING COMPLETE ===");
        println!("  Unique content hashes: {}", self.content_map.len());
        println!("  Total Cargo.toml files: {}", total_files);
        println!("  Identical content across repos: {}", duplicates.len());
        
        if duplicates.len() > 0 {
            let duplicate_instances: usize = duplicates.iter()
                .map(|c| c.locations.len())
                .sum();
            
            let savings = duplicate_instances - duplicates.len();
            println!("  Deduplication savings: {} files", savings);
        }
        
        println!("\n🔗 CONTENT ADDRESSING: git obj -> cargo.toml -> content hash -> identical across repos");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = ContentAddressableCargoMapper::new();
    mapper.run()
}
