use std::os::unix::process::ExitStatusExt; // Added for from_raw
#[cfg(feature = "with-anyhow")]
use anyhow::Result;
#[cfg(not(feature = "with-anyhow"))]
use std::error::Error; // For fallback Result
#[cfg(not(feature = "with-anyhow"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>; // Fallback for Result
use std::path::Path;
use std::process::Output;

use crate::git_traits::GitRepositoryOperations;
use crate::git_types::SubmoduleInfo;

pub struct MockGitRepositoryOperations;

impl GitRepositoryOperations for MockGitRepositoryOperations {
    fn git_add_all(&self, _repo_path: &Path) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git add all stdout".to_vec(),
            stderr: b"mock git add all stderr".to_vec(),
        })
    }

    fn git_commit(&self, _repo_path: &Path, _message: &str) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git commit stdout".to_vec(),
            stderr: b"mock git commit stderr".to_vec(),
        })
    }

    fn git_push(&self, _repo_path: &Path, _remote: &str, _branch: &str) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git push stdout".to_vec(),
            stderr: b"mock git push stderr".to_vec(),
        })
    }

    fn git_status(&self, _repo_path: &Path) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git status stdout".to_vec(),
            stderr: b"mock git status stderr".to_vec(),
        })
    }

    fn git_submodule_add(&self, _repo_path: &Path, _url: &str, _path: &str, _name: Option<&str>, _branch: Option<&str>) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git submodule add stdout".to_vec(),
            stderr: b"mock git submodule add stderr".to_vec(),
        })
    }

    fn git_submodule_update(&self, _repo_path: &Path, _init: bool, _recursive: bool) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git submodule update stdout".to_vec(),
            stderr: b"mock git submodule update stderr".to_vec(),
        })
    }

    fn git_submodule_status(&self, _repo_path: &Path) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git submodule status stdout".to_vec(),
            stderr: b"mock git submodule status stderr".to_vec(),
        })
    }

    fn git_submodule_remove(&self, _repo_path: &Path, _path: &str) -> Result<Output> {
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"mock git submodule remove stdout".to_vec(),
            stderr: b"mock git submodule remove stderr".to_vec(),
        })
    }

    fn submodules(&self, _repo_path: &Path) -> Result<Vec<SubmoduleInfo>> {
        Ok(vec![])
    }
}
