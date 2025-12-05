use anyhow::{anyhow, Result};
use std::collections::HashMap; // For RepoState
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Output; // For DummyExecv
use std::sync::Arc;
use std::time::SystemTime; // For FileMetadata

// --- GitExecutor Trait and Implementations ---
#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::git_traits::GitExecutor;
#[cfg(not(feature = "git_enabled"))]
pub trait GitExecutor: Send + Sync {
    fn clone(&self, _repo_url: &str, _target_path: &Path) -> Result<()> {
        Err(anyhow!("Dummy GitExecutor: clone not implemented"))
    }
    fn checkout(&self, _path: &Path, _branch: &str) -> Result<()> {
        Err(anyhow!("Dummy GitExecutor: checkout not implemented"))
    }
    fn add_submodule(
        &self,
        _repo_url: &str,
        _path: &Path,
        _name: Option<&str>,
        _branch: Option<&str>,
    ) -> Result<()> {
        Err(anyhow!("Dummy GitExecutor: add_submodule not implemented"))
    }
    fn submodule_update(&self, _path: &Path) -> Result<()> {
        Err(anyhow!(
            "Dummy GitExecutor: submodule_update not implemented"
        ))
    }
    fn current_commit_hash(&self, _path: &Path) -> Result<String> {
        Ok("dummy_commit_hash".to_string())
    }
    fn current_branch(&self, _path: &Path) -> Result<String> {
        Ok("dummy_branch".to_string())
    }
    fn fetch_submodules(&self, _path: &Path) -> Result<()> {
        Err(anyhow!(
            "Dummy GitExecutor: fetch_submodules not implemented"
        ))
    }
    fn get_submodule_url(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_url".to_string())
    }
    fn set_submodule_url(&self, _submodule_path: &Path, _new_url: &str) -> Result<()> {
        Err(anyhow!(
            "Dummy GitExecutor: set_submodule_url not implemented"
        ))
    }
    fn get_submodule_branch(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_branch".to_string())
    }
    fn set_submodule_branch(&self, _submodule_path: &Path, _new_branch: &str) -> Result<()> {
        Err(anyhow!(
            "Dummy GitExecutor: set_submodule_branch not implemented"
        ))
    }
    fn get_submodule_path(&self, _submodule_name: &str) -> Result<PathBuf> {
        Ok(PathBuf::from("dummy_path"))
    }
    fn list_submodules(&self, _path: &Path) -> Result<Vec<(String, PathBuf)>> {
        Ok(vec![])
    }
    fn get_submodule_head_commit(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_head_commit".to_string())
    }
    fn get_submodule_remote_url(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_remote_url".to_string())
    }
    fn get_submodule_remote_branch(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_remote_branch".to_string())
    }
    fn get_submodule_status(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_status".to_string())
    }
    fn get_submodule_config_path(&self, _submodule_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("dummy_config_path"))
    }
    fn get_submodule_git_dir(&self, _submodule_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("dummy_git_dir"))
    }
    fn get_submodule_work_tree(&self, _submodule_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("dummy_work_tree"))
    }
    fn get_submodule_index_path(&self, _submodule_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("dummy_index_path"))
    }
    fn get_submodule_head_oid(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_head_oid".to_string())
    }
    fn get_submodule_branch_oid(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_branch_oid".to_string())
    }
    fn get_submodule_remote_oid(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_remote_oid".to_string())
    }
    fn get_submodule_diff(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_diff".to_string())
    }
    fn get_submodule_log(&self, _submodule_path: &Path) -> Result<String> {
        Ok("dummy_log".to_string())
    }
    fn get_submodule_remotes(&self, _submodule_path: &Path) -> Result<Vec<String>> {
        Ok(vec![])
    }
    fn get_submodule_remote_head(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
    ) -> Result<String> {
        Ok("dummy_remote_head".to_string())
    }
    fn get_submodule_remote_branches(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
    ) -> Result<Vec<String>> {
        Ok(vec![])
    }
    fn get_submodule_remote_tags(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
    ) -> Result<Vec<String>> {
        Ok(vec![])
    }
    fn get_submodule_remote_refs(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
    ) -> Result<Vec<String>> {
        Ok(vec![])
    }
    fn get_submodule_remote_ref_oid(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_oid".to_string())
    }
    fn get_submodule_remote_ref_log(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_log".to_string())
    }
    fn get_submodule_remote_ref_diff(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_diff".to_string())
    }
    fn get_submodule_remote_ref_commit(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit".to_string())
    }
    fn get_submodule_remote_ref_commit_oid(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_oid".to_string())
    }
    fn get_submodule_remote_ref_commit_message(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_message".to_string())
    }
    fn get_submodule_remote_ref_commit_author(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_author".to_string())
    }
    fn get_submodule_remote_ref_commit_committer(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_committer".to_string())
    }
    fn get_submodule_remote_ref_commit_time(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_time".to_string())
    }
    fn get_submodule_remote_ref_commit_tree(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_oid(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_oid".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry_oid(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry_oid".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry_type(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry_type".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry_filemode(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry_filemode".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry_size(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry_size".to_string())
    }
    fn get_submodule_remote_ref_commit_tree_entry_content(
        &self,
        _submodule_path: &Path,
        _remote_name: &str,
        _ref_name: &str,
        _entry_path: &str,
    ) -> Result<String> {
        Ok("dummy_ref_commit_tree_entry_content".to_string())
    }
    fn get_file_git_info(&self, _root_dir: &Path, _path: &Path) -> Result<(bool, Option<String>)> {
        Ok((false, None))
    }
}

#[cfg(not(feature = "git_enabled"))]
pub struct DummyGitExecutor;

#[cfg(not(feature = "git_enabled"))]
impl GitExecutor for DummyGitExecutor {}

#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::pure_rust_git_executor::PureRustGitExecutor;
#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::system_git_executor::SystemGitExecutor;
#[cfg(not(feature = "git_enabled"))]
pub type SystemGitExecutor = DummyGitExecutor; // Alias dummy when git is not enabled
#[cfg(not(feature = "git_enabled"))]
pub type PureRustGitExecutor = DummyGitExecutor; // Alias dummy when git is not enabled

// --- Execv Trait and Implementations ---
#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::execv::RealExecv;
#[cfg(not(feature = "git_enabled"))]
pub trait Execv: Send + Sync {
    fn execv(
        &self,
        _program: &OsStr,
        _args: &[&OsStr],
        _current_dir: Option<&Path>,
    ) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0), // Success
            stdout: b"dummy stdout".to_vec(),
            stderr: b"dummy stderr".to_vec(),
        })
    }
}

#[cfg(not(feature = "git_enabled"))]
pub struct DummyExecv;

#[cfg(not(feature = "git_enabled"))]
impl Execv for DummyExecv {}
#[cfg(not(feature = "git_enabled"))]
pub type RealExecv = DummyExecv; // Alias dummy when git is not enabled

// --- RollupLock Struct ---
#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::git_types::RollupLock;
#[cfg(not(feature = "git_enabled"))]
pub struct DummyRollupLock;

#[cfg(not(feature = "git_enabled"))]
impl DummyRollupLock {
    pub fn load(_root_dir: &Path) -> Result<Self> {
        Ok(DummyRollupLock)
    }
    pub fn get_submodule_commit(&self, _submodule_path: &Path) -> Option<String> {
        Some("dummy_commit".to_string())
    }
    pub fn set_submodule_commit(&mut self, _submodule_path: &Path, _commit: String) {}
    pub fn save(&self, _root_dir: &Path) -> Result<()> {
        Ok(())
    }
}
#[cfg(not(feature = "git_enabled"))]
pub type RollupLock = DummyRollupLock; // Alias dummy when git is not enabled

// --- RepoStateCollector Trait and Implementations ---
#[cfg(feature = "git_enabled")]
pub use git_wrapper_lib::repo_state_collector::{RealRepoStateCollector, RepoStateCollector};
#[cfg(not(feature = "git_enabled"))]
pub trait RepoStateCollector: Send + Sync {
    fn collect_repo_state(&self, project_root: &Path) -> Result<RepoState> {
        println!(
            "Dummy RepoStateCollector: collect_repo_state for {:?}",
            project_root
        );
        Ok(RepoState::default())
    }
}

#[cfg(not(feature = "git_enabled"))]
pub struct DummyRealRepoStateCollector; // Renamed to avoid conflict with RealRepoStateCollector

#[cfg(not(feature = "git_enabled"))]
impl DummyRealRepoStateCollector {
    pub fn new(
        _git_executor: Arc<dyn GitExecutor + Send + Sync>,
        _cargo_metadata_provider: Box<
            dyn crate::analysis::cargo_metadata_provider::CargoMetadataProvider + Send + Sync,
        >,
    ) -> Self {
        DummyRealRepoStateCollector {}
    }
}

#[cfg(not(feature = "git_enabled"))]
impl RepoStateCollector for DummyRealRepoStateCollector {}
#[cfg(not(feature = "git_enabled"))]
pub type RealRepoStateCollector = DummyRealRepoStateCollector;

// --- Data Structures (now re-exported from tool-traits-lib) ---
pub use tool_traits_lib::{
    CargoWorkspaceInfo, DependencyInfo, FileMetadata, NixFlakeInfo, PackageInfo, RepoState,
    SubmoduleInfo,
};
