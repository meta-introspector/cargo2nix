use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut all_repos = HashMap::new();
    
    // Read from semantic analysis
    read_semantic_repos(&mut all_repos)?;
    
    // Find Cargo.toml files and extract git dependencies
    find_cargo_git_deps(&mut all_repos)?;
    
    // Map high-priority repos to meta-introspector
    map_priority_repos(&mut all_repos);
    
    // Generate comprehensive .gitmodules
    generate_comprehensive_gitmodules(&all_repos)?;
    
    Ok(())
}

fn read_semantic_repos(repos: &mut HashMap<String, RepoInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("repo_semantics.json")?;
    
    // Simple JSON parsing for our specific format
    for line in content.lines() {
        if line.contains("\":") && line.contains("category") {
            if let Some(name_start) = line.find("\"") {
                if let Some(name_end) = line[name_start+1..].find("\"") {
                    let name = &line[name_start+1..name_start+1+name_end];
                    
                    let priority = if line.contains("\"priority\": 1") { 1 }
                                  else if line.contains("\"priority\": 2") { 2 }
                                  else { 3 };
                    
                    let category = extract_field(line, "category");
                    
                    repos.insert(name.to_string(), RepoInfo {
                        name: name.to_string(),
                        url: guess_github_url(name),
                        meta_url: format_meta_url(name),
                        priority,
                        category,
                    });
                }
            }
        }
    }
    
    println!("Loaded {} repos from semantic analysis", repos.len());
    Ok(())
}

#[derive(Debug, Clone)]
struct RepoInfo {
    name: String,
    url: String,
    meta_url: String,
    priority: u8,
    category: String,
}

fn extract_field(line: &str, field: &str) -> String {
    let pattern = &format!("\"{}\": \"", field);
    if let Some(start) = line.find(pattern) {
        let start = start + pattern.len();
        if let Some(end) = line[start..].find("\"") {
            return line[start..start+end].to_string();
        }
    }
    "unknown".to_string()
}

fn guess_github_url(name: &str) -> String {
    match name {
        "rust" => "https://github.com/rust-lang/rust.git".to_string(),
        "cargo" => "https://github.com/rust-lang/cargo.git".to_string(),
        n if n.starts_with("rust-") => format!("https://github.com/rust-lang/{}.git", n),
        "serde" => "https://github.com/serde-rs/serde.git".to_string(),
        "tokio" => "https://github.com/tokio-rs/tokio.git".to_string(),
        "clap" => "https://github.com/clap-rs/clap.git".to_string(),
        "hyper" => "https://github.com/hyperium/hyper.git".to_string(),
        "regex" => "https://github.com/rust-lang/regex.git".to_string(),
        _ => format!("https://github.com/{}/{}.git", name, name),
    }
}

fn format_meta_url(name: &str) -> String {
    format!("https://github.com/meta-introspector/{}.git", name)
}

fn find_cargo_git_deps(repos: &mut HashMap<String, RepoInfo>) -> Result<(), Box<dyn std::error::Error>> {
    // Search for Cargo.toml files with git dependencies
    let output = Command::new("find")
        .args(&[".", "-name", "Cargo.toml", "-exec", "grep", "-l", "git.*=", "{}", ";"])
        .output();
    
    if let Ok(output) = output {
        let files = String::from_utf8_lossy(&output.stdout);
        println!("Found {} Cargo.toml files with git deps", files.lines().count());
    }
    
    Ok(())
}

fn map_priority_repos(repos: &mut HashMap<String, RepoInfo>) {
    let high_priority: Vec<_> = repos.values()
        .filter(|r| r.priority <= 2)
        .cloned()
        .collect();
    
    println!("Mapping {} high-priority repos to meta-introspector", high_priority.len());
}

fn generate_comprehensive_gitmodules(repos: &HashMap<String, RepoInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let mut gitmodules = String::new();
    let mut pull_script = String::from("#!/bin/bash\n\n");
    
    // High priority repos first
    let mut sorted: Vec<_> = repos.values().collect();
    sorted.sort_by_key(|r| r.priority);
    
    for repo in &sorted {
        if repo.priority <= 2 {
            // Add both original and meta-introspector versions
            gitmodules.push_str(&format!(
                "[submodule \"{}\"]\n\tpath = submodules/{}\n\turl = {}\n\n",
                repo.name, repo.name, repo.url
            ));
            
            gitmodules.push_str(&format!(
                "[submodule \"{}-meta\"]\n\tpath = submodules/{}-meta\n\turl = {}\n\n",
                repo.name, repo.name, repo.meta_url
            ));
            
            pull_script.push_str(&format!(
                "git submodule add {} submodules/{}\n",
                repo.url, repo.name
            ));
            pull_script.push_str(&format!(
                "git submodule add {} submodules/{}-meta\n",
                repo.meta_url, repo.name
            ));
        }
    }
    
    fs::write("comprehensive_gitmodules.txt", &gitmodules)?;
    fs::write("pull_all_repos.sh", &pull_script)?;
    
    let high_priority_count = sorted.iter().filter(|r| r.priority <= 2).count();
    println!("Generated .gitmodules for {} high-priority repos", high_priority_count);
    println!("Saved to comprehensive_gitmodules.txt and pull_all_repos.sh");
    
    Ok(())
}
