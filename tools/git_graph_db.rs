use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct GitRepo {
    path: String,
    url: String,
    branch: String,
    submodules: Vec<String>, // URLs of submodules
    remotes: Vec<String>,    // Remote URLs
}

struct GitGraphDB {
    repos: HashMap<String, GitRepo>, // path -> repo
    url_to_path: HashMap<String, String>, // url -> path
    relationships: HashMap<String, Vec<String>>, // parent_path -> [child_paths]
}

impl GitGraphDB {
    fn new() -> Self {
        Self {
            repos: HashMap::new(),
            url_to_path: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
    
    fn load_all_git_repos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading all git repositories...");
        
        // Read git inventory
        let content = fs::read_to_string("../git_files_inventory2.txt")?;
        
        for line in content.lines() {
            if line.ends_with("/.git") {
                let repo_path = line.replace("/.git", "");
                self.process_git_repo(&repo_path)?;
            }
        }
        
        println!("Loaded {} git repositories", self.repos.len());
        Ok(())
    }
    
    fn process_git_repo(&mut self, repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut repo = GitRepo {
            path: repo_path.to_string(),
            url: String::new(),
            branch: "main".to_string(),
            submodules: Vec::new(),
            remotes: Vec::new(),
        };
        
        // Read .git/config
        let config_path = format!("{}/.git/config", repo_path);
        if let Ok(config) = fs::read_to_string(&config_path) {
            repo.url = self.extract_origin_url(&config);
            repo.remotes = self.extract_all_remotes(&config);
        }
        
        // Read .gitmodules
        let gitmodules_path = format!("{}/.gitmodules", repo_path);
        if let Ok(gitmodules) = fs::read_to_string(&gitmodules_path) {
            repo.submodules = self.extract_submodule_urls(&gitmodules);
            
            // Build relationships
            for submodule_url in &repo.submodules {
                self.relationships.entry(repo_path.to_string())
                    .or_insert_with(Vec::new)
                    .push(submodule_url.clone());
            }
        }
        
        // Store mappings
        if !repo.url.is_empty() {
            self.url_to_path.insert(repo.url.clone(), repo_path.to_string());
        }
        
        self.repos.insert(repo_path.to_string(), repo);
        Ok(())
    }
    
    fn extract_origin_url(&self, config: &str) -> String {
        for line in config.lines() {
            let line = line.trim();
            if line.starts_with("url = ") {
                return line[6..].to_string();
            }
        }
        String::new()
    }
    
    fn extract_all_remotes(&self, config: &str) -> Vec<String> {
        let mut remotes = Vec::new();
        for line in config.lines() {
            let line = line.trim();
            if line.starts_with("url = ") {
                remotes.push(line[6..].to_string());
            }
        }
        remotes
    }
    
    fn extract_submodule_urls(&self, gitmodules: &str) -> Vec<String> {
        let mut urls = Vec::new();
        for line in gitmodules.lines() {
            let line = line.trim();
            if line.starts_with("url = ") {
                urls.push(line[6..].to_string());
            }
        }
        urls
    }
    
    fn resolve_all_urls(&mut self) {
        println!("Resolving URL relationships...");
        
        // Create reverse mapping: child_url -> parent_paths
        let mut child_to_parents: HashMap<String, Vec<String>> = HashMap::new();
        
        for (parent_path, child_urls) in &self.relationships {
            for child_url in child_urls {
                child_to_parents.entry(child_url.clone())
                    .or_insert_with(Vec::new)
                    .push(parent_path.clone());
            }
        }
        
        println!("URL resolution complete:");
        println!("  {} unique repositories", self.repos.len());
        println!("  {} URL mappings", self.url_to_path.len());
        println!("  {} parent-child relationships", self.relationships.len());
    }
    
    fn generate_graph_report(&self) {
        println!("\n=== GIT GRAPH DATABASE REPORT ===");
        
        println!("\n1. TOP-LEVEL REPOSITORIES:");
        let mut top_level = 0;
        for (path, repo) in &self.repos {
            if !repo.submodules.is_empty() {
                println!("  {} -> {} ({} submodules)", path, repo.url, repo.submodules.len());
                top_level += 1;
                if top_level >= 5 { break; } // Limit output
            }
        }
        
        println!("\n2. SUBMODULE RELATIONSHIPS:");
        let mut rel_count = 0;
        for (parent, children) in &self.relationships {
            println!("  {} has {} submodules", parent, children.len());
            for child in children.iter().take(3) {
                println!("    -> {}", child);
            }
            rel_count += 1;
            if rel_count >= 5 { break; }
        }
        
        println!("\n3. URL STATISTICS:");
        let github_urls = self.repos.values().filter(|r| r.url.contains("github.com")).count();
        let gitlab_urls = self.repos.values().filter(|r| r.url.contains("gitlab.com")).count();
        let other_urls = self.repos.len() - github_urls - gitlab_urls;
        
        println!("  GitHub repositories: {}", github_urls);
        println!("  GitLab repositories: {}", gitlab_urls);
        println!("  Other repositories: {}", other_urls);
        
        println!("\n4. GRAPH STRUCTURE:");
        println!("  Total nodes (repos): {}", self.repos.len());
        println!("  Total edges (relationships): {}", 
                 self.relationships.values().map(|v| v.len()).sum::<usize>());
        
        let max_submodules = self.repos.values()
            .map(|r| r.submodules.len())
            .max()
            .unwrap_or(0);
        println!("  Max submodules per repo: {}", max_submodules);
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.load_all_git_repos()?;
        self.resolve_all_urls();
        self.generate_graph_report();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = GitGraphDB::new();
    db.run()
}
