use std::process::Command;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Fixing git submodule status issues");
    
    // Get submodule status and capture errors
    let output = Command::new("git")
        .args(&["submodule", "status"])
        .current_dir("..")
        .output()?;
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    println!("📊 Current status:");
    println!("  Working submodules: {}", stdout.lines().count());
    
    // Find problematic submodules
    let mut problematic = Vec::new();
    for line in stderr.lines() {
        if line.contains("no submodule mapping found") {
            if let Some(path) = extract_path_from_error(line) {
                problematic.push(path);
            }
        }
    }
    
    println!("  Problematic submodules: {}", problematic.len());
    
    // Fix each problematic submodule
    for path in &problematic {
        println!("🗑️ Removing orphaned submodule: {}", path);
        
        // Remove the directory
        let full_path = format!("../{}", path);
        if std::path::Path::new(&full_path).exists() {
            fs::remove_dir_all(&full_path)?;
            println!("  ✓ Removed directory: {}", path);
        }
        
        // Remove from git index
        let _ = Command::new("git")
            .args(&["rm", "--cached", path])
            .current_dir("..")
            .output();
    }
    
    // Clean up git modules
    for path in &problematic {
        let modules_path = format!("../.git/modules/{}", path);
        if std::path::Path::new(&modules_path).exists() {
            fs::remove_dir_all(&modules_path)?;
            println!("  ✓ Cleaned git modules: {}", path);
        }
    }
    
    // Verify fix
    let verify_output = Command::new("git")
        .args(&["submodule", "status"])
        .current_dir("..")
        .output()?;
    
    let verify_stderr = String::from_utf8_lossy(&verify_output.stderr);
    let verify_stdout = String::from_utf8_lossy(&verify_output.stdout);
    
    println!("\n✅ VERIFICATION:");
    println!("  Working submodules: {}", verify_stdout.lines().count());
    println!("  Errors remaining: {}", verify_stderr.lines().count());
    
    if verify_stderr.trim().is_empty() {
        println!("  🎉 All submodule issues fixed!");
    } else {
        println!("  ⚠️ Some issues remain:");
        for line in verify_stderr.lines().take(3) {
            println!("    {}", line);
        }
    }
    
    Ok(())
}

fn extract_path_from_error(error_line: &str) -> Option<String> {
    // Extract path from: "fatal: no submodule mapping found in .gitmodules for path 'submodules/annotate-snippets'"
    if let Some(start) = error_line.find("for path '") {
        let start = start + 10; // Length of "for path '"
        if let Some(end) = error_line[start..].find("'") {
            return Some(error_line[start..start + end].to_string());
        }
    }
    None
}
