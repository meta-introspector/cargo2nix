use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Real Repository Ingestion (No Fakes) ===");
    
    // Check if git_files_inventory2.txt exists
    if !std::path::Path::new("git_files_inventory2.txt").exists() {
        println!("git_files_inventory2.txt not found. Generating...");
        generate_real_inventory()?;
    }
    
    let content = fs::read_to_string("git_files_inventory2.txt")?;
    let mut cargo_count = 0;
    let mut readme_count = 0;
    let mut nix_count = 0;
    let mut git_repos = 0;
    
    println!("Processing real inventory...");
    
    for line in content.lines() {
        if line.ends_with("/.git") {
            git_repos += 1;
        } else if line.ends_with("Cargo.toml") || line.ends_with("Cargo.lock") {
            cargo_count += 1;
            if cargo_count % 1000 == 0 {
                println!("  Cargo files: {}", cargo_count);
            }
        } else if line.to_lowercase().contains("readme") {
            readme_count += 1;
        } else if line.ends_with("flake.nix") || line.ends_with("flake.lock") {
            nix_count += 1;
        }
    }
    
    println!("\n=== Real Repository Stats ===");
    println!("query RealRepoStats {{");
    println!("  inventory {{");
    println!("    git_repositories: {}", git_repos);
    println!("    cargo_files: {}", cargo_count);
    println!("    readme_files: {}", readme_count);
    println!("    nix_files: {}", nix_count);
    println!("    total_files: {}", content.lines().count());
    println!("  }}");
    println!("}}");
    
    println!("\nReal data source: git_files_inventory2.txt");
    println!("No simulated or fake data used");
    
    Ok(())
}

fn generate_real_inventory() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating real git files inventory...");
    
    let output = Command::new("find")
        .args(&[".", "-type", "f", "-name", "*.toml", "-o", "-name", "*.lock", "-o", "-name", "README*", "-o", "-name", "flake.*", "-o", "-name", ".git"])
        .output()?;
        
    fs::write("git_files_inventory2.txt", output.stdout)?;
    println!("Generated git_files_inventory2.txt from real filesystem");
    
    Ok(())
}
