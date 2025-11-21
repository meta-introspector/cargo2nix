use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::process::{Command, Output, ExitStatus};
use std::os::unix::process::ExitStatusExt; // Needed for ExitStatus::from_raw

use crate::git_traits::{GitExecutor, GitRepositoryOperations};
use crate::git_types::SubmoduleInfo;

pub struct RealGitRepositoryOperations {
    git_executor: Arc<dyn GitExecutor + Send + Sync>,
}

impl RealGitRepositoryOperations {
    pub fn new(git_executor: Arc<dyn GitExecutor + Send + Sync>) -> Self {
        RealGitRepositoryOperations { git_executor }
    }
}

impl GitRepositoryOperations for RealGitRepositoryOperations {
    fn git_add_all(&self, repo_path: &Path) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("add")
            .arg(".")
            .output()
            .context("Failed to execute git add .")
    }

    fn git_commit(&self, repo_path: &Path, message: &str) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .context("Failed to execute git commit")
    }

    fn git_push(&self, repo_path: &Path, remote: &str, branch: &str) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("push")
            .arg(remote)
            .arg(branch)
            .output()
            .context("Failed to execute git push")
    }

    fn git_status(&self, repo_path: &Path) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("status")
            .output()
            .context("Failed to execute git status")
    }

    fn git_submodule_add(&self, repo_path: &Path, url: &str, path: &str, name: Option<&str>, branch: Option<&str>) -> Result<Output> {
        let mut command = Command::new("git");
        command.arg("-C").arg(repo_path).arg("submodule").arg("add");
        if let Some(name) = name {
            command.arg("--name").arg(name);
        }
        if let Some(branch) = branch {
            command.arg("--branch").arg(branch);
        }
        command.arg(url).arg(path).output().context("Failed to execute git submodule add")
    }

    fn git_submodule_update(&self, repo_path: &Path, init: bool, recursive: bool) -> Result<Output> {
        let mut command = Command::new("git");
        command.arg("-C").arg(repo_path).arg("submodule").arg("update");
        if init {
            command.arg("--init");
        }
        if recursive {
            command.arg("--recursive");
        }
        command.output().context("Failed to execute git submodule update")
    }

    fn git_submodule_status(&self, repo_path: &Path) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("submodule")
            .arg("status")
            .output()
            .context("Failed to execute git submodule status")
    }

    fn git_submodule_remove(&self, repo_path: &Path, path: &str) -> Result<Output> {
        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("submodule")
            .arg("deinit")
            .arg(path)
            .output()
            .context("Failed to deinitialize submodule")?;

        Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("rm")
            .arg(path)
            .output()
            .context("Failed to remove submodule from git index")?;

        // Remove the .git/modules/<path> directory
        let git_modules_path = repo_path.join(".git/modules").join(path);
        if git_modules_path.exists() {
            std::fs::remove_dir_all(&git_modules_path)
                .context(format!("Failed to remove .git/modules/{} directory", path))?;
        }

        Ok(Output {
            status: ExitStatus::from_raw(0), // Success
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    }

    fn submodules(&self, repo_path: &Path) -> Result<Vec<SubmoduleInfo>> {
        let submodules_list = self.git_executor.list_submodules(repo_path)?;
        let mut submodules_info = Vec::new();
        for (url, path) in submodules_list {
            let name = path.file_name()
                           .and_then(|os_str| os_str.to_str())
                           .unwrap_or_default()
                           .to_string();
            submodules_info.push(SubmoduleInfo { name, path, url: Some(url) });
        }
        Ok(submodules_info)
    }
}
