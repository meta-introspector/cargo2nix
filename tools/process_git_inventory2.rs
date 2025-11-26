use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Processing git_files_inventory2.txt for GraphQL ingestion ===");
    
    let content = fs::read_to_string("git_files_inventory2.txt")?;
    let mut git_objects = HashMap::new();
    let mut cargo_files = Vec::new();
    let mut readme_files = Vec::new();
    let mut gitmodule_files = Vec::new();
    
    for line in content.lines() {
        if line.ends_with("/.git") {
            let repo_path = &line[..line.len()-5];
            if let Some(name) = repo_path.split('/').last() {
                git_objects.insert(name.to_string(), repo_path.to_string());
            }
        } else if line.contains("Cargo.toml") || line.contains("Cargo.lock") {
            cargo_files.push(line.to_string());
        } else if line.to_lowercase().contains("readme") {
            readme_files.push(line.to_string());
        } else if line.contains(".gitmodules") {
            gitmodule_files.push(line.to_string());
        }
    }
    
    println!("GraphQL ingestion summary:");
    println!("  Git repositories: {}", git_objects.len());
    println!("  Cargo files: {}", cargo_files.len());
    println!("  README files: {}", readme_files.len());
    println!("  .gitmodules files: {}", gitmodule_files.len());
    
    // Generate GraphQL cache data
    println!("\nquery IngestInventory {{");
    println!("  gitObjects: {} repositories", git_objects.len());
    println!("  cargoFiles: {} files", cargo_files.len());
    println!("  readmeFiles: {} files", readme_files.len());
    println!("  gitmoduleFiles: {} files", gitmodule_files.len());
    println!("}}");
    
    // Save structured data for RocksDB ingestion
    let mut output = String::new();
    output.push_str("# Git Objects\n");
    for (name, path) in &git_objects {
        output.push_str(&format!("git_repo|{}|{}\n", name, path));
    }
    
    output.push_str("\n# Cargo Files\n");
    for file in &cargo_files {
        output.push_str(&format!("cargo_file|{}\n", file));
    }
    
    output.push_str("\n# README Files\n");
    for file in &readme_files {
        output.push_str(&format!("readme_file|{}\n", file));
    }
    
    output.push_str("\n# Gitmodule Files\n");
    for file in &gitmodule_files {
        output.push_str(&format!("gitmodule_file|{}\n", file));
    }
    
    fs::write("rocksdb_ingest_data.txt", output)?;
    println!("\nSaved structured data to rocksdb_ingest_data.txt for RocksDB ingestion");
    
    Ok(())
}
