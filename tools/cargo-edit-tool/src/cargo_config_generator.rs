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

use anyhow::{Context, Result};
use cargo_metadata::Metadata;
use git_wrapper_lib::git_adapters::GitAdapter;
use std::path::{Path, PathBuf};
use cargo_edit_lib::{WorkspaceInfo, WorkspaceInfoProvider, CargoMetadataProvider as CargoMetadataProviderTrait}; // Added

pub struct CargoConfigGeneratorImpl; // New struct

impl WorkspaceInfoProvider for CargoConfigGeneratorImpl {
    fn parse_members_file(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProviderTrait, // Use the trait from cargo_edit_lib
        project_root: &Path,
    ) -> Result<Vec<WorkspaceInfo>> {
        let members_file_path = project_root.join("submodules/members.txt");
        let members_file_content = std::fs::read_to_string(&members_file_path)
            .context(format!("Failed to read members file at {:?}", members_file_path))?;

        let mut workspace_infos = Vec::new();

        for line in members_file_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                eprintln!("Warning: Malformed line in members.txt: {}", line);
                continue;
            }

            let submodule_path_str = parts[0];
            let member_crates_str = parts[1];

            let submodule_base_path_rel = PathBuf::from(submodule_path_str);
            let member_crates: Vec<String> = member_crates_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            workspace_infos.push(WorkspaceInfo {
                member_crates,
                submodule_base_path_rel,
            });
        }

        Ok(workspace_infos)
    }
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
