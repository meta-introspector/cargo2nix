use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoAction {
    pub repo_url: String,
    pub owner: String,
    pub repo_name: String,
    pub submodule_path: PathBuf,
    pub target_org: String,
    pub target_branch: String,
}