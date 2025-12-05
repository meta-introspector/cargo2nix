use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut git_repos = HashMap::new();
    
    // Extract from Cargo.lock files
    extract_from_cargo_metadata(&mut git_repos)?;
    
    // Read existing .gitmodules
    read_gitmodules("../.gitmodules", &mut git_repos)?;
    read_gitmodules(".gitmodules", &mut git_repos)?;
    
    // Map to meta-introspector equivalents
    map_to_meta_introspector(&mut git_repos);
    
    // Generate new .gitmodules entries
    generate_gitmodules_entries(&git_repos)?;
    
    Ok(())
}

fn extract_from_cargo_metadata(repos: &mut HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1"])
        .output()?;
    
    if !output.status.success() {
        return Ok(());
    }
    
    let metadata = String::from_utf8(output.stdout)?;
    
    // Simple parsing for git URLs
    for line in metadata.lines() {
        if line.contains("\"source\":") && line.contains("git+") {
            if let Some(start) = line.find("git+") {
                if let Some(end) = line[start..].find("\"") {
                    let git_url = &line[start+4..start+end];
                    if let Some(name) = extract_repo_name(git_url) {
                        repos.insert(name, git_url.to_string());
                    }
                }
            }
        }
    }
    
    println!("Extracted {} git repos from cargo metadata", repos.len());
    Ok(())
}

fn read_gitmodules(path: &str, repos: &mut HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(content) = fs::read_to_string(path) {
        let mut current_name = String::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("[submodule \"") {
                current_name = line[12..line.len()-2].to_string();
            } else if line.starts_with("url = ") && !current_name.is_empty() {
                let url = &line[6..];
                repos.insert(current_name.clone(), url.to_string());
                current_name.clear();
            }
        }
        println!("Read {} entries from {}", repos.len(), path);
    }
    Ok(())
}

fn extract_repo_name(url: &str) -> Option<String> {
    url.split('/').last()?.strip_suffix(".git").map(|s| s.to_string())
}

fn map_to_meta_introspector(repos: &mut HashMap<String, String>) {
    let meta_mappings = vec![
        ("rust", "https://github.com/meta-introspector/rust.git"),
        ("cargo", "https://github.com/meta-introspector/cargo.git"),
        ("serde", "https://github.com/meta-introspector/serde.git"),
        ("tokio", "https://github.com/meta-introspector/tokio.git"),
        ("clap", "https://github.com/meta-introspector/clap.git"),
        ("hyper", "https://github.com/meta-introspector/hyper.git"),
        ("regex", "https://github.com/meta-introspector/regex.git"),
    ];
    
    for (name, meta_url) in meta_mappings {
        if repos.contains_key(name) {
            repos.insert(format!("{}-meta", name), meta_url.to_string());
        }
    }
}

fn generate_gitmodules_entries(repos: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = String::new();
    
    for (name, url) in repos {
        entries.push_str(&format!(
            "[submodule \"{}\"]\n\tpath = submodules/{}\n\turl = {}\n\n",
            name, name, url
        ));
    }
    
    fs::write("new_gitmodules_entries.txt", &entries)?;
    
    // Also generate pull commands
    let mut commands = String::new();
    for (name, url) in repos {
        commands.push_str(&format!(
            "git submodule add {} submodules/{}\n",
            url, name
        ));
    }
    
    fs::write("pull_commands.sh", &commands)?;
    
    println!("Generated {} .gitmodules entries", repos.len());
    println!("Saved to new_gitmodules_entries.txt and pull_commands.sh");
    
    Ok(())
}
