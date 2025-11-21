use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use toml_edit::{Document, DocumentMut, Item, Table, Value};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize}; // Added

use git_wrapper_lib::git_adapters::GitAdapter; // Updated
#[cfg(feature = "nix_generation")]
use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};

/// Represents a single patch entry for a crate.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct PatchEntry {
    pub crate_name: String,
    pub path: PathBuf,
}

/// Represents the generated patches, grouped by their repository URL.
/// The key is the repository URL (e.g., "https://github.com/meta-introspector/time-rs").
/// The value is a vector of `PatchEntry` for that repository.
pub type GeneratedPatches = HashMap<String, Vec<PatchEntry>>;

/// Represents information parsed from the members.txt file for a single submodule.
#[derive(Debug, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct SubmoduleWorkspaceInfo {
    pub submodule_base_path_rel: PathBuf,
    pub member_crates: Vec<String>,
}

/// Represents all workspace information parsed from the members.txt file.
pub type WorkspaceInfo = Vec<SubmoduleWorkspaceInfo>;

/// Parses .gitmodules to get a list of (relative_submodule_path, members_list) tuples.
pub fn parse_members_file(
    git_adapter: &dyn GitAdapter, // Changed from git_executor
    cargo_metadata_provider: &dyn CargoMetadataProvider,
    project_root: &Path
) -> anyhow::Result<WorkspaceInfo> {
    let mut workspace_info = Vec::new();

    for (url, submodule_path_rel) in git_adapter.list_submodules(project_root)? { // Changed from git_executor
        let submodule_abs_path = project_root.join(&submodule_path_rel);

        let mut member_crates = Vec::new();

        // Check if the submodule itself is a Rust package or workspace
        let submodule_cargo_toml = submodule_abs_path.join("Cargo.toml");
        if submodule_cargo_toml.exists() {
            let metadata = cargo_metadata_provider.provide_metadata(&submodule_cargo_toml)?;
            
            if metadata.workspace_root == submodule_cargo_toml.parent().unwrap() {
                // It's a workspace, add all its members
                for member_id in &metadata.workspace_members {
                    if let Some(pkg) = metadata.packages.iter().find(|p| &p.id == member_id) {
                        member_crates.push(pkg.name.to_string());
                    }
                }
            } else {
                // It's a single package within the submodule root
                if let Some(pkg) = metadata.packages.first() {
                    member_crates.push(pkg.name.to_string());
                }
            }
        }

        // Also search for Cargo.toml files in subdirectories of the submodule
        for entry in WalkDir::new(&submodule_abs_path)
            .min_depth(1) // Start searching from subdirectories
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
        {
            let sub_cargo_toml_path = entry.path();
            // Avoid re-processing the root Cargo.toml if already handled
            if sub_cargo_toml_path == submodule_cargo_toml {
                continue;
            }

            let metadata = cargo_metadata_provider.provide_metadata(sub_cargo_toml_path)?;
            
            if let Some(pkg) = metadata.packages.first() {
                member_crates.push(pkg.name.to_string());
            }
        }

        // Deduplicate member crates
        member_crates.sort_unstable();
        member_crates.dedup();

        if !member_crates.is_empty() {
            workspace_info.push(SubmoduleWorkspaceInfo {
                submodule_base_path_rel: submodule_path_rel,
                member_crates,
            });
        }
    }
    Ok(workspace_info)
}

/// Generates [patch] entries for .cargo/config.toml for each workspace member.
pub fn generate_patch_entries(project_root: &Path, workspace_info: &WorkspaceInfo) -> GeneratedPatches {
    let mut generated_patches = HashMap::new();

    for info in workspace_info {
        let submodule_name = info.submodule_base_path_rel
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        
        let patch_section_header = format!("https://github.com/meta-introspector/{}", submodule_name);
        let mut entries = Vec::new();

        for member_name in &info.member_crates {
            let member_abs_path = project_root
                .join(&info.submodule_base_path_rel)
                .join(member_name);
            
            entries.push(PatchEntry {
                crate_name: member_name.clone(),
                path: member_abs_path,
            });
        }
        generated_patches.insert(patch_section_header, entries);
    }
    generated_patches
}

/// Reads existing .cargo/config.toml, updates patch sections, and writes back.
pub fn update_config_toml(config_toml_path: &Path, new_patches: &GeneratedPatches) -> anyhow::Result<()> {
    let mut doc = if config_toml_path.exists() {
        let contents = fs::read_to_string(config_toml_path)
            .context(format!("Failed to read existing config.toml: {:?}", config_toml_path))?;
        contents.parse::<DocumentMut>()
            .context(format!("Failed to parse existing config.toml: {:?}", config_toml_path))?
    } else {
        println!("Warning: {:?} not found. Creating a new one.", config_toml_path);
        DocumentMut::new()
    };

    let patches_table = doc
        .entry("patch")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .context("Expected 'patch' to be a table")?;

    for (repo_url, entries) in new_patches {
        let repo_table = patches_table
            .entry(repo_url)
            .or_insert(Item::Table(Table::new()))
            .as_table_mut()
            .context(format!("Expected patch section for {} to be a table", repo_url))?;

        for entry in entries {
            let mut crate_table = Table::new();
            crate_table.insert(
                "path",
                Item::Value(Value::String(toml_edit::Formatted::new(entry.path.to_str().context("Invalid path")?.to_string()))),
            );
            repo_table.insert(&entry.crate_name, Item::Table(crate_table));
        }
    }

    fs::write(config_toml_path, doc.to_string())
        .context(format!("Failed to write updated config.toml: {:?}", config_toml_path))?;

    println!("Updated {:?}", config_toml_path);
    Ok(())
}
