use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cargo.toml Extractor for RocksDB ===");
    
    let content = fs::read_to_string("git_files_inventory2.txt")?;
    let mut cargo_files = Vec::new();
    let mut processed_repos = 0;
    
    for line in content.lines() {
        if line.ends_with("/.git") {
            let repo_path = &line[..line.len()-5];
            
            // Find Cargo.toml files in this repo
            if let Ok(output) = Command::new("find")
                .args(&[repo_path, "-name", "Cargo.toml"])
                .output() {
                
                let files = String::from_utf8_lossy(&output.stdout);
                for cargo_path in files.lines() {
                    if let Ok(content) = fs::read_to_string(cargo_path) {
                        let name = extract_package_name(&content);
                        let version = extract_package_version(&content);
                        
                        cargo_files.push(format!("cargo:{}:{}|path={}|repo={}", 
                            name, version, cargo_path, repo_path));
                    }
                }
            }
            
            processed_repos += 1;
            if processed_repos % 1000 == 0 {
                println!("  Processed {} repos, found {} Cargo.toml files", 
                    processed_repos, cargo_files.len());
            }
        }
    }
    
    println!("\nquery CargoTomlRocksDB {{");
    println!("  total_cargo_files: {}", cargo_files.len());
    println!("  sample_packages: [");
    
    for entry in cargo_files.iter().take(10) {
        let parts: Vec<&str> = entry.split('|').collect();
        let key_parts: Vec<&str> = parts[0].split(':').collect();
        if key_parts.len() >= 3 {
            println!("    {{ name: \"{}\", version: \"{}\" }}", key_parts[1], key_parts[2]);
        }
    }
    
    println!("  ]");
    println!("}}");
    
    // Save for RocksDB ingestion
    fs::write("rocksdb_cargo_files.txt", cargo_files.join("\n"))?;
    
    println!("\n✓ {} Cargo.toml files ready for RocksDB", cargo_files.len());
    println!("✓ Build reasoner can now resolve package dependencies");
    
    Ok(())
}

fn extract_package_name(content: &str) -> String {
    for line in content.lines() {
        if line.trim().starts_with("name") && line.contains('=') {
            if let Some(value) = line.split('=').nth(1) {
                return value.trim().trim_matches('"').to_string();
            }
        }
    }
    "unknown".to_string()
}

fn extract_package_version(content: &str) -> String {
    for line in content.lines() {
        if line.trim().starts_with("version") && line.contains('=') {
            if let Some(value) = line.split('=').nth(1) {
                return value.trim().trim_matches('"').to_string();
            }
        }
    }
    "0.0.0".to_string()
}
