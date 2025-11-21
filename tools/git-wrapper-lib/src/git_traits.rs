use anyhow::Result;
use std::path::{Path, PathBuf};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::process::Output;
use std::ffi::OsStr;

use crate::git_types::{RollupLock, SubmoduleStat, SubmoduleInfo};

// --- GitExecutor Trait ---
pub trait GitExecutor: Send + Sync {
    fn submodule_add(&self, repo_url: &str, submodule_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()>;
    fn checkout_branch(&self, submodule_path: &Path, branch: &str, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()>;
    fn status(&self, submodule_path: &Path) -> Result<String>; // For submodule status
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>>;
    fn clone(&self, repo_url: &str, target_path: &Path) -> Result<()>;
    fn get_file_git_info(&self, repo_path: &Path, file_path: &Path) -> Result<(bool, Option<String>)>;
    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat>;
    fn as_any(&self) -> &dyn Any;
}

pub trait GitRepositoryOperations {
    fn git_add_all(&self, repo_path: &Path) -> Result<Output>;
    fn git_commit(&self, repo_path: &Path, message: &str) -> Result<Output>;
    fn git_push(&self, repo_path: &Path, remote: &str, branch: &str) -> Result<Output>;
    fn git_status(&self, repo_path: &Path) -> Result<Output>;
    fn git_submodule_add(&self, repo_path: &Path, url: &str, path: &str, name: Option<&str>, branch: Option<&str>) -> Result<Output>;
    fn git_submodule_update(&self, repo_path: &Path, init: bool, recursive: bool) -> Result<Output>;
    fn git_submodule_status(&self, repo_path: &Path) -> Result<Output>;
    fn git_submodule_remove(&self, repo_path: &Path, path: &str) -> Result<Output>;
    fn submodules(&self, repo_path: &Path) -> Result<Vec<SubmoduleInfo>>;
}

// --- GhExecutor Trait ---
pub trait GhExecutor {
    fn repo_fork(&self, repo_url: &str, target_org: &str) -> Result<()>;
    fn repo_view(&self, forked_repo_url: &str) -> Result<bool>; // Returns true if repo exists
}

// --- The Execv Trait ---
pub trait Execv: Send + Sync {
    fn execv(&self, program: &OsStr, args: &[&OsStr], current_dir: Option<&Path>) -> Result<Output>;
}