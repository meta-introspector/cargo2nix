use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Submodule database for organizing git repositories
pub struct SubmoduleDB {
    pub repos: HashMap<String, RepoInfo>,
    pub categories: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name: String,
    pub url: String,
    pub category: String,
    pub priority: u8, // 1=highest, 5=lowest
    pub has_cargo_toml: bool,
    pub submodule_path: String,
}

impl SubmoduleDB {
    pub fn new() -> Self {
        Self {
            repos: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// Initialize with existing submodules from directory scan
    pub fn scan_existing_submodules(
        &mut self,
        submodules_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(submodules_dir);
        if !path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            if entry.file_type()?.is_dir() && !name.starts_with('.') {
                let submodule_path = format!("{}/{}", submodules_dir, name);
                let has_cargo = Path::new(&format!("{}/Cargo.toml", submodule_path)).exists();

                let repo_info = RepoInfo {
                    name: name.clone(),
                    url: self.guess_repo_url(&name),
                    category: self.categorize_repo(&name),
                    priority: self.calculate_priority(&name),
                    has_cargo_toml: has_cargo,
                    submodule_path,
                };

                self.add_repo(repo_info);
            }
        }

        Ok(())
    }

    /// Add repository to database
    pub fn add_repo(&mut self, repo: RepoInfo) {
        let category = repo.category.clone();
        let name = repo.name.clone();
        self.repos.insert(name.clone(), repo);

        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Get repositories by category
    pub fn get_by_category(&self, category: &str) -> Vec<&RepoInfo> {
        self.categories
            .get(category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.repos.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get high priority repositories
    pub fn get_high_priority(&self) -> Vec<&RepoInfo> {
        self.repos
            .values()
            .filter(|repo| repo.priority <= 2)
            .collect()
    }

    /// Initialize with known important repositories
    pub fn initialize_core_repos(&mut self) {
        let core_repos = vec![
            RepoInfo {
                name: "rust".to_string(),
                url: "https://github.com/rust-lang/rust.git".to_string(),
                category: "compiler".to_string(),
                priority: 1,
                has_cargo_toml: true,
                submodule_path: "submodules/rust".to_string(),
            },
            RepoInfo {
                name: "cargo".to_string(),
                url: "https://github.com/rust-lang/cargo.git".to_string(),
                category: "build-tools".to_string(),
                priority: 1,
                has_cargo_toml: true,
                submodule_path: "submodules/cargo".to_string(),
            },
            RepoInfo {
                name: "serde".to_string(),
                url: "https://github.com/serde-rs/serde.git".to_string(),
                category: "serialization".to_string(),
                priority: 2,
                has_cargo_toml: true,
                submodule_path: "submodules/serde".to_string(),
            },
            RepoInfo {
                name: "tokio".to_string(),
                url: "https://github.com/tokio-rs/tokio.git".to_string(),
                category: "async".to_string(),
                priority: 2,
                has_cargo_toml: true,
                submodule_path: "submodules/tokio".to_string(),
            },
        ];

        for repo in core_repos {
            self.add_repo(repo);
        }
    }

    fn guess_repo_url(&self, name: &str) -> String {
        // Try to guess GitHub URL from common patterns
        match name {
            n if n.starts_with("rust-") => format!("https://github.com/rust-lang/{}.git", n),
            n if n.ends_with("-rs") => {
                let base = &n[..n.len() - 3];
                format!("https://github.com/{}-rs/{}.git", base, base)
            }
            _ => format!("https://github.com/unknown/{}.git", name),
        }
    }

    fn categorize_repo(&self, name: &str) -> String {
        match name {
            "rust" | "rustc" | "rustfmt" | "clippy" => "compiler".to_string(),
            "cargo" | "cargo2nix" => "build-tools".to_string(),
            "serde" | "serde-rs" | "bincode" | "toml" => "serialization".to_string(),
            "tokio" | "async-std" | "futures-rs" => "async".to_string(),
            "clap" | "structopt" => "cli".to_string(),
            n if n.contains("crypto") || n.contains("hash") => "crypto".to_string(),
            n if n.contains("http") || n.contains("hyper") || n.contains("reqwest") => {
                "networking".to_string()
            }
            _ => "utility".to_string(),
        }
    }

    fn calculate_priority(&self, name: &str) -> u8 {
        match name {
            "rust" | "cargo" => 1,
            "serde" | "tokio" | "clap" => 2,
            n if n.starts_with("rust-") => 2,
            _ => 3,
        }
    }

    /// Export database as JSON for external tools
    pub fn export_json(&self) -> String {
        // Simple JSON export
        let mut json = String::from("{\n");
        json.push_str("  \"repos\": {\n");

        for (i, (name, repo)) in self.repos.iter().enumerate() {
            if i > 0 {
                json.push_str(",\n");
            }
            json.push_str(&format!(
                "    \"{}\": {{\n      \"url\": \"{}\",\n      \"category\": \"{}\",\n      \"priority\": {},\n      \"has_cargo\": {}\n    }}",
                name, repo.url, repo.category, repo.priority, repo.has_cargo_toml
            ));
        }

        json.push_str("\n  }\n}");
        json
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = SubmoduleDB::new();

    // Initialize with core repositories
    db.initialize_core_repos();

    // Scan existing submodules
    db.scan_existing_submodules("submodules")?;

    println!("Submodule Database Initialized:");
    println!("Total repositories: {}", db.repos.len());

    for category in db.categories.keys() {
        let repos = db.get_by_category(category);
        println!("  {}: {} repos", category, repos.len());
    }

    println!("\nHigh Priority Repositories:");
    for repo in db.get_high_priority() {
        println!("  {} (priority {}): {}", repo.name, repo.priority, repo.url);
    }

    // Export to JSON
    let json = db.export_json();
    fs::write("submodule_database.json", json)?;
    println!("\nDatabase exported to submodule_database.json");

    Ok(())
}
