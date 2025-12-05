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
        
        // Collect fork URLs to process
        let fork_urls: Vec<String> = self.fork_relations.values()
            .filter(|r| r.fork_url.contains("meta-introspector"))
            .map(|r| r.fork_url.clone())
            .collect();
        
        // Update upstream URLs
        for fork_url in fork_urls {
            let upstream = self.find_upstream_url(&fork_url);
            
            // Find and update the relation
            for relation in self.fork_relations.values_mut() {
                if relation.fork_url == fork_url {
                    relation.upstream_url = upstream;
                    break;
                }
            }
        }
        
        println!("  ✓ Analyzed fork relationships");
        Ok(())
    }
    
    fn find_upstream_url(&self, fork_url: &str) -> Option<String> {
        // Extract repo name from fork URL
        if let Some(repo_name) = fork_url.split('/').last() {
            let clean_name = repo_name.replace(".git", "");
            
            // Map known forks to their upstreams
            if clean_name.contains("blake3") || clean_name == "BLAKE3" {
                return Some("https://github.com/BLAKE3-team/BLAKE3".to_string());
            } else if clean_name.contains("base64") {
                return Some("https://github.com/marshallpierce/rust-base64".to_string());
            } else if clean_name.contains("backtrace") {
                return Some("https://github.com/rust-lang/backtrace-rs".to_string());
            } else if clean_name.contains("rust") || clean_name.contains("rustc") {
                return Some(format!("https://github.com/rust-lang/{}", clean_name));
            } else if clean_name.contains("serde") {
                return Some(format!("https://github.com/serde-rs/{}", clean_name));
            } else if clean_name.contains("tokio") {
                return Some(format!("https://github.com/tokio-rs/{}", clean_name));
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
        
        // Group by fork type
        let mut meta_forks = Vec::new();
        let mut upstream_repos = Vec::new();
        
        for relation in self.fork_relations.values() {
            if relation.fork_url.contains("meta-introspector") {
                meta_forks.push(relation);
            } else {
                upstream_repos.push(relation);
            }
        }
        
        report.push_str("## Meta-Introspector Fork Relationships\n\n");
        for relation in &meta_forks {
            let short_path = relation.submodule_path.replace("submodules/", "");
            report.push_str(&format!("### {}\n", short_path));
            report.push_str(&format!("- **Fork**: `{}`\n", relation.fork_url));
            
            if let Some(upstream) = &relation.upstream_url {
                report.push_str(&format!("- **Upstream**: `{}`\n", upstream));
                report.push_str(&format!("- **Relationship**: `{}` → **fork of** → `{}`\n", short_path, upstream));
            } else {
                report.push_str("- **Upstream**: Unknown\n");
                report.push_str(&format!("- **Relationship**: `{}` → **fork of** → ❓\n", short_path));
            }
            
            report.push_str(&format!("- **Git Object**: `{}`\n", relation.git_object));
            report.push_str("\n");
        }
        
        // Statistics
        report.push_str("## Fork Statistics\n");
        report.push_str(&format!("- Total git modules: {}\n", self.fork_relations.len()));
        report.push_str(&format!("- Meta-introspector forks: {}\n", meta_forks.len()));
        report.push_str(&format!("- Direct upstream repos: {}\n", upstream_repos.len()));
        
        let with_upstream = self.fork_relations.values()
            .filter(|r| r.upstream_url.is_some())
            .count();
        
        report.push_str(&format!("- Mapped upstream relationships: {}\n", with_upstream));
        
        let fork_coverage = if meta_forks.len() > 0 {
            (with_upstream as f64 / meta_forks.len() as f64) * 100.0
        } else { 0.0 };
        
        report.push_str(&format!("- Fork mapping coverage: {:.1}%\n", fork_coverage));
        
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
        
        let coverage = if meta_forks > 0 {
            (with_upstream as f64 / meta_forks as f64) * 100.0
        } else { 0.0 };
        
        println!("  Fork mapping coverage: {:.1}%", coverage);
        
        println!("\n🔗 FORK RELATIONSHIP CHAIN: git module → fork of → git module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = GitForkMapper::new();
    mapper.run()
}
