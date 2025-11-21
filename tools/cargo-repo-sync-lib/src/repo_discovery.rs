use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use toml_edit::{Document, Item};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoInfo {
    pub repo_url: String,
    pub owner: String,
    pub repo_name: String,
    pub target_org: String,
    pub target_branch: String,
}

pub trait RepoDiscoverer {
    fn discover_repos(&self, root_dir: &Path) -> Result<(Vec<RepoInfo>, HashSet<String>)>;
}

pub struct PureRustRepoDiscoverer {
    target_org: String,
    target_branch: String,
}

impl PureRustRepoDiscoverer {
    pub fn new(target_org: String, target_branch: String) -> Self {
        PureRustRepoDiscoverer {
            target_org,
            target_branch,
        }
    }

    fn clean_repo_url(&self, repo_url: &str) -> String {
        let re = Regex::new(r"^(https?://github\.com/[^/]+/[^/.]+)(/tree/[^/]+/.+)?(\.git)?$").unwrap();
        if let Some(captures) = re.captures(repo_url) {
            let base_url = captures.get(1).unwrap().as_str();
            let git_suffix = captures.get(3).map_or("", |m| m.as_str());
            format!("{}{}", base_url, git_suffix)
        } else {
            repo_url.to_string()
        }
    }

    fn extract_owner_repo_name(&self, cleaned_url: &str) -> Option<(String, String)> {
        let re = Regex::new(r"github\.com/([^/]+)/([^/.]+)(\.git)?").unwrap();
        if let Some(captures) = re.captures(cleaned_url) {
            let owner = captures.get(1).unwrap().as_str().to_string();
            let repo_name = captures.get(2).unwrap().as_str().to_string();
            Some((owner, repo_name))
        } else {
            None
        }
    }

    fn process_cargo_toml(
        &self,
        path: &Path,
        discovered_repos: &mut HashSet<RepoInfo>,
        all_vendored_crate_names: &mut HashSet<String>,
    ) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read Cargo.toml at {:?}", path))?;

        let doc = content
            .parse::<Document<String>>()
            .with_context(|| format!("Failed to parse Cargo.toml at {:?}", path))?;

        // Extract package name
        if let Some(package_name) = doc
            .get("package")
            .and_then(|item| item.as_table())
            .and_then(|table| table.get("name"))
            .and_then(|item| item.as_str())
        {
            all_vendored_crate_names.insert(package_name.to_string());
        }

        // Extract repository URL for top-level repos
        if let Some(Item::Value(toml_edit::Value::String(s))) = doc.get("package").and_then(|item| item.as_table()).and_then(|table| table.get("repository")) {
            let repo_url = s.value().to_string();
            if repo_url.contains("github.com") {
                let cleaned_url = self.clean_repo_url(&repo_url);
                if let Some((owner, repo_name)) = self.extract_owner_repo_name(&cleaned_url) {
                    discovered_repos.insert(RepoInfo {
                        repo_url: cleaned_url,
                        owner,
                        repo_name,
                        target_org: self.target_org.clone(),
                        target_branch: self.target_branch.clone(),
                    });
                } else {
                    eprintln!("WARNING: Could not extract owner or repository name from: {}. Skipping.", cleaned_url);
                }
            }
        }

        // Handle workspace members
        if let Some(workspace_table) = doc.get("workspace").and_then(|item| item.as_table()) {
            if let Some(members_array) = workspace_table.get("members").and_then(|item| item.as_array()) {
                for member_item in members_array.iter() {
                    if let Some(member_path_str) = member_item.as_str() {
                        let member_path = path.parent().unwrap().join(member_path_str).join("Cargo.toml");
                        if member_path.exists() {
                            // Only extract package name, do not recurse fully to avoid infinite loops
                            if let Ok(member_content) = fs::read_to_string(&member_path) {
                                if let Ok(member_doc) = member_content.parse::<Document<String>>() {
                                    if let Some(member_package_name) = member_doc
                                        .get("package")
                                        .and_then(|item| item.as_table())
                                        .and_then(|table| table.get("name"))
                                        .and_then(|item| item.as_str())
                                    {
                                        all_vendored_crate_names.insert(member_package_name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Handle path dependencies within this Cargo.toml
        let sections = ["dependencies", "dev-dependencies", "build-dependencies"];
        for section_name in sections {
            if let Some(deps_table) = doc.get(section_name).and_then(|item| item.as_table()) {
                for (_key, item) in deps_table.iter() {
                    if let Some(dep_table) = item.as_table() {
                        if let Some(path_item) = dep_table.get("path") {
                            if let Some(path_str) = path_item.as_str() {
                                let dep_cargo_toml_path = path.parent().unwrap().join(path_str).join("Cargo.toml");
                                if dep_cargo_toml_path.exists() {
                                    if let Ok(dep_content) = fs::read_to_string(&dep_cargo_toml_path) {
                                        if let Ok(dep_doc) = dep_content.parse::<Document<String>>() {
                                            if let Some(dep_package_name) = dep_doc
                                                .get("package")
                                                .and_then(|item| item.as_table())
                                                .and_then(|table| table.get("name"))
                                                .and_then(|item| item.as_str())
                                            {
                                                all_vendored_crate_names.insert(dep_package_name.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl RepoDiscoverer for PureRustRepoDiscoverer {
    fn discover_repos(&self, root_dir: &Path) -> Result<(Vec<RepoInfo>, HashSet<String>)> {
        let mut discovered_repos: HashSet<RepoInfo> = HashSet::new();
        let mut all_vendored_crate_names: HashSet<String> = HashSet::new();

        for entry in WalkDir::new(root_dir)
            .into_iter()
            .filter_entry(|e| {
                let path = e.path();
                !(path.ends_with("target")
                    || path.ends_with("tests")
                    || path.ends_with("examples")
                    || path.ends_with("submodules/target")
                    || path.ends_with("submodules/tests")
                    || path.ends_with("submodules/examples"))
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "Cargo.toml" {
                self.process_cargo_toml(entry.path(), &mut discovered_repos, &mut all_vendored_crate_names)?;
            }
        }

        Ok((discovered_repos.into_iter().collect(), all_vendored_crate_names))
    }
}