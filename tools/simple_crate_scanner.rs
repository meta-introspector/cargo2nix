use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning submodules for Rust crates...");
    
    let mut crate_count = 0;
    let mut report = String::from("# Submodule Crate Report\n\n");
    
    if let Ok(entries) = fs::read_dir("submodules") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let cargo_toml = path.join("Cargo.toml");
                    if cargo_toml.exists() {
                        let name = path.file_name().unwrap().to_string_lossy();
                        
                        // Read crate name from Cargo.toml
                        let crate_name = if let Ok(content) = fs::read_to_string(&cargo_toml) {
                            extract_crate_name(&content).unwrap_or(name.to_string())
                        } else {
                            name.to_string()
                        };
                        
                        report.push_str(&format!("{}. **{}** ({})\n", crate_count + 1, crate_name, name));
                        crate_count += 1;
                    }
                }
            }
        }
    }
    
    report.push_str(&format!("\nTotal: {} crates found\n", crate_count));
    
    fs::write("crate_scan_report.md", report)?;
    
    println!("Found {} crates", crate_count);
    println!("Report saved to crate_scan_report.md");
    
    Ok(())
}

fn extract_crate_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("name = \"") {
            if let Some(end) = line[8..].find("\"") {
                return Some(line[8..8+end].to_string());
            }
        }
    }
    None
}
