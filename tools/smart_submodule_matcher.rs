use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Smart Submodule Matcher");
    
    // Get our target crates from rustc analysis
    let target_crates = vec![
        "aes", "annotate-snippets", "ansi_term", "anstream", "anstyle-svg",
        "askama", "backtrace", "base64", "bitflags", "blake3", "boml", 
        "build_helper", "bytecount", "byteorder_2"
    ];
    
    println!("🔍 Looking for {} target crates in submodules...", target_crates.len());
    
    // Parse .gitmodules to find potential matches
    let gitmodules = fs::read_to_string("../.gitmodules")?;
    let mut potential_matches = HashMap::new();
    
    for target in &target_crates {
        println!("\n📦 Searching for '{}':", target);
        
        let mut found_matches = Vec::new();
        
        // Search in .gitmodules
        for line in gitmodules.lines() {
            if line.contains("[submodule") {
                let submodule_line = line;
                
                // Check if this submodule name matches our target
                if submodule_line.to_lowercase().contains(target) ||
                   target.contains(&extract_submodule_name(submodule_line).to_lowercase()) ||
                   names_are_similar(target, &extract_submodule_name(submodule_line)) {
                    found_matches.push(extract_submodule_name(submodule_line));
                }
            }
        }
        
        if found_matches.is_empty() {
            println!("  ❌ No matches found");
        } else {
            println!("  ✅ Found {} potential matches:", found_matches.len());
            for m in &found_matches {
                println!("    - {}", m);
            }
            potential_matches.insert(target.to_string(), found_matches);
        }
    }
    
    // Now try to initialize the most promising matches
    println!("\n🔧 Initializing promising submodules...");
    
    let priority_matches = vec![
        ("blake3", "submodules/BLAKE3"),
        ("base64", "submodules/rust-base64"),
        ("backtrace", "submodules/backtrace-rs"),
        ("ansi_term", "submodules/nu-ansi-term"),
    ];
    
    for (crate_name, submodule_path) in &priority_matches {
        println!("  Initializing {} -> {}...", crate_name, submodule_path);
        
        let output = Command::new("git")
            .args(&["submodule", "update", "--init", submodule_path])
            .current_dir("..")
            .output()?;
        
        if output.status.success() {
            println!("    ✅ Success");
            
            // Check if it has Cargo.toml
            let cargo_check = Command::new("find")
                .args(&[&format!("../{}", submodule_path), "-name", "Cargo.toml"])
                .output()?;
            
            if cargo_check.status.success() {
                let cargo_files = String::from_utf8_lossy(&cargo_check.stdout);
                if !cargo_files.trim().is_empty() {
                    println!("    📦 Found Cargo.toml files");
                } else {
                    println!("    ⚠️ No Cargo.toml found");
                }
            }
        } else {
            println!("    ❌ Failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    
    // Final scan
    println!("\n📋 Final scan for Cargo.toml files...");
    let output = Command::new("find")
        .args(&["../submodules", "-name", "Cargo.toml", "-type", "f"])
        .output()?;
    
    let cargo_files = String::from_utf8_lossy(&output.stdout);
    let count = cargo_files.lines().count();
    
    println!("  ✅ Found {} Cargo.toml files total", count);
    
    // Extract crate names from found Cargo.toml files
    let mut found_crates = Vec::new();
    for toml_path in cargo_files.lines() {
        if let Ok(content) = fs::read_to_string(toml_path) {
            for line in content.lines() {
                if line.trim().starts_with("name = ") {
                    if let Some(name) = line.split('"').nth(1) {
                        found_crates.push(name.to_string());
                        break;
                    }
                }
            }
        }
    }
    
    println!("\n🎯 MATCHING RESULTS:");
    for target in &target_crates {
        if found_crates.contains(&target.to_string()) {
            println!("  ✅ {} - EXACT MATCH", target);
        } else {
            // Check for similar names
            let mut found_similar = false;
            for found in &found_crates {
                if names_are_similar(target, found) {
                    println!("  🔍 {} - SIMILAR: {}", target, found);
                    found_similar = true;
                    break;
                }
            }
            if !found_similar {
                println!("  ❌ {} - NOT FOUND", target);
            }
        }
    }
    
    let exact_matches = target_crates.iter()
        .filter(|&t| found_crates.contains(&t.to_string()))
        .count();
    
    let coverage = (exact_matches as f64 / target_crates.len() as f64) * 100.0;
    
    println!("\n📊 COVERAGE SUMMARY:");
    println!("  Target crates: {}", target_crates.len());
    println!("  Exact matches: {}", exact_matches);
    println!("  Coverage: {:.1}%", coverage);
    
    Ok(())
}

fn extract_submodule_name(line: &str) -> String {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            let full_path = &line[start + 1..start + 1 + end];
            return full_path.split('/').last().unwrap_or(full_path).to_string();
        }
    }
    "unknown".to_string()
}

fn names_are_similar(target: &str, candidate: &str) -> bool {
    let target_lower = target.to_lowercase().replace("-", "_");
    let candidate_lower = candidate.to_lowercase().replace("-", "_");
    
    target_lower == candidate_lower ||
    target_lower.contains(&candidate_lower) ||
    candidate_lower.contains(&target_lower) ||
    candidate_lower.starts_with(&target_lower) ||
    candidate_lower.ends_with(&target_lower)
}
