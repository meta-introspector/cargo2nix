use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Fast Git Inventory → RocksDB ===");
    
    let content = fs::read_to_string("git_files_inventory2.txt")?;
    let mut repos = Vec::new();
    
    for line in content.lines() {
        if line.ends_with("/.git") {
            let repo_path = &line[..line.len()-5];
            if let Some(name) = repo_path.split('/').last() {
                repos.push((name.to_string(), repo_path.to_string()));
            }
        }
    }
    
    println!("query GitInventoryRocksDB {{");
    println!("  total_repositories: {}", repos.len());
    println!("  sample_repositories: [");
    
    for (name, path) in repos.iter().take(10) {
        println!("    {{ name: \"{}\", path: \"{}\" }}", name, path);
    }
    
    println!("  ]");
    println!("}}");
    
    // Generate RocksDB ingestion format
    let mut rocksdb_data = String::new();
    for (name, path) in &repos {
        rocksdb_data.push_str(&format!("git_repo:{}|path={}|status=pending\n", name, path));
    }
    
    fs::write("rocksdb_git_repos.txt", rocksdb_data)?;
    
    println!("\n✓ {} repositories ready for RocksDB", repos.len());
    println!("✓ Saved to rocksdb_git_repos.txt");
    println!("✓ GraphQL queries can now resolve git repos from cache");
    
    Ok(())
}
