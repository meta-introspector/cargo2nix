use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Initialize and Scan Submodules");
    
    // Get list of uninitialized submodules
    let output = Command::new("git")
        .args(&["submodule", "status"])
        .current_dir("..")
        .output()?;
    
    let status = String::from_utf8_lossy(&output.stdout);
    let mut uninitialized = Vec::new();
    let mut initialized = Vec::new();
    
    for line in status.lines() {
        if line.starts_with('-') {
            // Uninitialized submodule
            if let Some(path) = line.split_whitespace().nth(1) {
                uninitialized.push(path.to_string());
            }
        } else if line.starts_with(' ') {
            // Initialized submodule
            if let Some(path) = line.split_whitespace().nth(1) {
                initialized.push(path.to_string());
            }
        }
    }
    
    println!("📊 Submodule Status:");
    println!("  Initialized: {}", initialized.len());
    println!("  Uninitialized: {}", uninitialized.len());
    
    // Initialize a few key submodules we need for rustc
    let key_submodules = vec![
        "submodules/BLAKE3",
        "submodules/rust-base64", 
        "submodules/aes",
        "submodules/backtrace",
        "submodules/ansi_term"
    ];
    
    println!("\n🔧 Initializing key submodules...");
    let mut initialized_count = 0;
    
    for submodule in &key_submodules {
        if uninitialized.contains(submodule) {
            println!("  Initializing {}...", submodule);
            
            let init_output = Command::new("git")
                .args(&["submodule", "update", "--init", submodule])
                .current_dir("..")
                .output()?;
            
            if init_output.status.success() {
                println!("    ✅ Success");
                initialized_count += 1;
            } else {
                println!("    ❌ Failed: {}", String::from_utf8_lossy(&init_output.stderr));
            }
        } else if initialized.contains(submodule) {
            println!("  {} already initialized ✓", submodule);
        } else {
            println!("  {} not found in .gitmodules", submodule);
        }
    }
    
    println!("\n📦 Scanning for Cargo.toml files in initialized submodules...");
    
    // Now scan for Cargo.toml files
    let output = Command::new("find")
        .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
        .output()?;
    
    let cargo_files = String::from_utf8_lossy(&output.stdout);
    let mut crate_names = HashMap::new();
    
    for toml_path in cargo_files.lines() {
        if let Ok(content) = std::fs::read_to_string(toml_path) {
            for line in content.lines() {
                if line.trim().starts_with("name = ") {
                    if let Some(name) = line.split('"').nth(1) {
                        crate_names.insert(name.to_string(), toml_path.to_string());
                        break;
                    }
                }
            }
        }
    }
    
    println!("📋 Found {} crates in submodules:", crate_names.len());
    
    // Check for our target crates
    let targets = vec!["blake3", "base64", "aes", "backtrace", "ansi_term"];
    
    for target in &targets {
        let target_string = target.to_string();
        if crate_names.contains_key(&target_string) {
            println!("  ✅ {} found", target);
        } else {
            // Check for similar names
            let mut found = false;
            for (name, path) in &crate_names {
                if name.contains(target) || target.contains(name) {
                    println!("  🔍 {} (similar to {}) at {}", name, target, path);
                    found = true;
                }
            }
            if !found {
                println!("  ❌ {} not found", target);
            }
        }
    }
    
    println!("\n🎯 SUMMARY:");
    println!("  Initialized {} new submodules", initialized_count);
    println!("  Found {} total crates", crate_names.len());
    println!("  Ready for Monster Protocol cross-reference!");
    
    Ok(())
}
