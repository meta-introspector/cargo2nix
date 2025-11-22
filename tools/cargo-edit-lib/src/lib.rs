use anyhow::{anyhow, Context, Result};
#[cfg(feature = "real_cargo_metadata")]
use cargo_metadata::Metadata;
use git_wrapper_lib::git_adapters::GitAdapter;
#[cfg(feature = "serde")]
#[cfg(feature = "serde_json_enabled")]
use serde_json; // Added for Metadata default construction
use std::collections::HashMap; // Added
use std::path::{Path, PathBuf}; // Added
use std::fs; // Added
#[cfg(feature = "regex_enabled")]
use regex::Regex; // Added
#[cfg(feature = "lazy_static_enabled")]
use lazy_static::lazy_static; // Added
#[cfg(feature = "toml_edit_enabled")]
use toml_edit::{Document, DocumentMut, Item, Table, Value}; // Added
#[cfg(feature = "walkdir_enabled")]
use walkdir::WalkDir; // Added
#[cfg(feature = "serde")]
#[cfg(feature = "serde_enabled")]
use serde::{Serialize, Deserialize}; // Added

pub trait AnyMetadata: Send + Sync {
    // Add methods here to access metadata fields if needed by the core logic
    // For now, it can be a marker trait or have minimal methods.
}

// Dummy Metadata struct when real_cargo_metadata is not enabled
#[cfg(not(feature = "real_cargo_metadata"))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DummyMetadata;

#[cfg(not(feature = "real_cargo_metadata"))]
impl AnyMetadata for DummyMetadata {}

// Implement AnyMetadata for real cargo_metadata::Metadata
#[cfg(feature = "real_cargo_metadata")]
impl AnyMetadata for cargo_metadata::Metadata {}

pub trait CargoMetadataProvider: Send + Sync {
    fn get_metadata(&self, cargo_toml_path: &std::path::Path) -> Result<Box<dyn AnyMetadata>>;
}

#[cfg(feature = "real_cargo_metadata")]
pub struct RealCargoMetadataProvider;

#[cfg(feature = "real_cargo_metadata")]
impl CargoMetadataProvider for RealCargoMetadataProvider {
    fn get_metadata(&self, cargo_toml_path: &std::path::Path) -> Result<Box<dyn AnyMetadata>> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(cargo_toml_path)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to get cargo metadata: {}", e))?;
        Ok(Box::new(metadata))
    }
}



pub struct MockCargoMetadataProvider;

impl CargoMetadataProvider for MockCargoMetadataProvider {
    fn get_metadata(&self, _cargo_toml_path: &std::path::Path) -> Result<Box<dyn AnyMetadata>> {
        // Mock implementation, return dummy metadata or an error as needed for tests
        Ok(Box::new(DummyMetadata::default()))
    }
}



pub trait CargoEditAdapter: Send + Sync {
    fn generate_cargo_config(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
    ) -> Result<String, anyhow::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceInfo {
    pub member_crates: Vec<String>,
    pub submodule_base_path_rel: std::path::PathBuf,
}

pub trait WorkspaceInfoProvider: Send + Sync {
    fn parse_members_file(
        &self,
        git_adapter: &dyn git_wrapper_lib::git_adapters::GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
        project_root: &std::path::Path,
    ) -> anyhow::Result<Vec<WorkspaceInfo>>;
}

/// Represents a single patch entry for a crate.
#[cfg_attr(feature = "serde_enabled", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PatchEntry {
    pub crate_name: String,
    pub path: PathBuf,
}

/// Represents the generated patches, grouped by their repository URL.
/// The key is the repository URL (e.g., "https://github.com/meta-introspector/time-rs").
/// The value is a vector of `PatchEntry` for that repository.
pub type GeneratedPatches = HashMap<String, Vec<PatchEntry>>;

pub struct CargoConfigGeneratorImpl;

impl WorkspaceInfoProvider for CargoConfigGeneratorImpl {
    fn parse_members_file(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
        project_root: &Path,
    ) -> Result<Vec<WorkspaceInfo>, anyhow::Error> {
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
pub fn generate_patch_entries(project_root: &Path, workspace_infos: &[WorkspaceInfo]) -> GeneratedPatches {
    let mut generated_patches = HashMap::new();

    for info in workspace_infos {
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

#[cfg(feature = "toml_edit_enabled")]
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
pub struct CargoEditAdapterImpl;

impl CargoEditAdapter for CargoEditAdapterImpl {
    fn generate_cargo_config(
        &self,
        git_adapter: &dyn GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
    ) -> Result<String, anyhow::Error> {
        // TODO: Implement the actual logic for generating cargo config
        // This will involve using git_adapter to get submodule info
        // and cargo_metadata_provider to get cargo metadata.
        // For now, return a dummy string.
        Ok("[cargo]\nbuild-std = [\"core\", \"alloc\"]\n".to_string())
    }
}
