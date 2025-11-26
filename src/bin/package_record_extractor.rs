use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug)]
struct PackageRecord {
    package_name: String,
    version: String,
    git_hash: String,
    branch: String,
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unique Package Records (Real Data) ===");
    
    let mut unique_packages = HashMap::new();
    
    // Find real Cargo.toml files
    let output = Command::new("find")
        .args(&[".", "-name", "Cargo.toml"])
        .output()?;
        
    let paths = String::from_utf8_lossy(&output.stdout);
    
    for path in paths.lines().take(50) {
        if let Ok(record) = extract_package_record(path) {
            let key = format!("{}:{}", record.package_name, record.version);
            unique_packages.insert(key, record);
        }
    }
    
    println!("query UniquePackages {{");
    for (key, record) in unique_packages.iter().take(10) {
        println!("  {} {{", key.replace(":", "_").replace("-", "_"));
        println!("    package_name: \"{}\"", record.package_name);
        println!("    version: \"{}\"", record.version);
        println!("    git_hash: \"{}\"", record.git_hash);
        println!("    branch: \"{}\"", record.branch);
        println!("    path: \"{}\"", record.path);
        println!("  }}");
    }
    println!("}}");
    
    println!("\nTotal unique packages: {}", unique_packages.len());
    
    Ok(())
}

fn extract_package_record(cargo_path: &str) -> Result<PackageRecord, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(cargo_path)?;
    
    let package_name = extract_field(&content, "name").unwrap_or_else(|| "unknown".to_string());
    let version = extract_field(&content, "version").unwrap_or_else(|| "0.0.0".to_string());
    
    let dir = std::path::Path::new(cargo_path).parent().unwrap_or(std::path::Path::new("."));
    let git_hash = get_git_hash(dir)?;
    let branch = get_branch(dir)?;
    
    Ok(PackageRecord {
        package_name,
        version,
        git_hash,
        branch,
        path: cargo_path.to_string(),
    })
}

fn extract_field(content: &str, field: &str) -> Option<String> {
    for line in content.lines() {
        if line.trim().starts_with(field) && line.contains('=') {
            if let Some(value) = line.split('=').nth(1) {
                return Some(value.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

fn get_git_hash(dir: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .current_dir(dir)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_branch(dir: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(dir)
        .output()?;
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(if branch.is_empty() { "detached".to_string() } else { branch })
}
