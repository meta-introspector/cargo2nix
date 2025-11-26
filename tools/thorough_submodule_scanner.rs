use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Thorough Submodule Scanner");
    
    // First, check how many submodules we have
    let output = Command::new("ls")
        .args(&["../submodules/"])
        .output()?;
    
    let submodules = String::from_utf8_lossy(&output.stdout);
    let submodule_count = submodules.lines().count();
    println!("📂 Total submodules: {}", submodule_count);
    
    // Find all Cargo.toml files
    let output = Command::new("find")
        .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
        .output()?;
    
    let cargo_files = String::from_utf8_lossy(&output.stdout);
    let cargo_count = cargo_files.lines().count();
    println!("📦 Total Cargo.toml files: {}", cargo_count);
    
    // Extract crate names
    let mut crate_names = HashMap::new();
    let mut processed = 0;
    
    for toml_path in cargo_files.lines() {
        if let Ok(content) = fs::read_to_string(toml_path) {
            for line in content.lines() {
                if line.trim().starts_with("name = ") {
                    if let Some(name) = line.split('"').nth(1) {
                        crate_names.insert(name.to_string(), toml_path.to_string());
                        processed += 1;
                        break;
                    }
                }
            }
        }
    }
    
    println!("✅ Processed {} Cargo.toml files", processed);
    println!("📋 Found {} unique crate names", crate_names.len());
    
    // Check for specific crates we're looking for
    let target_crates = vec![
        "base64", "rust-base64", "blake3", "aes", "backtrace", 
        "ansi_term", "askama", "bytecount", "annotate-snippets"
    ];
    
    println!("\n🎯 Checking for target crates:");
    for target in &target_crates {
        if crate_names.contains_key(target) {
            println!("  ✅ {} → {}", target, crate_names[target]);
        } else {
            // Check for similar names
            let mut found_similar = false;
            for (crate_name, path) in &crate_names {
                if crate_name.contains(target) || target.contains(crate_name) {
                    println!("  🔍 {} (similar to {}) → {}", crate_name, target, path);
                    found_similar = true;
                }
            }
            if !found_similar {
                println!("  ❌ {} not found", target);
            }
        }
    }
    
    // Show first 20 crates we found
    println!("\n📋 Sample of found crates:");
    for (i, (name, path)) in crate_names.iter().enumerate() {
        if i < 20 {
            let short_path = path.replace("../submodules/", "").replace("/Cargo.toml", "");
            println!("  {} → {}", name, short_path);
        }
    }
    
    if crate_names.len() > 20 {
        println!("  ... and {} more", crate_names.len() - 20);
    }
    
    Ok(())
}
