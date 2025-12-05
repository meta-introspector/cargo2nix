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
        let content_hash = self.calculate_git_hash(content);
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
    
    fn calculate_git_hash(&self, content: &str) -> String {
        // Use git hash-object to get the actual git content hash
        let output = Command::new("git")
            .args(&["hash-object", "--stdin"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn();
        
        if let Ok(mut child) = output {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                let _ = stdin.write_all(content.as_bytes());
            }
            
            if let Ok(output) = child.wait_with_output() {
                if output.status.success() {
                    return String::from_utf8_lossy(&output.stdout).trim().to_string();
                }
            }
        }
        
        // Fallback to simple hash
        format!("fallback_{:x}", content.len())
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
        report.push_str("git obj -> cargo.toml -> name and version -> git content hash -> merge by content address\n\n");
        
        report.push_str("## All Cargo.toml Content Mappings\n\n");
        
        // Group by git repo
        let mut by_repo: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            by_repo.entry(entry.git_repo.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        // Sort repos by number of Cargo.toml files
        let mut sorted_repos: Vec<_> = by_repo.iter().collect();
        sorted_repos.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        
        for (git_repo, entries) in &sorted_repos {
            report.push_str(&format!("### {} ({} Cargo.toml files)\n", git_repo, entries.len()));
            
            if let Some(first_entry) = entries.first() {
                report.push_str(&format!("- **Git Object**: `{}`\n", first_entry.git_object));
            }
            
            for entry in entries {
                report.push_str(&format!("  - **{}** v{}\n", entry.name, entry.version));
                report.push_str(&format!("    - Content Hash: `{}`\n", &entry.content_hash[..16]));
                
                let short_path = entry.cargo_toml_path.replace("../", "");
                report.push_str(&format!("    - Path: `{}`\n", short_path));
                
                report.push_str(&format!("    - **Mapping**: `{}` -> `{}` -> `{}` v{}\n", 
                    entry.git_object[..8].to_string(), 
                    &entry.content_hash[..8], 
                    entry.name, 
                    entry.version));
            }
            
            report.push_str("\n");
        }
        
        // Content hash analysis
        let mut content_hashes: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_hashes.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates: Vec<_> = content_hashes.values().filter(|v| v.len() > 1).collect();
        
        report.push_str("## Content Hash Analysis\n");
        report.push_str(&format!("- Total Cargo.toml files: {}\n", self.entries.len()));
        report.push_str(&format!("- Unique content hashes: {}\n", content_hashes.len()));
        report.push_str(&format!("- Identical content instances: {}\n", duplicates.len()));
        
        // Show content hash to git object mappings
        report.push_str("\n## Content Hash to Git Object Mappings\n");
        for (i, entry) in self.entries.iter().enumerate() {
            if i < 15 { // Show first 15
                report.push_str(&format!("- `{}` -> `{}` -> **{}** v{} in `{}`\n",
                    &entry.git_object[..8],
                    &entry.content_hash[..8],
                    entry.name,
                    entry.version,
                    entry.git_repo));
            }
        }
        
        if self.entries.len() > 15 {
            report.push_str(&format!("- ... and {} more mappings\n", self.entries.len() - 15));
        }
        
        // Potential for content-addressable storage
        report.push_str("\n## Content-Addressable Storage Benefits\n");
        report.push_str("- **Deduplication**: Identical Cargo.toml content stored once\n");
        report.push_str("- **Version Tracking**: Git objects track repository state\n");
        report.push_str("- **Content Integrity**: Content hashes ensure data integrity\n");
        report.push_str("- **Cross-Repo Analysis**: Same content visible across repositories\n");
        
        fs::write("ENHANCED_CONTENT_ADDRESSABLE.md", &report)?;
        
        println!("  ✓ Report written to ENHANCED_CONTENT_ADDRESSABLE.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Enhanced Content-Addressable Cargo.toml Mapper");
        
        self.scan_all_cargo_tomls()?;
        self.generate_enhanced_report()?;
        
        // Group by content hash to find duplicates
        let mut content_groups: HashMap<String, Vec<&ContentEntry>> = HashMap::new();
        for entry in &self.entries {
            content_groups.entry(entry.content_hash.clone()).or_insert_with(Vec::new).push(entry);
        }
        
        let duplicates = content_groups.values().filter(|v| v.len() > 1).count();
        
        println!("\n🎯 === ENHANCED CONTENT-ADDRESSABLE MAPPING COMPLETE ===");
        println!("  Total Cargo.toml files: {}", self.entries.len());
        println!("  Unique content hashes: {}", content_groups.len());
        println!("  Duplicate content groups: {}", duplicates);
        
        println!("\n🔗 CONTENT ADDRESSING: git obj -> cargo.toml -> git content hash -> merge by content");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = EnhancedContentMapper::new();
    mapper.run()
}
