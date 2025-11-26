use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Name Search ===");
    
    // Search for solana-runtime in Cargo.toml files
    let output = Command::new("find")
        .args(&[".", "-name", "Cargo.toml", "-exec", "grep", "-l", "solana-runtime", "{}", ";"])
        .output()?;
        
    let paths = String::from_utf8_lossy(&output.stdout);
    
    println!("Found solana-runtime in:");
    for path in paths.lines() {
        println!("  {}", path);
    }
    
    // Also search for just "solana" packages
    let output2 = Command::new("find")
        .args(&[".", "-name", "Cargo.toml", "-exec", "grep", "-l", "name.*solana", "{}", ";"])
        .output()?;
        
    let paths2 = String::from_utf8_lossy(&output2.stdout);
    
    println!("\nFound solana packages in:");
    for path in paths2.lines().take(10) {
        println!("  {}", path);
    }
    
    Ok(())
}
