use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, Default, PartialEq, Eq, Clone)
)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub branch: Option<String>,
    pub commit_id: String,
}

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, Default, PartialEq, Eq, Clone)
)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<DependencyInfo>,
}

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, Default, PartialEq, Eq, Clone)
)]
pub struct DependencyInfo {
    pub name: String,
    pub source: String,
    pub req: String,
}

#[cfg_attr(
    feature = "serde",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, Default, PartialEq, Eq, Clone)
)]
pub struct CargoWorkspaceInfo {
    pub manifest_path: PathBuf,
    pub packages: Vec<PackageInfo>,
}

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(not(feature = "serde"), derive(Debug, Default, PartialEq, Eq, Clone))]
pub struct NixFlakeInfo {
    pub flake_path: PathBuf,
    pub inputs: HashMap<String, String>,
    pub outputs: Vec<String>,
}

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(
        Debug,
        Default,
        PartialEq,
        Eq,
        Clone,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(not(feature = "serde"), derive(Debug, Default, PartialEq, Eq, Clone))]
pub struct RepoState {
    pub submodules: Vec<SubmoduleInfo>,
    pub cargo_workspaces: Vec<CargoWorkspaceInfo>,
    pub nix_flakes: Vec<NixFlakeInfo>,
}

#[cfg_attr(
    feature = "serde",
    derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, PartialEq, Eq, Clone)
)]
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

#[cfg_attr(
    feature = "serde_json_enabled",
    derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    not(feature = "serde_json_enabled"),
    derive(Debug, PartialEq, Eq, Clone)
)]
pub struct MergedCrateInfo {
    pub layer: i32,
    pub usage_count: u32,
}
