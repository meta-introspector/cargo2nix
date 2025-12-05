use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ContentEntry {
    content_hash: String,
    name: String,
    version: String,
    git_repo: String,
    git_object: String,
    cargo_toml_path: String,
}

struct EnhancedContentMapper {
    entries: Vec<ContentEntry>,
}

impl EnhancedContentMapper {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    fn scan_all_cargo_tomls(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Enhanced scanning of all Cargo.toml files...");
        
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if !output.status.success() {
            return Ok(());
        }
        
        let cargo_files = String::from_utf8_lossy(&output.stdout);
        
        for cargo_toml_path in cargo_files.lines() {
            if let Ok(content) = fs::read_to_string(cargo_toml_path) {
                if let Some(entry) = self.create_content_entry(cargo_toml_path, &content) {
                    self.entries.push(entry);
                }
            }
        }
        
        println!("  ✓ Processed {} Cargo.toml files", self.entries.len());
        Ok(())
    }
    
    fn create_content_entry(&self, cargo_toml_path: &str, content: &str) -> Option<ContentEntry> {
        let content_hash = self.calculate_simple_hash(content);
        let (name, version) = self.extract_name_version(content);
        let git_repo = self.extract_git_repo(cargo_toml_path);
        let git_object = self.get_git_object(&git_repo);
        
        Some(ContentEntry {
            content_hash,
            name,
            version,
            git_repo,
            git_object,
            cargo_toml_path: cargo_toml_path.to_string(),
        })
    }
    
    fn calculate_simple_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
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
    
    fn generate_enhanced_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating enhanced content-addressable report...");
        
        let mut report = String::new();
        report.push_str("# Enhanced Content-Addressable Cargo.toml Report\n\n");
        
        report.push_str("## Content Addressing Flow\n");
        report.push_str("git obj -> cargo.toml -> name and version -> content hash -> merge by content address\n\n");
        
        report.push_str("## Git Object to Content Hash Mappings\n\n");
        
        for (i, entry) in self.entries.iter().enumerate() {
            if i < 20 { // Show first 20
                report.push_str(&format!("### {} v{} in {}\n", entry.name, entry.version, entry.git_repo));
                report.push_str(&format!("- **Git Object**: `{}`\n", &entry.git_object[..16]));
                report.push_str(&format!("- **Content Hash**: `{}`\n", &entry.content_hash[..16]));
                
                let short_path = entry.cargo_toml_path.replace("../", "");
                report.push_str(&format!("- **Path**: `{}`\n", short_path));
                
                report.push_str(&format!("- **Mapping**: `{}` -> `{}` -> **{}** v{}\n", 
                    &entry.git_object[..8], 
                    &entry.content_hash[..8], 
                    entry.name, 
                    entry.version));
                
                report.push_str("\n");
            }
        }
        
        if self.entries.len() > 20 {
            report.push_str(&format!("... and {} more mappings\n\n", self.entries.len() - 20));
        }
        
        // Content hash analysis
        let mut content_groups: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_groups.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates: Vec<_> = content_groups.values().filter(|v| v.len() > 1).collect();
        
        report.push_str("## Content Hash Analysis\n");
        report.push_str(&format!("- Total Cargo.toml files: {}\n", self.entries.len()));
        report.push_str(&format!("- Unique content hashes: {}\n", content_groups.len()));
        report.push_str(&format!("- Identical content groups: {}\n", duplicates.len()));
        
        if !duplicates.is_empty() {
            report.push_str("\n## Identical Content Across Repositories\n");
            for (i, group) in duplicates.iter().enumerate() {
                if i < 5 { // Show first 5 duplicate groups
                    if let Some(first) = group.first() {
                        report.push_str(&format!("### Content Hash: {}\n", &first.content_hash[..16]));
                        report.push_str(&format!("- Found in {} locations:\n", group.len()));
                        
                        for entry in group.iter() {
                            report.push_str(&format!("  - **{}** v{} in `{}`\n", 
                                entry.name, entry.version, entry.git_repo));
                        }
                        report.push_str("\n");
                    }
                }
            }
        }
        
        // Repository analysis
        let mut by_repo: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            by_repo.entry(entry.git_repo.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        report.push_str("## Repository Content Summary\n");
        for (repo, entries) in &by_repo {
            report.push_str(&format!("- **{}**: {} Cargo.toml files\n", repo, entries.len()));
        }
        
        report.push_str("\n## Content-Addressable Storage Benefits\n");
        report.push_str("- **Deduplication**: Store identical content once by hash\n");
        report.push_str("- **Version Tracking**: Git objects track repository state\n");
        report.push_str("- **Content Integrity**: Hashes ensure data integrity\n");
        report.push_str("- **Cross-Repo Visibility**: Same content visible across repos\n");
        report.push_str("- **Merge Capability**: Combine file systems by content address\n");
        
        fs::write("ENHANCED_CONTENT_ADDRESSABLE.md", &report)?;
        
        println!("  ✓ Report written to ENHANCED_CONTENT_ADDRESSABLE.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Enhanced Content-Addressable Cargo.toml Mapper");
        
        self.scan_all_cargo_tomls()?;
        self.generate_enhanced_report()?;
        
        // Analyze content duplication
        let mut content_groups: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_groups.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates = content_groups.values().filter(|v| v.len() > 1).count();
        
        println!("\n🎯 === ENHANCED CONTENT-ADDRESSABLE MAPPING COMPLETE ===");
        println!("  Total Cargo.toml files: {}", self.entries.len());
        println!("  Unique content hashes: {}", content_groups.len());
        println!("  Duplicate content groups: {}", duplicates);
        
        if duplicates > 0 {
            println!("  🔗 Found identical content across repositories!");
        } else {
            println!("  📦 All Cargo.toml files have unique content");
        }
        
        println!("\n🔗 CONTENT ADDRESSING: git obj -> cargo.toml -> content hash -> merge by content");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = EnhancedContentMapper::new();
    mapper.run()
}
