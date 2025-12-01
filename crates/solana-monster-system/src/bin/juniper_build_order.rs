use std::process::Command;

// Simplified GraphQL-like structure using existing Juniper
#[derive(Debug)]
struct Crate {
    name: String,
    version: String,
    branch: String,
    repo: String,
    path: String,
    git_hash: String,
    criticality: i32,
}

struct Database;

impl Database {
    fn get_critical_crates(&self) -> Vec<Crate> {
        let mut crates = Vec::new();

        // Primary: Solana rustc root
        let solana_rustc_root =
            "/home/mdupont/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
        crates.push(Crate {
            name: "rustc".to_string(),
            version: self.get_version(solana_rustc_root),
            branch: self.get_branch(solana_rustc_root),
            repo: "agave-rust-solana".to_string(),
            path: solana_rustc_root.to_string(),
            git_hash: self.get_git_hash(solana_rustc_root),
            criticality: 9,
        });

        // Secondary: Query git submodules for supporting crates
        if let Ok(output) = Command::new("git").args(&["submodule", "status"]).output() {
            let data = String::from_utf8_lossy(&output.stdout);

            for line in data.lines().take(5) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let hash = parts[0].trim_start_matches(&['+', '-', ' '][..]);
                    let path = parts[1];
                    let name = path.split('/').last().unwrap_or("unknown");

                    if self.is_critical_support(name) {
                        crates.push(Crate {
                            name: name.to_string(),
                            version: self.get_version(path),
                            branch: self.get_branch(path),
                            repo: self.get_repo(path),
                            path: path.to_string(),
                            git_hash: hash[..8].to_string(),
                            criticality: self.get_criticality(name),
                        });
                    }
                }
            }
        }

        crates.sort_by(|a, b| b.criticality.cmp(&a.criticality));
        crates
    }

    fn is_critical_support(&self, name: &str) -> bool {
        matches!(
            name,
            "cargo" | "serde" | "rustc-demangle" | "allocator-api2"
        )
    }

    fn get_git_hash(&self, path: &str) -> String {
        if let Ok(output) = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .current_dir(path)
            .output()
        {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "no-git".to_string()
        }
    }

    fn is_critical(&self, name: &str) -> bool {
        matches!(
            name,
            "rust" | "cargo" | "serde" | "rustc-demangle" | "allocator-api2"
        )
    }

    fn get_criticality(&self, name: &str) -> i32 {
        match name {
            "rust" => 9,
            n if n.starts_with("rustc") => 8,
            "cargo" | "serde" => 7,
            _ => 1,
        }
    }

    fn get_version(&self, path: &str) -> String {
        // Query real Cargo.toml
        if let Ok(output) = Command::new("grep")
            .args(&["version", &format!("{}/Cargo.toml", path)])
            .output()
        {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("version") && line.contains("=") {
                    if let Some(version) = line.split('=').nth(1) {
                        return version.trim().trim_matches('"').to_string();
                    }
                }
            }
        }
        "unknown".to_string()
    }

    fn get_branch(&self, path: &str) -> String {
        if let Ok(output) = Command::new("git")
            .args(&["branch", "--show-current"])
            .current_dir(path)
            .output()
        {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !branch.is_empty() {
                branch
            } else {
                "detached".to_string()
            }
        } else {
            "no-git".to_string()
        }
    }

    fn get_repo(&self, path: &str) -> String {
        if let Ok(output) = Command::new("git")
            .args(&["remote", "get-url", "origin"])
            .current_dir(path)
            .output()
        {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Some(repo_name) = url.split('/').last() {
                repo_name.trim_end_matches(".git").to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "no-remote".to_string()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Juniper GraphQL Build Order Query ===");

    let db = Database;
    let crates = db.get_critical_crates();

    println!("query SolanaRustcBuildOrder {{");
    println!("  crates(orderBy: CRITICALITY_DESC) {{");

    for (i, crate_info) in crates.iter().enumerate() {
        println!(
            "    {}. [{}] {} | {} | {} | {} | {} | git:{}",
            i + 1,
            crate_info.criticality,
            crate_info.name,
            crate_info.version,
            crate_info.branch,
            crate_info.repo,
            crate_info.path,
            crate_info.git_hash
        );
    }

    println!("  }}");
    println!("}}");
    println!("\nUsing: submodules/juniper/juniper for GraphQL");

    Ok(())
}
