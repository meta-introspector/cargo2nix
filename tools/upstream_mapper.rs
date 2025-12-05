use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct UpstreamRelation {
    upstream_url: String,
    forks: Vec<String>,  // List of fork URLs that point to this upstream
    git_objects: Vec<String>,
}

struct UpstreamMapper {
    upstream_relations: HashMap<String, UpstreamRelation>,
}

impl UpstreamMapper {
    fn new() -> Self {
        Self {
            upstream_relations: HashMap::new(),
        }
    }
    
    fn load_fork_mappings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Loading fork mappings to build upstream relationships...");
        
        // Read the fork mapping report
        let fork_report = fs::read_to_string("GIT_FORK_MAPPING.md")?;
        
        for line in fork_report.lines() {
            if line.contains("→ **fork of** →") {
                // Parse: `fork_name` → **fork of** → `upstream_url`
                if let Some(parts) = self.parse_fork_relationship(line) {
                    let (fork_name, upstream_url) = parts;
                    
                    // Add to upstream relations
                    self.upstream_relations
                        .entry(upstream_url.clone())
                        .or_insert_with(|| UpstreamRelation {
                            upstream_url: upstream_url.clone(),
                            forks: Vec::new(),
                            git_objects: Vec::new(),
                        })
                        .forks
                        .push(fork_name);
                }
            }
        }
        
        println!("  ✓ Found {} upstream repositories", self.upstream_relations.len());
        Ok(())
    }
    
    fn parse_fork_relationship(&self, line: &str) -> Option<(String, String)> {
        // Parse: `fork_name` → **fork of** → `upstream_url`
        if let Some(start) = line.find('`') {
            if let Some(end) = line[start + 1..].find('`') {
                let fork_name = &line[start + 1..start + 1 + end];
                
                // Find the upstream URL
                if let Some(upstream_start) = line.rfind('`') {
                    if upstream_start > start + 1 + end {
                        if let Some(upstream_end) = line[..upstream_start].rfind('`') {
                            if upstream_end > start + 1 + end {
                                let upstream_url = &line[upstream_end + 1..upstream_start];
                                return Some((fork_name.to_string(), upstream_url.to_string()));
                            }
                        }
                    }
                }
            }
        }
        None
    }
    
    fn enhance_with_git_objects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Enhancing with git object information...");
        
        // Read gitmodules to get git objects for forks
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut path_to_url = HashMap::new();
        let mut current_path = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                current_path.clear();
            } else if line.starts_with("path = ") {
                current_path = line.replace("path = ", "");
            } else if line.starts_with("url = ") && !current_path.is_empty() {
                let url = line.replace("url = ", "");
                path_to_url.insert(current_path.clone(), url);
            }
        }
        
        // Match fork URLs to git objects
        for upstream_relation in self.upstream_relations.values_mut() {
            for fork_name in &upstream_relation.forks {
                // Find matching submodule path
                for (path, url) in &path_to_url {
                    if url.contains(fork_name) || path.contains(fork_name) {
                        let git_object = self.get_git_object(path);
                        if git_object != "unknown" {
                            upstream_relation.git_objects.push(git_object);
                        }
                        break;
                    }
                }
            }
        }
        
        println!("  ✓ Enhanced with git object data");
        Ok(())
    }
    
    fn get_git_object(&self, submodule_path: &str) -> String {
        use std::process::Command;
        
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
    
    fn generate_upstream_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating upstream relationship report...");
        
        let mut report = String::new();
        report.push_str("# Upstream Relationship Mapping Report\n\n");
        
        report.push_str("## Upstream Relationship Flow\n");
        report.push_str("git module ← upstream of ← git module\n\n");
        
        report.push_str("## Upstream Repositories\n\n");
        
        // Sort by number of forks (most popular first)
        let mut sorted_upstreams: Vec<_> = self.upstream_relations.iter().collect();
        sorted_upstreams.sort_by(|a, b| b.1.forks.len().cmp(&a.1.forks.len()));
        
        for (upstream_url, relation) in &sorted_upstreams {
            let repo_name = upstream_url.split('/').last().unwrap_or("unknown");
            
            report.push_str(&format!("### {} ({})\n", repo_name, upstream_url));
            report.push_str(&format!("- **Upstream URL**: `{}`\n", upstream_url));
            report.push_str(&format!("- **Fork Count**: {}\n", relation.forks.len()));
            
            report.push_str("- **Forks**:\n");
            for fork in &relation.forks {
                report.push_str(&format!("  - `{}` ← **upstream of** ← `{}`\n", upstream_url, fork));
            }
            
            if !relation.git_objects.is_empty() {
                report.push_str("- **Git Objects**:\n");
                for obj in &relation.git_objects {
                    report.push_str(&format!("  - `{}`\n", obj));
                }
            }
            
            report.push_str("\n");
        }
        
        // Statistics
        report.push_str("## Upstream Statistics\n");
        report.push_str(&format!("- Total upstream repositories: {}\n", self.upstream_relations.len()));
        
        let total_forks: usize = self.upstream_relations.values()
            .map(|r| r.forks.len())
            .sum();
        
        report.push_str(&format!("- Total forks tracked: {}\n", total_forks));
        
        if self.upstream_relations.len() > 0 {
            let avg_forks = total_forks as f64 / self.upstream_relations.len() as f64;
            report.push_str(&format!("- Average forks per upstream: {:.1}\n", avg_forks));
        }
        
        // Top upstream repositories
        report.push_str("\n## Top Upstream Repositories\n");
        for (i, (upstream_url, relation)) in sorted_upstreams.iter().enumerate() {
            if i < 10 {
                let repo_name = upstream_url.split('/').last().unwrap_or("unknown");
                report.push_str(&format!("{}. **{}** - {} forks\n", i + 1, repo_name, relation.forks.len()));
            }
        }
        
        report.push_str("\n## RocksDB Upstream Schema\n");
        report.push_str("```\n");
        report.push_str("Key: upstream_url\n");
        report.push_str("Value: {\n");
        report.push_str("  upstream_url: string,\n");
        report.push_str("  forks: [fork_urls],\n");
        report.push_str("  git_objects: [fork_git_objects],\n");
        report.push_str("  relationship: \"upstream_of\"\n");
        report.push_str("}\n");
        report.push_str("```\n");
        
        fs::write("UPSTREAM_MAPPING.md", &report)?;
        
        println!("  ✓ Report written to UPSTREAM_MAPPING.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Upstream Mapper - git module ← upstream of ← git module");
        
        self.load_fork_mappings()?;
        self.enhance_with_git_objects()?;
        self.generate_upstream_report()?;
        
        let total_forks: usize = self.upstream_relations.values()
            .map(|r| r.forks.len())
            .sum();
        
        println!("\n🎯 === UPSTREAM MAPPING COMPLETE ===");
        println!("  Upstream repositories: {}", self.upstream_relations.len());
        println!("  Total forks tracked: {}", total_forks);
        
        if self.upstream_relations.len() > 0 {
            let avg_forks = total_forks as f64 / self.upstream_relations.len() as f64;
            println!("  Average forks per upstream: {:.1}", avg_forks);
        }
        
        println!("\n🔗 UPSTREAM RELATIONSHIP CHAIN: git module ← upstream of ← git module");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mapper = UpstreamMapper::new();
    mapper.run()
}
