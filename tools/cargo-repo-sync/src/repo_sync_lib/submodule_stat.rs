use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SubmoduleStat {
    pub head_commit: String,
    pub workdir_hash: String, // A hash representing the state of the working directory (e.g., from git status --porcelain)
}
