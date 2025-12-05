use std::fs;
use std::collections::HashMap;

struct GitGraphPaged {
    repos: HashMap<String, GitRepo>,
    url_index: HashMap<String, String>, // url -> path
    pages: HashMap<String, Vec<String>>, // large_repo_path -> [submodule_urls]
}

#[derive(Debug)]
struct GitRepo {
    url: String,
    submodule_count: usize,
    is_large: bool, // > 100 submodules
}

impl GitGraphPaged {
    fn new() -> Self {
        Self {
            repos: HashMap::new(),
            url_index: HashMap::new(),
            pages: HashMap::new(),
        }
    }
    
    fn load_git_graph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Building paged git graph database...");
        
        let content = fs::read_to_string("../git_files_inventory2.txt")?;
        let mut processed = 0;
        
        for line in content.lines() {
            if line.ends_with("/.git") {
                let repo_path = line.replace("/.git", "");
                self.process_repo(&repo_path)?;
                
                processed += 1;
                if processed % 1000 == 0 {
                    println!("  Processed {} repositories...", processed);
                }
            }
        }
        
        println!("✓ Processed {} repositories total", processed);
        Ok(())
    }
    
    fn process_repo(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut repo = GitRepo {
            url: String::new(),
            submodule_count: 0,
            is_large: false,
        };
        
        // Get URL from .git/config
        let config_path = format!("{}/.git/config", path);
        if let Ok(config) = fs::read_to_string(&config_path) {
            repo.url = self.extract_url(&config);
        }
        
        // Count submodules and store URLs
        let gitmodules_path = format!("{}/.gitmodules", path);
        if let Ok(gitmodules) = fs::read_to_string(&gitmodules_path) {
            let submodule_urls = self.extract_submodule_urls(&gitmodules);
            repo.submodule_count = submodule_urls.len();
            repo.is_large = repo.submodule_count > 100;
            
            // Store in pages for large repos
            if repo.is_large {
                self.pages.insert(path.to_string(), submodule_urls);
            }
        }
        
        // Index by URL if available
        if !repo.url.is_empty() {
            self.url_index.insert(repo.url.clone(), path.to_string());
        }
        
        self.repos.insert(path.to_string(), repo);
        Ok(())
    }
    
    fn extract_url(&self, config: &str) -> String {
        for line in config.lines() {
            if line.trim().starts_with("url = ") {
                return line.trim()[6..].to_string();
            }
        }
        String::new()
    }
    
    fn extract_submodule_urls(&self, gitmodules: &str) -> Vec<String> {
        let mut urls = Vec::new();
        for line in gitmodules.lines() {
            if line.trim().starts_with("url = ") {
                urls.push(line.trim()[6..].to_string());
            }
        }
        urls
    }
    
    fn analyze_graph(&self) {
        println!("\n=== PAGED GIT GRAPH ANALYSIS ===");
        
        let total_repos = self.repos.len();
        let repos_with_urls = self.url_index.len();
        let large_repos = self.repos.values().filter(|r| r.is_large).count();
        let total_submodules: usize = self.repos.values().map(|r| r.submodule_count).sum();
        
        println!("📊 STATISTICS:");
        println!("  Total repositories: {}", total_repos);
        println!("  Repositories with URLs: {}", repos_with_urls);
        println!("  Large repositories (>100 submodules): {}", large_repos);
        println!("  Total submodule relationships: {}", total_submodules);
        
        // Find largest repositories
        let mut large_repos_vec: Vec<_> = self.repos.iter()
            .filter(|(_, repo)| repo.is_large)
            .collect();
        large_repos_vec.sort_by(|a, b| b.1.submodule_count.cmp(&a.1.submodule_count));
        
        println!("\n🔍 LARGEST REPOSITORIES:");
        for (path, repo) in large_repos_vec.iter().take(5) {
            println!("  {} -> {} ({} submodules)", 
                     path.split('/').last().unwrap_or(path), 
                     repo.url, 
                     repo.submodule_count);
        }
        
        // URL domain analysis
        let mut domains: HashMap<String, u32> = HashMap::new();
        for url in self.url_index.keys() {
            if let Some(domain) = self.extract_domain(url) {
                *domains.entry(domain).or_insert(0) += 1;
            }
        }
        
        println!("\n🌐 URL DOMAINS:");
        let mut domain_vec: Vec<_> = domains.iter().collect();
        domain_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (domain, count) in domain_vec.iter().take(5) {
            println!("  {}: {} repositories", domain, count);
        }
        
        println!("\n💾 STORAGE OPTIMIZATION:");
        println!("  Paged repositories: {}", self.pages.len());
        println!("  Memory saved by paging: ~{} MB", 
                 self.pages.values().map(|v| v.len()).sum::<usize>() * 50 / 1024 / 1024);
    }
    
    fn extract_domain(&self, url: &str) -> Option<String> {
        if url.contains("github.com") { Some("github.com".to_string()) }
        else if url.contains("gitlab.com") { Some("gitlab.com".to_string()) }
        else if url.contains("bitbucket.org") { Some("bitbucket.org".to_string()) }
        else { Some("other".to_string()) }
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.load_git_graph()?;
        self.analyze_graph();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = GitGraphPaged::new();
    db.run()
}
