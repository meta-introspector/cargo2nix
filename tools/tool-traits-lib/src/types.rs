use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub branch: Option<String>,
    pub commit_id: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct DependencyInfo {
    pub name: String,
    pub source: String,
    pub req: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct CargoWorkspaceInfo {
    pub manifest_path: PathBuf,
    pub packages: Vec<PackageInfo>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct NixFlakeInfo {
    pub flake_path: PathBuf,
    pub inputs: BTreeMap<String, String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct RepoState {
    pub repo_url: String, // Added
    pub owner: String, // Added
    pub repo_name: String, // Added
    pub target_org: String, // Added
    pub target_branch: String, // Added
    pub submodules: Vec<SubmoduleInfo>,
    pub cargo_workspaces: Vec<CargoWorkspaceInfo>,
    pub nix_flakes: Vec<NixFlakeInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub len: u64,
    pub hash: String,
    pub git_object_hash: Option<String>,
    pub is_git_tracked: bool,
}

impl Default for FileMetadata {
    fn default() -> Self {
        FileMetadata {
            modified: UNIX_EPOCH,
            len: 0,
            hash: String::new(),
            git_object_hash: None,
            is_git_tracked: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde_enabled", derive(serde::Serialize, serde::Deserialize))]
pub struct MergedCrateInfo {
    pub layer: i32,
    pub usage_count: u32,
}
