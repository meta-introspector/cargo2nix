use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SOLANA RUSTC BUILD ORDER (Real Data Only) ===");

    // Read RUST_SRC_PATH from Makefile
    let makefile_content = fs::read_to_string("tools/Makefile")?;
    let rust_src_path = extract_rust_src_path(&makefile_content)?;

    // Get real git data from Solana rustc
    let solana_hash = get_git_hash(&rust_src_path)?;
    let solana_branch = get_git_branch(&rust_src_path)?;

    // Get real submodule data
    let submodule_output = Command::new("git")
        .args(&["submodule", "status"])
        .output()?;
    let submodule_data = String::from_utf8_lossy(&submodule_output.stdout);

    println!(
        "1. [9] solana-rustc | real | {} | agave | {}/compiler | git:{}",
        solana_branch, rust_src_path, solana_hash
    );

    // Parse real submodule data
    let mut order = 2;
    for line in submodule_data.lines().take(5) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hash = parts[0].trim_start_matches(&['+', '-', ' '][..]);
            let path = parts[1];
            let name = path.split('/').last().unwrap_or("unknown");

            if matches!(
                name,
                "rustc-build-sysroot" | "rustc-demangle" | "cargo" | "serde" | "allocator-api2"
            ) {
                let branch = get_git_branch(path).unwrap_or_else(|_| "unknown".to_string());
                println!(
                    "{}. [8] {} | real | {} | real | {} | git:{}",
                    order,
                    name,
                    branch,
                    path,
                    &hash[..8]
                );
                order += 1;
            }
        }
    }

    Ok(())
}

fn extract_rust_src_path(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    for line in content.lines() {
        if line.starts_with("RUST_SRC_PATH") {
            if let Some(path) = line.split("=").nth(1) {
                return Ok(path.trim().to_string());
            }
        }
    }
    Err("RUST_SRC_PATH not found".into())
}

fn get_git_hash(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .current_dir(path)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_git_branch(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(path)
        .output()?;
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(if branch.is_empty() {
        "detached".to_string()
    } else {
        branch
    })
}
