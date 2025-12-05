use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Simple Submodule Checker");
    
    // Check if BLAKE3 is initialized
    let blake3_path = "../submodules/BLAKE3";
    
    println!("📦 Checking BLAKE3 submodule...");
    let output = Command::new("ls")
        .args(&["-la", blake3_path])
        .output()?;
    
    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.contains("Cargo.toml") {
            println!("  ✅ BLAKE3 has Cargo.toml");
        } else {
            println!("  ❌ BLAKE3 missing Cargo.toml - not initialized");
            
            // Try to initialize it
            println!("  🔧 Initializing BLAKE3...");
            let init_output = Command::new("git")
                .args(&["submodule", "update", "--init", "submodules/BLAKE3"])
                .current_dir("..")
                .output()?;
            
            if init_output.status.success() {
                println!("    ✅ BLAKE3 initialized successfully");
            } else {
                println!("    ❌ Failed to initialize: {}", String::from_utf8_lossy(&init_output.stderr));
            }
        }
    }
    
    // Check rust-base64
    println!("\n📦 Checking rust-base64 submodule...");
    let base64_path = "../submodules/rust-base64";
    let output = Command::new("ls")
        .args(&["-la", base64_path])
        .output()?;
    
    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.contains("Cargo.toml") {
            println!("  ✅ rust-base64 has Cargo.toml");
        } else {
            println!("  ❌ rust-base64 missing Cargo.toml - not initialized");
        }
    } else {
        println!("  ❌ rust-base64 directory not found");
    }
    
    // Now re-scan for Cargo.toml files
    println!("\n🔍 Re-scanning for Cargo.toml files...");
    let output = Command::new("find")
        .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
        .output()?;
    
    let cargo_files = String::from_utf8_lossy(&output.stdout);
    let count = cargo_files.lines().count();
    
    println!("📋 Found {} Cargo.toml files:", count);
    for (i, file) in cargo_files.lines().enumerate() {
        if i < 10 {
            let short_path = file.replace("../submodules/", "");
            println!("  {}", short_path);
        }
    }
    
    if count > 10 {
        println!("  ... and {} more", count - 10);
    }
    
    Ok(())
}
