// cargo-submodule-tool-lib/src/analysis/cargo_toml_processor.rs
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tool_traits_lib::{CargoTomlProcessor, RegexMatcher, RepoState};

// Import the CurrentRegexMatcher for use in the RealCargoTomlProcessor
use super::regex_matcher::CurrentRegexMatcher;

#[cfg(feature = "toml_edit_enabled")]
use toml_edit::{Document, Item};

#[cfg(feature = "toml_edit_enabled")]
pub struct RealCargoTomlProcessor;

#[cfg(feature = "toml_edit_enabled")]
impl RealCargoTomlProcessor {
    // Helper functions from PureRustRepoDiscoverer
    fn clean_repo_url(&self, repo_url: &str) -> String {
        let regex_matcher = CurrentRegexMatcher::new(
            r"^(https?://github\.com/[^/]+/[^/.]+)(/tree/[^/]+/.+)?(\.git)?$",
        )
        .expect("Failed to create regex matcher for clean_repo_url");
        if let Some(captures) = regex_matcher.captures(repo_url) {
            let base_url = captures
                .get(1)
                .expect("Expected capture group 1")
                .to_string();
            let git_suffix = captures.get(3).unwrap_or("").to_string();
            format!("{}{}", base_url, git_suffix)
        } else {
            repo_url.to_string()
        }
    }

    fn extract_owner_repo_name(&self, cleaned_url: &str) -> Option<(String, String)> {
        let regex_matcher = CurrentRegexMatcher::new(r"github\.com/([^/]+)/([^/.]+)(\.git)?")
            .expect("Failed to create regex matcher for extract_owner_repo_name");
        if let Some(captures) = regex_matcher.captures(cleaned_url) {
            let owner = captures
                .get(1)
                .expect("Expected capture group 1")
                .to_string();
            let repo_name = captures
                .get(2)
                .expect("Expected capture group 2")
                .to_string();
            Some((owner, repo_name))
        } else {
            None
        }
    }
}

#[cfg(feature = "toml_edit_enabled")]
impl CargoTomlProcessor for RealCargoTomlProcessor {
    fn process_cargo_toml(
        &self,
        path: &Path,
        discovered_repos: &mut HashSet<RepoState>,
        all_vendored_crate_names: &mut HashSet<String>,
        target_org: &str,
        target_branch: &str,
    ) -> std::result::Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read Cargo.toml at {:?}: {}", path, e))?;

        let doc = content
            .parse::<Document<String>>()
            .map_err(|e| format!("Failed to parse Cargo.toml at {:?}: {}", path, e))?;

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
        if let Some(Item::Value(toml_edit::Value::String(s))) = doc
            .get("package")
            .and_then(|item| item.as_table())
            .and_then(|table| table.get("repository"))
        {
            let repo_url = s.value().to_string();
            if repo_url.contains("github.com") {
                let cleaned_url = self.clean_repo_url(&repo_url);
                if let Some((owner, repo_name)) = self.extract_owner_repo_name(&cleaned_url) {
                    discovered_repos.insert(RepoState {
                        // Use RepoState
                        repo_url: cleaned_url,
                        owner,
                        repo_name,
                        target_org: target_org.to_string(),
                        target_branch: target_branch.to_string(),
                        ..Default::default() // Fill other fields with default values
                    });
                } else {
                    eprintln!(
                        "WARNING: Could not extract owner or repository name from: {}. Skipping.",
                        cleaned_url
                    );
                }
            }
        }

        // Handle workspace members
        if let Some(workspace_table) = doc.get("workspace").and_then(|item| item.as_table()) {
            if let Some(members_array) = workspace_table
                .get("members")
                .and_then(|item| item.as_array())
            {
                for member_item in members_array.iter() {
                    if let Some(member_path_str) = member_item.as_str() {
                        let member_path = path
                            .parent()
                            .unwrap()
                            .join(member_path_str)
                            .join("Cargo.toml");
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
                                        all_vendored_crate_names
                                            .insert(member_package_name.to_string());
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
                                let dep_cargo_toml_path =
                                    path.parent().unwrap().join(path_str).join("Cargo.toml");
                                if dep_cargo_toml_path.exists() {
                                    if let Ok(dep_content) =
                                        fs::read_to_string(&dep_cargo_toml_path)
                                    {
                                        if let Ok(dep_doc) = dep_content.parse::<Document<String>>()
                                        {
                                            if let Some(dep_package_name) = dep_doc
                                                .get("package")
                                                .and_then(|item| item.as_table())
                                                .and_then(|table| table.get("name"))
                                                .and_then(|item| item.as_str())
                                            {
                                                all_vendored_crate_names
                                                    .insert(dep_package_name.to_string());
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

#[cfg(not(feature = "toml_edit_enabled"))]
pub struct DummyCargoTomlProcessor;

#[cfg(not(feature = "toml_edit_enabled"))]
impl CargoTomlProcessor for DummyCargoTomlProcessor {
    fn process_cargo_toml(
        &self,
        _path: &Path,
        _discovered_repos: &mut HashSet<RepoState>,
        _all_vendored_crate_names: &mut HashSet<String>,
        _target_org: &str,
        _target_branch: &str,
    ) -> std::result::Result<(), String> {
        Err("toml_edit_enabled feature is required for CargoTomlProcessor.".to_string())
    }
}

#[cfg(feature = "toml_edit_enabled")]
pub type CurrentCargoTomlProcessor = RealCargoTomlProcessor;
#[cfg(not(feature = "toml_edit_enabled"))]
pub type CurrentCargoTomlProcessor = DummyCargoTomlProcessor;
