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

impl ContentEntry {
    fn safe_truncate(s: &str, len: usize) -> String {
        if s.len() >= len {
            s[..len].to_string()
        } else {
            s.to_string()
        }
    }
}

struct SafeContentMapper {
    entries: Vec<ContentEntry>,
}

impl SafeContentMapper {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
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
                if let Some(entry) = self.create_content_entry(cargo_toml_path, &content) {
                    self.entries.push(entry);
                }
            }
        }
        
        println!("  ✓ Processed {} Cargo.toml files", self.entries.len());
        Ok(())
    }
    
    fn create_content_entry(&self, cargo_toml_path: &str, content: &str) -> Option<ContentEntry> {
        let content_hash = self.calculate_content_hash(content);
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
    
    fn calculate_content_hash(&self, content: &str) -> String {
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
    
    fn generate_content_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating content-addressable report...");
        
        let mut report = String::new();
        report.push_str("# Content-Addressable Cargo.toml Report\n\n");
        
        report.push_str("## Content Addressing Flow\n");
        report.push_str("git obj -> cargo.toml -> name and version -> content hash -> merge by content address\n\n");
        
        report.push_str("## Content Mappings\n\n");
        
        for (i, entry) in self.entries.iter().enumerate() {
            if i < 15 { // Show first 15
                report.push_str(&format!("### {} v{}\n", entry.name, entry.version));
                
                let git_obj_short = ContentEntry::safe_truncate(&entry.git_object, 12);
                let content_hash_short = ContentEntry::safe_truncate(&entry.content_hash, 12);
                
                report.push_str(&format!("- **Git Object**: `{}`\n", git_obj_short));
                report.push_str(&format!("- **Content Hash**: `{}`\n", content_hash_short));
                report.push_str(&format!("- **Repository**: `{}`\n", entry.git_repo));
                
                let short_path = entry.cargo_toml_path.replace("../", "");
                report.push_str(&format!("- **Path**: `{}`\n", short_path));
                
                report.push_str(&format!("- **Mapping**: `{}` -> `{}` -> **{}** v{}\n", 
                    git_obj_short, content_hash_short, entry.name, entry.version));
                
                report.push_str("\n");
            }
        }
        
        if self.entries.len() > 15 {
            report.push_str(&format!("... and {} more mappings\n\n", self.entries.len() - 15));
        }
        
        // Content analysis
        let mut content_groups: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_groups.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates: Vec<_> = content_groups.values().filter(|v| v.len() > 1).collect();
        
        report.push_str("## Content Analysis\n");
        report.push_str(&format!("- Total Cargo.toml files: {}\n", self.entries.len()));
        report.push_str(&format!("- Unique content hashes: {}\n", content_groups.len()));
        report.push_str(&format!("- Identical content groups: {}\n", duplicates.len()));
        
        // Repository breakdown
        let mut by_repo: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            by_repo.entry(entry.git_repo.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        report.push_str("\n## Repository Breakdown\n");
        for (repo, entries) in &by_repo {
            report.push_str(&format!("- **{}**: {} Cargo.toml files\n", repo, entries.len()));
        }
        
        // Content-addressable benefits
        report.push_str("\n## Content-Addressable Storage Benefits\n");
        report.push_str("- **Deduplication**: Store identical content once by hash\n");
        report.push_str("- **Version Tracking**: Git objects track repository state\n");
        report.push_str("- **Content Integrity**: Hashes ensure data integrity\n");
        report.push_str("- **Cross-Repo Analysis**: Same content visible across repos\n");
        report.push_str("- **Merge Capability**: Combine file systems by content address\n");
        report.push_str("- **Git Log Integration**: Track changes through git history\n");
        
        report.push_str("\n## RocksDB Content-Addressable Schema\n");
        report.push_str("```\n");
        report.push_str("Key: content_hash\n");
        report.push_str("Value: {\n");
        report.push_str("  name: string,\n");
        report.push_str("  version: string,\n");
        report.push_str("  git_locations: [{\n");
        report.push_str("    git_repo: string,\n");
        report.push_str("    git_object: string,\n");
        report.push_str("    cargo_toml_path: string\n");
        report.push_str("  }]\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("CONTENT_ADDRESSABLE_CARGO_FINAL.md", &report)?;
        
        println!("  ✓ Report written to CONTENT_ADDRESSABLE_CARGO_FINAL.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Content-Addressable Cargo.toml Mapper");
        
        self.scan_all_cargo_tomls()?;
        self.generate_content_report()?;
        
        // Analyze content duplication
        let mut content_groups: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_groups.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates = content_groups.values().filter(|v| v.len() > 1).count();
        
        println!("\n🎯 === CONTENT-ADDRESSABLE MAPPING COMPLETE ===");
        println!("  Total Cargo.toml files: {}", self.entries.len());
        println!("  Unique content hashes: {}", content_groups.len());
        println!("  Duplicate content groups: {}", duplicates);
        
        if duplicates > 0 {
            println!("  🔗 Found {} identical content groups across repositories!", duplicates);
        } else {
            println!("  📦 All Cargo.toml files have unique content");
        }
        
        println!("\n🔗 CONTENT ADDRESSING: git obj -> cargo.toml -> content hash -> merge by content");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = SafeContentMapper::new();
    mapper.run()
}
