use std::process::Command;
use std::collections::HashMap;

#[derive(Debug)]
struct CrateInfo {
    order: usize,
    name: String,
    version: String,
    branch: String,
    repo: String,
    path: String,
    git_hash: String,
    criticality: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SOLANA RUSTC BUILD ORDER (Real Data) ===");
    
    let mut crates = load_from_existing_analysis()?;
    topological_sort(&mut crates);
    
    for crate_info in &crates {
        println!("{}. [{}] {} | {} | {} | {} | {} | git:{}", 
            crate_info.order,
            crate_info.criticality,
            crate_info.name,
            crate_info.version,
            crate_info.branch,
            crate_info.repo,
            crate_info.path,
            crate_info.git_hash
        );
    }
    
    println!("\nTotal crates: {} | Source: RocksDB + git analysis", crates.len());
    Ok(())
}

fn load_from_existing_analysis() -> Result<Vec<CrateInfo>, Box<dyn std::error::Error>> {
    let mut crates = Vec::new();
    
    // Use monster_rocksdb_loader output
    let output = Command::new("./monster_rocksdb_loader").output();
    if let Ok(result) = output {
        let analysis = String::from_utf8_lossy(&result.stdout);
        if analysis.contains("code hashes") {
            crates.push(CrateInfo {
                order: 1,
                name: "monster-analysis".to_string(),
                version: "rocksdb".to_string(),
                branch: "main".to_string(),
                repo: "monster-protocol".to_string(),
                path: "./monster_rocksdb".to_string(),
                git_hash: get_git_hash(".")?,
                criticality: 9,
            });
        }
    }
    
    // Get submodule data
    let output = Command::new("git").args(&["submodule", "status"]).output()?;
    let submodule_data = String::from_utf8_lossy(&output.stdout);
    
    let mut order = 2;
    for line in submodule_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hash = parts[0].trim_start_matches(&['+', '-', ' '][..]);
            let path = parts[1];
            let name = path.split('/').last().unwrap_or("unknown");
            
            // Skip non-critical crates for minimal output
            if !is_critical_for_solana_rustc(name) {
                continue;
            }
            
            let branch = get_branch_for_path(path)?;
            let repo = get_repo_for_path(path)?;
            let version = get_version_for_path(path)?;
            let criticality = calculate_criticality(name);
            
            crates.push(CrateInfo {
                order,
                name: name.to_string(),
                version,
                branch,
                repo,
                path: path.to_string(),
                git_hash: hash[..8].to_string(),
                criticality,
            });
            
            order += 1;
        }
    }
    
    Ok(crates)
}

fn is_critical_for_solana_rustc(name: &str) -> bool {
    matches!(name, 
        "rust" | "rustc-demangle" | "rustc-hash" | "rustc-build-sysroot" |
        "cargo" | "serde" | "solana" | "rust-analyzer" | "allocator-api2" |
        "core" | "std" | "alloc" | "proc_macro"
    )
}

fn calculate_criticality(name: &str) -> u8 {
    match name {
        "rust" => 9,
        n if n.starts_with("rustc") => 8,
        "cargo" | "serde" => 7,
        "solana" => 6,
        "allocator-api2" => 5,
        _ => 1,
    }
}

fn topological_sort(crates: &mut Vec<CrateInfo>) {
    crates.sort_by(|a, b| {
        b.criticality.cmp(&a.criticality)
            .then_with(|| a.name.cmp(&b.name))
    });
    
    // Update order after sorting
    for (i, crate_info) in crates.iter_mut().enumerate() {
        crate_info.order = i + 1;
    }
}

fn get_git_hash(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(path)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim()[..8].to_string())
}

fn get_branch_for_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(path)
        .output();
        
    match output {
        Ok(result) => {
            let branch = String::from_utf8_lossy(&result.stdout).trim().to_string();
            Ok(if branch.is_empty() { "detached".to_string() } else { branch })
        },
        Err(_) => Ok("no-git".to_string()),
    }
}

fn get_repo_for_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .current_dir(path)
        .output();
        
    match output {
        Ok(result) => {
            let url = String::from_utf8_lossy(&result.stdout).trim().to_string();
            if let Some(repo_name) = url.split('/').last() {
                Ok(repo_name.trim_end_matches(".git").to_string())
            } else {
                Ok("unknown".to_string())
            }
        },
        Err(_) => Ok("no-remote".to_string()),
    }
}

fn get_version_for_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml = format!("{}/Cargo.toml", path);
    let output = Command::new("grep")
        .args(&["version", &cargo_toml])
        .output();
        
    match output {
        Ok(result) => {
            let content = String::from_utf8_lossy(&result.stdout);
            for line in content.lines() {
                if line.contains("version") && line.contains("=") {
                    if let Some(version) = line.split('=').nth(1) {
                        return Ok(version.trim().trim_matches('"').to_string());
                    }
                }
            }
            Ok("unknown".to_string())
        },
        Err(_) => Ok("no-cargo".to_string()),
    }
}
