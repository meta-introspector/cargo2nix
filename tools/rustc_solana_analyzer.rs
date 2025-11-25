use std::fs;
use std::process::Command;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut solana_crates = HashSet::new();
    let mut existing_submodules = HashSet::new();
    let mut missing_crates = Vec::new();
    
    // Find Solana rustc and extract its dependencies
    find_solana_rustc_deps(&mut solana_crates)?;
    
    // Scan existing submodules
    scan_existing_submodules(&mut existing_submodules)?;
    
    // Compare and find missing
    for crate_name in &solana_crates {
        if !existing_submodules.contains(crate_name) {
            missing_crates.push(crate_name.clone());
        }
    }
    
    // Generate report
    generate_analysis_report(&solana_crates, &existing_submodules, &missing_crates)?;
    
    Ok(())
}

fn find_solana_rustc_deps(crates: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Look for Solana-related Cargo.lock files
    let output = Command::new("find")
        .args(&[".", "-path", "*/solana*", "-name", "Cargo.lock"])
        .output()?;
    
    let solana_locks = String::from_utf8_lossy(&output.stdout);
    
    for lock_path in solana_locks.lines() {
        if let Ok(content) = fs::read_to_string(lock_path) {
            parse_cargo_lock_crates(&content, crates);
            println!("Processed: {}", lock_path);
        }
    }
    
    // Also check for rustc-specific dependencies
    let rustc_output = Command::new("find")
        .args(&[".", "-path", "*/rust*", "-name", "Cargo.lock"])
        .output()?;
    
    let rustc_locks = String::from_utf8_lossy(&rustc_output.stdout);
    
    for lock_path in rustc_locks.lines() {
        if lock_path.contains("rustc") || lock_path.contains("compiler") {
            if let Ok(content) = fs::read_to_string(lock_path) {
                parse_cargo_lock_crates(&content, crates);
                println!("Processed rustc: {}", lock_path);
            }
        }
    }
    
    println!("Found {} unique crates from Solana/rustc", crates.len());
    Ok(())
}

fn parse_cargo_lock_crates(content: &str, crates: &mut HashSet<String>) {
    let mut in_package = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line == "[[package]]" {
            in_package = true;
        } else if in_package && line.starts_with("name = \"") {
            let name = &line[8..line.len()-1];
            crates.insert(name.to_string());
            in_package = false;
        } else if line.is_empty() {
            in_package = false;
        }
    }
}

fn scan_existing_submodules(submodules: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(entries) = fs::read_dir("submodules") {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type()?.is_dir() && !name.starts_with('.') {
                    submodules.insert(name);
                }
            }
        }
    }
    
    println!("Found {} existing submodules", submodules.len());
    Ok(())
}

fn generate_analysis_report(
    solana_crates: &HashSet<String>,
    existing_submodules: &HashSet<String>,
    missing_crates: &[String]
) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("# Rustc Solana Dependency Analysis\n\n");
    
    report.push_str(&format!("## Summary\n"));
    report.push_str(&format!("- **Solana/Rustc crates found**: {}\n", solana_crates.len()));
    report.push_str(&format!("- **Existing submodules**: {}\n", existing_submodules.len()));
    report.push_str(&format!("- **Missing crates**: {}\n\n", missing_crates.len()));
    
    // Coverage analysis
    let coverage = if solana_crates.is_empty() { 0.0 } else {
        ((solana_crates.len() - missing_crates.len()) as f64 / solana_crates.len() as f64) * 100.0
    };
    report.push_str(&format!("**Coverage**: {:.1}%\n\n", coverage));
    
    // Missing crates
    let mut sorted_missing = missing_crates.clone();
    sorted_missing.sort();
    
    if !missing_crates.is_empty() {
        report.push_str("## Missing Crates (Need to Add)\n");
        
        for crate_name in &sorted_missing {
            let github_url = guess_github_url(crate_name);
            report.push_str(&format!("- **{}** → `{}`\n", crate_name, github_url));
        }
        report.push_str("\n");
    }
    
    // Available crates
    let mut available: Vec<_> = solana_crates.iter()
        .filter(|c| existing_submodules.contains(*c))
        .collect();
    available.sort();
    
    if !available.is_empty() {
        report.push_str("## Available Crates (Already in Submodules)\n");
        for crate_name in &available {
            report.push_str(&format!("- **{}** ✓\n", crate_name));
        }
        report.push_str("\n");
    }
    
    // Generate .gitmodules entries for missing crates
    if !missing_crates.is_empty() {
        report.push_str("## Suggested .gitmodules Entries\n```\n");
        for crate_name in &sorted_missing {
            let github_url = guess_github_url(crate_name);
            report.push_str(&format!(
                "[submodule \"{}\"]\n\tpath = submodules/{}\n\turl = {}\n\n",
                crate_name, crate_name, github_url
            ));
        }
        report.push_str("```\n\n");
    }
    
    // Generate pull commands
    if !missing_crates.is_empty() {
        report.push_str("## Pull Commands\n```bash\n");
        for crate_name in &sorted_missing {
            let github_url = guess_github_url(crate_name);
            report.push_str(&format!(
                "git submodule add {} submodules/{}\n",
                github_url, crate_name
            ));
        }
        report.push_str("```\n");
    }
    
    fs::write("rustc_solana_analysis.md", &report)?;
    
    // Also create a simple missing list
    let missing_list = sorted_missing.join("\n");
    fs::write("missing_crates.txt", &missing_list)?;
    
    println!("Analysis complete:");
    println!("  Coverage: {:.1}%", coverage);
    println!("  Missing: {} crates", missing_crates.len());
    println!("  Reports saved to rustc_solana_analysis.md and missing_crates.txt");
    
    Ok(())
}

fn guess_github_url(crate_name: &str) -> String {
    match crate_name {
        // Rust core crates
        n if n.starts_with("rustc") => format!("https://github.com/rust-lang/rust.git"),
        n if n.starts_with("std") => format!("https://github.com/rust-lang/rust.git"),
        
        // Solana ecosystem
        n if n.starts_with("solana") => format!("https://github.com/solana-labs/{}.git", n),
        n if n.starts_with("spl") => format!("https://github.com/solana-labs/solana-program-library.git"),
        
        // Common patterns
        "serde" => "https://github.com/serde-rs/serde.git".to_string(),
        "tokio" => "https://github.com/tokio-rs/tokio.git".to_string(),
        "clap" => "https://github.com/clap-rs/clap.git".to_string(),
        "regex" => "https://github.com/rust-lang/regex.git".to_string(),
        
        // Default pattern
        _ => format!("https://github.com/{}/{}.git", crate_name, crate_name),
    }
}
