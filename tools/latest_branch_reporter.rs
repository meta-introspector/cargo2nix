use std::fs;
use std::process::Command;
use std::collections::HashMap;

struct LatestBranchReporter {
    submodule_repos: HashMap<String, String>, // submodule_path -> repo_url
    branch_info: HashMap<String, String>,     // submodule_path -> latest_branch
}

impl LatestBranchReporter {
    fn new() -> Self {
        Self {
            submodule_repos: HashMap::new(),
            branch_info: HashMap::new(),
        }
    }
    
    fn parse_gitmodules(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Parsing .gitmodules for repository URLs...");
        
        let gitmodules = fs::read_to_string("../.gitmodules")?;
        let mut current_path = String::new();
        let mut current_url = String::new();
        
        for line in gitmodules.lines() {
            let line = line.trim();
            
            if line.starts_with("[submodule") {
                // Save previous entry if we have both path and url
                if !current_path.is_empty() && !current_url.is_empty() {
                    self.submodule_repos.insert(current_path.clone(), current_url.clone());
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
            self.submodule_repos.insert(current_path, current_url);
        }
        
        println!("  ✓ Found {} submodule repositories", self.submodule_repos.len());
        Ok(())
    }
    
    fn check_latest_branches(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌿 Checking latest branches for key repositories...");
        
        // Focus on key repositories we need for rustc
        let key_repos = vec![
            "submodules/BLAKE3",
            "submodules/rust-base64", 
            "submodules/aes",
            "submodules/backtrace",
            "submodules/ansi_term",
            "submodules/askama",
            "submodules/bytecount"
        ];
        
        for submodule_path in &key_repos {
            if let Some(repo_url) = self.submodule_repos.get(*submodule_path) {
                println!("  Checking {} -> {}", submodule_path, repo_url);
                
                let branch = self.get_default_branch(repo_url)?;
                self.branch_info.insert(submodule_path.to_string(), branch);
            } else {
                println!("  ❌ {} not found in .gitmodules", submodule_path);
            }
        }
        
        Ok(())
    }
    
    fn get_default_branch(&self, repo_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Try to get the default branch using git ls-remote
        let output = Command::new("git")
            .args(&["ls-remote", "--symref", repo_url, "HEAD"])
            .output()?;
        
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            
            // Look for "ref: refs/heads/BRANCH_NAME"
            for line in result.lines() {
                if line.starts_with("ref: refs/heads/") {
                    let branch = line.replace("ref: refs/heads/", "").trim().to_string();
                    return Ok(branch);
                }
            }
            
            // Fallback: try to find main or master
            if result.contains("refs/heads/main") {
                return Ok("main".to_string());
            } else if result.contains("refs/heads/master") {
                return Ok("master".to_string());
            }
        }
        
        // Default fallback
        Ok("main".to_string())
    }
    
    fn generate_branch_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Generating latest branch report...");
        
        let mut report = String::new();
        report.push_str("# Latest Branch Report for Key Submodules\n\n");
        
        report.push_str("## Repository Branch Information\n");
        report.push_str("Commands to reset submodules to latest branches:\n\n");
        report.push_str("```bash\n");
        report.push_str("cd /mnt/data1/nix/vendor/rust/cargo2nix\n\n");
        
        for (submodule_path, branch) in &self.branch_info {
            if let Some(repo_url) = self.submodule_repos.get(submodule_path) {
                report.push_str(&format!("# Reset {} to latest {}\n", submodule_path, branch));
                report.push_str(&format!("git submodule deinit -f {}\n", submodule_path));
                report.push_str(&format!("git submodule add -b {} {} {}\n", branch, repo_url, submodule_path));
                report.push_str(&format!("git submodule update --init --recursive {}\n\n", submodule_path));
            }
        }
        
        report.push_str("```\n\n");
        
        report.push_str("## Summary\n");
        for (submodule_path, branch) in &self.branch_info {
            if let Some(repo_url) = self.submodule_repos.get(submodule_path) {
                report.push_str(&format!("- `{}` → `{}` (branch: `{}`)\n", 
                    submodule_path.replace("submodules/", ""), repo_url, branch));
            }
        }
        
        report.push_str("\n## Next Steps\n");
        report.push_str("1. Run the commands above to reset submodules to latest branches\n");
        report.push_str("2. Re-run cross-reference checker to verify Cargo.toml files\n");
        report.push_str("3. Update Monster Protocol with found crates\n");
        
        fs::write("LATEST_BRANCH_REPORT.md", &report)?;
        
        println!("  ✓ Report written to LATEST_BRANCH_REPORT.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Latest Branch Reporter for Key Submodules");
        
        self.parse_gitmodules()?;
        self.check_latest_branches()?;
        self.generate_branch_report()?;
        
        println!("\n🎯 === BRANCH ANALYSIS COMPLETE ===");
        println!("  Submodule repos: {}", self.submodule_repos.len());
        println!("  Branch info collected: {}", self.branch_info.len());
        
        println!("\n📋 Key Repository Branches:");
        for (path, branch) in &self.branch_info {
            println!("  {} → {}", path.replace("submodules/", ""), branch);
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reporter = LatestBranchReporter::new();
    reporter.run()
}
