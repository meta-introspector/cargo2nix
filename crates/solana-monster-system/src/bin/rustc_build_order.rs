use std::process::Command;

#[derive(Debug)]
struct CrateBuildOrder {
    criticality: u8,
    git_hash: String,
    name: String,
    branch: String,
    repo: String,
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RUSTC BUILD ORDER (Real Data) ===");

    let mut build_order = get_real_build_order()?;
    build_order.sort_by(|a, b| b.criticality.cmp(&a.criticality));

    for (i, crate_info) in build_order.iter().enumerate() {
        println!(
            "{}. [{}] {} | {} | {} | {} | {}",
            i + 1,
            crate_info.criticality,
            crate_info.git_hash,
            crate_info.name,
            crate_info.branch,
            crate_info.repo,
            crate_info.path
        );
    }

    Ok(())
}

fn get_real_build_order() -> Result<Vec<CrateBuildOrder>, Box<dyn std::error::Error>> {
    let mut crates = Vec::new();

    // Get submodule data
    let output = Command::new("git")
        .args(&["submodule", "status"])
        .output()?;

    let submodule_data = String::from_utf8_lossy(&output.stdout);

    for line in submodule_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hash = parts[0].trim_start_matches(&['+', '-', ' '][..]);
            let path = parts[1];
            let name = path.split('/').last().unwrap_or("unknown");

            // Get branch info
            let branch = get_branch_for_path(path)?;
            let repo = get_repo_for_path(path)?;

            let criticality = match name {
                n if n == "rust" => 9,
                n if n.contains("rustc") => 8,
                n if n.contains("rust") => 7,
                n if n.contains("cargo") => 6,
                n if n.contains("solana") => 5,
                _ => 1,
            };

            crates.push(CrateBuildOrder {
                criticality,
                git_hash: hash[..8].to_string(),
                name: name.to_string(),
                branch,
                repo,
                path: path.to_string(),
            });
        }
    }

    Ok(crates)
}

fn get_branch_for_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(path)
        .output();

    match output {
        Ok(result) => {
            let branch = String::from_utf8_lossy(&result.stdout).trim().to_string();
            Ok(if branch.is_empty() {
                "detached".to_string()
            } else {
                branch
            })
        }
        Err(_) => Ok("no-git".to_string()),
    }
}

fn get_repo_for_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .current_dir(path)
        .output();

    match output {
        Ok(result) => {
            let url = String::from_utf8_lossy(&result.stdout).trim().to_string();
            // Extract repo name from URL
            if let Some(repo_name) = url.split('/').last() {
                Ok(repo_name.trim_end_matches(".git").to_string())
            } else {
                Ok("unknown".to_string())
            }
        }
        Err(_) => Ok("no-remote".to_string()),
    }
}
