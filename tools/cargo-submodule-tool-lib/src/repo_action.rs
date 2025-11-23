use std::path::PathBuf;
#[cfg(feature = "tool_traits_lib_enabled")]
use tool_traits_lib::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter};

#[cfg_attr(feature = "serde_enabled", derive(Debug, Serialize, Deserialize))]
pub struct RepoAction {
    pub repo_url: String,
    pub owner: String,
    pub repo_name: String,
    pub submodule_path: PathBuf,
    pub target_org: String,
    pub target_branch: String,
}
