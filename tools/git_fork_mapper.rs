use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ForkRelation {
    submodule_path: String,
    fork_url: String,
    upstream_url: Option<String>,
    git_object: String,
}

struct GitForkMapper {
    fork_relations: HashMap<String, ForkRelation>,
}

impl GitForkMapper {
    fn new() -> Self {
        Self {
            fork_relations: HashMap::new(),
        }
    }
    
    fn parse_gitmodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Parsing .gitmodules for fork relationships...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut current_path = String::new();
        let mut current_url = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                // Save previous entry
                if !current_path.is_empty() && !current_url.is_empty() {
                    let git_object = self.get_git_object(&current_path);
                    
                    self.fork_relations.insert(current_path.clone(), ForkRelation {
                        submodule_path: current_path.clone(),
                        fork_url: current_url.clone(),
                        upstream_url: None,
                        git_object,
                    });
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
            let git_object = self.get_git_object(&current_path);
            
            self.fork_relations.insert(current_path.clone(), ForkRelation {
                submodule_path: current_path,
                fork_url: current_url,
                upstream_url: None,
                git_object,
            });
        }
        
        println!("  ✓ Found {} git modules", self.fork_relations.len());
        Ok(())
    }
    
    fn detect_upstream_relationships(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Detecting upstream fork relationships...");
        
        for (path, relation) in &mut self.fork_relations {
            // Check if this is a meta-introspector fork
            if relation.fork_url.contains("meta-introspector") {
                // Try to find the upstream URL
                let upstream = self.find_upstream_url(&relation.fork_url);
                relation.upstream_url = upstream;
            }
        }
        
        println!("  ✓ Analyzed fork relationships");
        Ok(())
    }
    
    fn find_upstream_url(&self, fork_url: &str) -> Option<String> {
        // Extract repo name from fork URL
        if let Some(repo_name) = fork_url.split('/').last() {
            let clean_name = repo_name.replace(".git", "");
            
            // Common upstream patterns
            let upstream_patterns = vec![
                format!("https://github.com/rust-lang/{}", clean_name),
                format!("https://github.com/RustCrypto/{}", clean_name),
                format!("https://github.com/tokio-rs/{}", clean_name),
                format!("https://github.com/serde-rs/{}", clean_name),
                format!("https://github.com/{}-team/{}", clean_name.to_uppercase(), clean_name),
            ];
            
            // For now, return the most likely upstream
            // In a real implementation, we'd check if these URLs exist
            if clean_name.contains("rust") || clean_name.contains("rustc") {
                return Some(format!("https://github.com/rust-lang/{}", clean_name));
            } else if clean_name.contains("blake3") {
                return Some("https://github.com/BLAKE3-team/BLAKE3".to_string());
            } else if clean_name.contains("base64") {
                return Some("https://github.com/marshallpierce/rust-base64".to_string());
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
    
    fn generate_fork_map_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating git fork mapping report...");
        
        let mut report = String::new();
        report.push_str("# Git Fork Mapping Report\n\n");
        
        report.push_str("## Fork Relationship Flow\n");
        report.push_str("git module → fork of → git module\n\n");
        
        report.push_str("## Fork Relationships\n\n");
        
        // Group by fork type
        let mut meta_forks = Vec::new();
        let mut upstream_repos = Vec::new();
        
        for (path, relation) in &self.fork_relations {
            if relation.fork_url.contains("meta-introspector") {
                meta_forks.push((path, relation));
            } else {
                upstream_repos.push((path, relation));
            }
        }
        
        report.push_str("### Meta-Introspector Forks\n");
        for (path, relation) in &meta_forks {
            report.push_str(&format!("#### {}\n", path.replace("submodules/", "")));
            report.push_str(&format!("- **Fork URL**: `{}`\n", relation.fork_url));
            
            if let Some(upstream) = &relation.upstream_url {
                report.push_str(&format!("- **Upstream**: `{}`\n", upstream));
                report.push_str(&format!("- **Relationship**: `{}` → fork of → `{}`\n", relation.fork_url, upstream));
            } else {
                report.push_str("- **Upstream**: Unknown\n");
            }
            
            report.push_str(&format!("- **Git Object**: `{}`\n", relation.git_object));
            report.push_str("\n");
        }
        
        report.push_str("### Direct Upstream Repositories\n");
        for (path, relation) in &upstream_repos {
            if upstream_repos.len() < 10 { // Show first 10
                report.push_str(&format!("- `{}` → `{}`\n", 
                    path.replace("submodules/", ""), relation.fork_url));
            }
        }
        
        // Statistics
        report.push_str("\n## Fork Statistics\n");
        report.push_str(&format!("- Total git modules: {}\n", self.fork_relations.len()));
        report.push_str(&format!("- Meta-introspector forks: {}\n", meta_forks.len()));
        report.push_str(&format!("- Direct upstream repos: {}\n", upstream_repos.len()));
        
        let with_upstream = self.fork_relations.values()
            .filter(|r| r.upstream_url.is_some())
            .count();
        
        report.push_str(&format!("- Mapped upstream relationships: {}\n", with_upstream));
        
        report.push_str("\n## RocksDB Fork Schema\n");
        report.push_str("```\n");
        report.push_str("Key: submodule_path:git_object\n");
        report.push_str("Value: {\n");
        report.push_str("  fork_url: string,\n");
        report.push_str("  upstream_url: string,\n");
        report.push_str("  fork_relationship: \"fork_of\",\n");
        report.push_str("  git_object: string\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("GIT_FORK_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to GIT_FORK_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Git Fork Mapper - git module → fork of → git module");
        
        self.parse_gitmodules()?;
        self.detect_upstream_relationships()?;
        self.generate_fork_map_report()?;
        
        let meta_forks = self.fork_relations.values()
            .filter(|r| r.fork_url.contains("meta-introspector"))
            .count();
        
        let with_upstream = self.fork_relations.values()
            .filter(|r| r.upstream_url.is_some())
            .count();
        
        println!("\n🎯 === FORK MAPPING COMPLETE ===");
        println!("  Total git modules: {}", self.fork_relations.len());
        println!("  Meta-introspector forks: {}", meta_forks);
        println!("  Upstream relationships: {}", with_upstream);
        
        println!("\n🔗 FORK RELATIONSHIP CHAIN MAPPED!");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = GitForkMapper::new();
    mapper.run()
}
