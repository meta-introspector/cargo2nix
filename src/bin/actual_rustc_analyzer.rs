use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Actual Rustc Use Analyzer ===");
    
    // Read our real monster solver file
    let content = fs::read_to_string("src/bin/real_monster_solver.rs")?;
    
    println!("Analyzing real_monster_solver.rs:");
    
    // Extract actual use statements
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().starts_with("use ") {
            println!("  Line {}: {}", line_num + 1, line.trim());
        }
    }
    
    // Extract actual declarations
    println!("\nActual declarations:");
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("struct ") || 
           trimmed.starts_with("enum ") || trimmed.starts_with("const ") {
            println!("  Line {}: {}", line_num + 1, trimmed);
        }
    }
    
    Ok(())
}
