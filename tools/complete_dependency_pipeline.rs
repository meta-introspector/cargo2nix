use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DependencyEntry {
    rustc_crate: String,
    cargo_metadata: Option<String>,
    repo_url: Option<String>,
    fork_url: Option<String>,
    branch: Option<String>,
    submodule_path: Option<String>,
    cargo_toml_path: Option<String>,
    package_name: Option<String>,
    git_object: Option<String>,
    dependencies: Vec<String>,
}

struct CompleteDependencyPipeline {
    entries: HashMap<String, DependencyEntry>,
    rustc_crates: Vec<String>,
    cargo_metadata: HashMap<String, String>,
    submodule_repos: HashMap<String, String>,
}

impl CompleteDependencyPipeline {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            rustc_crates: Vec::new(),
            cargo_metadata: HashMap::new(),
            submodule_repos: HashMap::new(),
        }
    }
    
    fn step1_load_rustc_crates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🦀 STEP 1: Loading rustc crates...");
        
        // Load from dry run report
        if let Ok(report) = fs::read_to_string("RUSTC_RECURSIVE_DRY_RUN.md") {
            for line in report.lines() {
                if line.starts_with("- `") && line.contains("` → https://github.com/") {
                    if let Some(crate_name) = line.strip_prefix("- `").and_then(|s| s.split('`').next()) {
                        self.rustc_crates.push(crate_name.to_string());
                        
                        // Initialize entry
                        self.entries.insert(crate_name.to_string(), DependencyEntry {
                            rustc_crate: crate_name.to_string(),
                            cargo_metadata: None,
                            repo_url: None,
                            fork_url: None,
                            branch: None,
                            submodule_path: None,
                            cargo_toml_path: None,
                            package_name: None,
                            git_object: None,
                            dependencies: Vec::new(),
                        });
                    }
                }
            }
        }
        
        println!("  ✓ Loaded {} rustc crates", self.rustc_crates.len());
        Ok(())
    }
    
    fn step2_load_cargo_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 STEP 2: Loading cargo metadata...");
        
        if let Ok(metadata) = fs::read_to_string("cargo_metadata_db.json") {
            // Parse JSON for repository URLs
            let mut current_crate = String::new();
            
            for line in metadata.lines() {
                if line.contains("\"name\":\"") {
                    if let Some(start) = line.find("\"name\":\"") {
                        let start = start + 8;
                        if let Some(end) = line[start..].find('"') {
                            current_crate = line[start..start + end].to_string();
                        }
                    }
                }
                
                if line.contains("\"repository\":") && !current_crate.is_empty() {
                    if let Some(start) = line.find("\"repository\":\"") {
                        let start = start + 14;
                        if let Some(end) = line[start..].find('"') {
                            let repo_url = line[start..start + end].to_string();
                            self.cargo_metadata.insert(current_crate.clone(), repo_url);
                        }
                    }
                }
            }
        }
        
        // Update entries with cargo metadata
        for (crate_name, entry) in &mut self.entries {
            if let Some(metadata) = self.cargo_metadata.get(crate_name) {
                entry.cargo_metadata = Some(metadata.clone());
                entry.repo_url = Some(metadata.clone());
            }
        }
        
        println!("  ✓ Loaded metadata for {} crates", self.cargo_metadata.len());
        Ok(())
    }
    
    fn step3_map_repo_forks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🍴 STEP 3: Mapping repository forks...");
        
        // Parse .gitmodules for fork URLs
        if let Ok(gitmodules) = fs::read_to_string("../.gitmodules") {
            let mut current_path = String::new();
            
            for line in gitmodules.lines() {
                let line = line.trim();
                
                if line.starts_with("[submodule") {
                    current_path.clear();
                } else if line.starts_with("path = ") {
                    current_path = line.replace("path = ", "");
                } else if line.starts_with("url = ") && !current_path.is_empty() {
                    let fork_url = line.replace("url = ", "");
                    self.submodule_repos.insert(current_path.clone(), fork_url);
                }
            }
        }
        
        // Match forks to entries
        for (crate_name, entry) in &mut self.entries {
            // Look for matching submodule
            for (submodule_path, fork_url) in &self.submodule_repos {
                if submodule_path.to_lowercase().contains(&crate_name.to_lowercase()) ||
                   crate_name.to_lowercase().contains(&submodule_path.to_lowercase().replace("submodules/", "")) {
                    entry.fork_url = Some(fork_url.clone());
                    entry.submodule_path = Some(submodule_path.clone());
                    break;
                }
            }
        }
        
        println!("  ✓ Mapped {} submodule forks", self.submodule_repos.len());
        Ok(())
    }
    
    fn step4_get_branches(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌿 STEP 4: Getting branch information...");
        
        for (crate_name, entry) in &mut self.entries {
            if let Some(fork_url) = &entry.fork_url {
                // Get default branch
                let output = Command::new("git")
                    .args(&["ls-remote", "--symref", fork_url, "HEAD"])
                    .output();
                
                if let Ok(result) = output {
                    if result.status.success() {
                        let output_str = String::from_utf8_lossy(&result.stdout);
                        for line in output_str.lines() {
                            if line.starts_with("ref: refs/heads/") {
                                let branch = line.replace("ref: refs/heads/", "").trim().to_string();
                                entry.branch = Some(branch);
                                break;
                            }
                        }
                    }
                }
                
                // Fallback to main/master
                if entry.branch.is_none() {
                    entry.branch = Some("main".to_string());
                }
            }
        }
        
        println!("  ✓ Retrieved branch info");
        Ok(())
    }
    
    fn step5_scan_cargo_tomls(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 STEP 5: Scanning Cargo.toml files...");
        
        // Find all Cargo.toml files in submodules
        let output = Command::new("find")
            .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
            .output()?;
        
        if output.status.success() {
            let cargo_files = String::from_utf8_lossy(&output.stdout);
            
            for toml_path in cargo_files.lines() {
                if let Ok(content) = fs::read_to_string(toml_path) {
                    // Extract package name
                    let mut package_name = None;
                    let mut dependencies = Vec::new();
                    
                    for line in content.lines() {
                        if line.trim().starts_with("name = ") {
                            package_name = line.split('"').nth(1).map(|s| s.to_string());
                        }
                    }
                    
                    // Extract dependencies
                    let mut in_deps = false;
                    for line in content.lines() {
                        let line = line.trim();
                        if line.starts_with("[dependencies") {
                            in_deps = true;
                            continue;
                        }
                        if line.starts_with('[') && in_deps {
                            in_deps = false;
                        }
                        if in_deps && line.contains('=') && !line.starts_with('#') {
                            if let Some(dep_name) = line.split('=').next() {
                                dependencies.push(dep_name.trim().replace('"', ""));
                            }
                        }
                    }
                    
                    // Match to entries
                    if let Some(pkg_name) = &package_name {
                        for (crate_name, entry) in &mut self.entries {
                            if crate_name == pkg_name || 
                               toml_path.contains(&format!("/{}/", crate_name)) {
                                entry.cargo_toml_path = Some(toml_path.to_string());
                                entry.package_name = package_name.clone();
                                entry.dependencies = dependencies.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        println!("  ✓ Scanned Cargo.toml files");
        Ok(())
    }
    
    fn step6_get_git_objects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 STEP 6: Getting git object hashes...");
        
        for (crate_name, entry) in &mut self.entries {
            if let Some(submodule_path) = &entry.submodule_path {
                // Get git object hash
                let output = Command::new("git")
                    .args(&["submodule", "status", submodule_path])
                    .current_dir("..")
                    .output();
                
                if let Ok(result) = output {
                    if result.status.success() {
                        let status_line = String::from_utf8_lossy(&result.stdout);
                        if let Some(hash) = status_line.split_whitespace().next() {
                            entry.git_object = Some(hash.trim_start_matches(' ').to_string());
                        }
                    }
                }
            }
        }
        
        println!("  ✓ Retrieved git object hashes");
        Ok(())
    }
    
    fn generate_complete_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Generating complete dependency pipeline report...");
        
        let mut report = String::new();
        report.push_str("# Complete Dependency Pipeline Report\n\n");
        
        report.push_str("## Pipeline Flow\n");
        report.push_str("rustc → crate → cargo metadata → repo → forks → branches → submodules → cargo.toml → package name → git object → database entry → deps\n\n");
        
        report.push_str("## Complete Dependency Entries\n\n");
        
        for (crate_name, entry) in &self.entries {
            report.push_str(&format!("### {}\n", crate_name));
            report.push_str(&format!("- **Rustc Crate**: `{}`\n", entry.rustc_crate));
            
            if let Some(metadata) = &entry.cargo_metadata {
                report.push_str(&format!("- **Cargo Metadata**: ✅\n"));
            } else {
                report.push_str(&format!("- **Cargo Metadata**: ❌\n"));
            }
            
            if let Some(repo_url) = &entry.repo_url {
                report.push_str(&format!("- **Repo URL**: `{}`\n", repo_url));
            }
            
            if let Some(fork_url) = &entry.fork_url {
                report.push_str(&format!("- **Fork URL**: `{}`\n", fork_url));
            }
            
            if let Some(branch) = &entry.branch {
                report.push_str(&format!("- **Branch**: `{}`\n", branch));
            }
            
            if let Some(submodule_path) = &entry.submodule_path {
                report.push_str(&format!("- **Submodule Path**: `{}`\n", submodule_path));
            }
            
            if let Some(cargo_toml) = &entry.cargo_toml_path {
                report.push_str(&format!("- **Cargo.toml**: `{}`\n", cargo_toml));
            }
            
            if let Some(package_name) = &entry.package_name {
                report.push_str(&format!("- **Package Name**: `{}`\n", package_name));
            }
            
            if let Some(git_object) = &entry.git_object {
                report.push_str(&format!("- **Git Object**: `{}`\n", git_object));
            }
            
            if !entry.dependencies.is_empty() {
                report.push_str(&format!("- **Dependencies**: {}\n", entry.dependencies.len()));
            }
            
            report.push_str("\n");
        }
        
        // Summary statistics
        let complete_entries = self.entries.values()
            .filter(|e| e.cargo_metadata.is_some() && e.submodule_path.is_some() && e.cargo_toml_path.is_some())
            .count();
        
        report.push_str("## Pipeline Statistics\n");
        report.push_str(&format!("- Total rustc crates: {}\n", self.entries.len()));
        report.push_str(&format!("- With cargo metadata: {}\n", self.entries.values().filter(|e| e.cargo_metadata.is_some()).count()));
        report.push_str(&format!("- With submodules: {}\n", self.entries.values().filter(|e| e.submodule_path.is_some()).count()));
        report.push_str(&format!("- With Cargo.toml: {}\n", self.entries.values().filter(|e| e.cargo_toml_path.is_some()).count()));
        report.push_str(&format!("- Complete pipeline: {}\n", complete_entries));
        
        let completion_rate = (complete_entries as f64 / self.entries.len() as f64) * 100.0;
        report.push_str(&format!("- Completion rate: {:.1}%\n", completion_rate));
        
        fs::write("COMPLETE_DEPENDENCY_PIPELINE.md", &report)?;
        
        println!("  ✓ Report written to COMPLETE_DEPENDENCY_PIPELINE.md");
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Complete Dependency Pipeline Analysis");
        
        self.step1_load_rustc_crates()?;
        self.step2_load_cargo_metadata()?;
        self.step3_map_repo_forks()?;
        self.step4_get_branches()?;
        self.step5_scan_cargo_tomls()?;
        self.step6_get_git_objects()?;
        self.generate_complete_report()?;
        
        println!("\n🎯 === COMPLETE PIPELINE ANALYSIS ===");
        
        let complete_entries = self.entries.values()
            .filter(|e| e.cargo_metadata.is_some() && e.submodule_path.is_some() && e.cargo_toml_path.is_some())
            .count();
        
        println!("  Total entries: {}", self.entries.len());
        println!("  Complete pipeline: {}", complete_entries);
        println!("  Completion rate: {:.1}%", (complete_entries as f64 / self.entries.len() as f64) * 100.0);
        
        println!("\n🎉 COMPLETE DEPENDENCY PIPELINE READY FOR ROCKSDB!");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pipeline = CompleteDependencyPipeline::new();
    pipeline.run()
}
