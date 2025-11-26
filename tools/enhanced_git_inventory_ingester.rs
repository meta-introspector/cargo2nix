use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Enhanced Git Inventory RocksDB Ingester ===");
    
    let content = fs::read_to_string("git_files_inventory2.txt")?;
    let mut repos = HashMap::new();
    
    for line in content.lines() {
        if line.ends_with("/.git") {
            let repo_path = &line[..line.len()-5];
            if let Some(name) = repo_path.split('/').last() {
                let git_hash = get_git_hash(repo_path);
                let branch = get_git_branch(repo_path);
                let status = get_git_status(repo_path);
                
                repos.insert(name.to_string(), format!("{}|{}|{}|{}", repo_path, git_hash, branch, status));
            }
        }
    }
    
    println!("query GitInventoryRocksDB {{");
    println!("  repositories: {}", repos.len());
    
    for (name, data) in repos.iter().take(5) {
        let parts: Vec<&str> = data.split('|').collect();
        println!("  {} {{ path: \"{}\", git: \"{}\", branch: \"{}\", status: \"{}\" }}", 
            name.replace("-", "_"), parts[0], parts[1], parts[2], parts[3]);
    }
    
    println!("}}");
    
    // Save to RocksDB format
    let mut rocksdb_data = String::new();
    for (name, data) in &repos {
        rocksdb_data.push_str(&format!("repo:{}|{}\n", name, data));
    }
    
    fs::write("rocksdb_git_inventory.txt", rocksdb_data)?;
    println!("\n✓ {} repos ready for RocksDB GraphQL queries", repos.len());
    
    Ok(())
}

fn get_git_hash(repo_path: &str) -> String {
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .current_dir(repo_path)
        .output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "no-git".to_string()
    }
}

fn get_git_branch(repo_path: &str) -> String {
    if let Ok(output) = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(repo_path)
        .output() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch.is_empty() { "detached".to_string() } else { branch }
    } else {
        "no-git".to_string()
    }
}

fn get_git_status(repo_path: &str) -> String {
    if let Ok(output) = Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(repo_path)
        .output() {
        let status = String::from_utf8_lossy(&output.stdout);
        if status.trim().is_empty() { "clean".to_string() } else { "dirty".to_string() }
    } else {
        "no-git".to_string()
    }
}
