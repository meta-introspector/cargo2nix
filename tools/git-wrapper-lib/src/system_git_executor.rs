#[cfg(feature = "anyhow_enabled")]
use anyhow::{Context, Result};
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error; // For fallback Result
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>; // Fallback for Result
#[cfg(not(feature = "anyhow_enabled"))]
trait Context<T> {
    // Fallback for anyhow::Context
    fn context<C>(self, _context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}
#[cfg(not(feature = "anyhow_enabled"))]
impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, _context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

use std::any::Any;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::git_adapters::GitAdapter;
use crate::git_traits::Execv;
use crate::git_traits::GitExecutor;
use crate::git_types::{RollupLock, SubmoduleStat}; // Added

#[cfg(feature = "hex_enabled")]
use hex;
#[cfg(feature = "sha1_enabled")]
use sha1::{Digest, Sha1};

pub struct SystemGitExecutor {
    git_executable_path: PathBuf,
    executor: Arc<dyn Execv + Send + Sync>,
    _rollup_lock: Arc<Mutex<RollupLock>>,
    _root_dir: PathBuf,
}

impl SystemGitExecutor {
    pub fn new(
        git_executable_path: PathBuf,
        executor: Arc<dyn Execv + Send + Sync>,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: PathBuf,
    ) -> Self {
        SystemGitExecutor {
            git_executable_path,
            executor,
            _rollup_lock: rollup_lock,
            _root_dir: root_dir,
        }
    }
}

impl GitExecutor for SystemGitExecutor {
    fn submodule_add(
        &self,
        repo_url: &str,
        submodule_path: &Path,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: &Path,
    ) -> Result<()> {
        #[cfg(feature = "with-trace")]
        println!("TRACE: submodule_add called with repo_url: {}, submodule_path: {:?}, rollup_lock: {:?}, root_dir: {:?}", repo_url, submodule_path, rollup_lock, root_dir);
        println!(
            "Executing git submodule add {} ?{:?}",
            repo_url, submodule_path
        );
        let program = self.git_executable_path.as_os_str();
        let args = &[
            OsStr::new("submodule"),
            OsStr::new("add"),
            OsStr::new(repo_url),
            submodule_path.as_os_str(),
        ];
        let output = self.executor.execv(program, args, None)?;

        if !output.status.success() {
            eprintln!(
                "Failed to add submodule {}: {}",
                repo_url,
                String::from_utf8_lossy(&output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Adding submodule failed for {}", repo_url),
            )));
        }
        println!("Successfully added {} as submodule.", repo_url);
        // create_snapshot(root_dir, rollup_lock, Arc::new(self.clone()))?; // Commented out
        Ok(())
    }

    fn checkout_branch(
        &self,
        submodule_path: &Path,
        branch: &str,
        _rollup_lock: Arc<Mutex<RollupLock>>, // Prefixed with _
        _root_dir: &Path, // Prefixed with _
    ) -> Result<()> {
        #[cfg(feature = "with-trace")]
        println!("TRACE: checkout_branch called with submodule_path: {:?}, branch: {}, rollup_lock: {:?}, root_dir: {:?}", submodule_path, branch, rollup_lock, root_dir);
        println!(
            "Executing git -C ?{:?} checkout ?{}",
            submodule_path, branch
        );
        let program = self.git_executable_path.as_os_str();
        let args = &[OsStr::new("checkout"), OsStr::new(branch)];
        let output = self.executor.execv(program, args, Some(submodule_path))?;

        if !output.status.success() {
            eprintln!(
                "Failed to checkout branch '{}' in submodule ?{:?}: {}",
                branch,
                submodule_path,
                String::from_utf8_lossy(&output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!("Branch checkout failed for ?{:?}", submodule_path);
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Branch checkout failed for ?{:?}", submodule_path),
            )));
        }
        println!(
            "Successfully checked out branch '{}' in submodule ?{:?}.",
            branch, submodule_path
        );
        // create_snapshot(root_dir, rollup_lock, Arc::new(self.clone()))?; // Commented out
        Ok(())
    }

    fn status(&self, submodule_path: &Path) -> Result<String> {
        println!("Executing git -C ?{:?} status", submodule_path);
        let program = self.git_executable_path.as_os_str();
        let args = &[OsStr::new("status")];
        let output = self.executor.execv(program, args, Some(submodule_path))?;

        if !output.status.success() {
            eprintln!(
                "Failed to get status for submodule ?{:?}: {}",
                submodule_path,
                String::from_utf8_lossy(&output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!("Failed to get status for ?{:?}", submodule_path);
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get status for ?{:?}", submodule_path),
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        println!(
            "Executing git submodule status --recursive in ?{:?}",
            root_dir
        );
        let program = self.git_executable_path.as_os_str();
        let args = &[
            OsStr::new("submodule"),
            OsStr::new("status"),
            OsStr::new("--recursive"),
        ];
        let output = self.executor.execv(program, args, Some(root_dir))?;

        if !output.status.success() {
            eprintln!(
                "Failed to list submodules in ?{:?}: {}",
                root_dir,
                String::from_utf8_lossy(&output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!("Failed to list submodules in ?{:?}", root_dir);
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to list submodules in ?{:?}", root_dir),
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut submodules = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let path_str = parts[1];
                let submodule_path = PathBuf::from(path_str);

                let config_url_arg = format!("submodule.{}.url", path_str); // Fix: create a let binding
                let config_args = &[
                    OsStr::new("config"),
                    OsStr::new("--file"),
                    OsStr::new(".gitmodules"),
                    OsStr::new(&config_url_arg), // Use the let binding here
                ];
                let config_output = self.executor.execv(program, config_args, Some(root_dir))?;

                if config_output.status.success() {
                    let url = String::from_utf8_lossy(&config_output.stdout)
                        .trim()
                        .to_string();
                    if !url.is_empty() {
                        submodules.push((url, submodule_path));
                    }
                } else {
                    eprintln!(
                        "Warning: Could not get URL for submodule {}. Stderr: {}",
                        path_str,
                        String::from_utf8_lossy(&config_output.stderr)
                    );
                }
            }
        }
        Ok(submodules)
    }

    fn clone(&self, repo_url: &str, target_path: &Path) -> Result<()> {
        println!("Executing git clone {} ?{:?}", repo_url, target_path);
        let program = self.git_executable_path.as_os_str();
        let args = &[
            OsStr::new("clone"),
            OsStr::new(repo_url),
            target_path.as_os_str(),
        ];
        let output = self.executor.execv(program, args, None)?;

        if !output.status.success() {
            eprintln!(
                "Failed to clone {}: {}",
                repo_url,
                String::from_utf8_lossy(&output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!("Cloning failed for {}", repo_url);
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Cloning failed for {}", repo_url),
            )));
        }
        println!(
            "Successfully cloned {} to ?{:?} using pure Rust.",
            repo_url, target_path
        );
        Ok(())
    }

    #[cfg(feature = "sha1_enabled")]
    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        let program = self.git_executable_path.as_os_str();

        let head_commit_args = &[
            OsStr::new("-C"),
            path.as_os_str(),
            OsStr::new("rev-parse"),
            OsStr::new("HEAD"),
        ];
        let head_commit_output = self.executor.execv(program, head_commit_args, None)?;
        if !head_commit_output.status.success() {
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!(
                "Failed to get HEAD commit for ?{:?}: ?{}",
                path,
                String::from_utf8_lossy(&head_commit_output.stderr)
            );
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to get HEAD commit for ?{:?}: ?{}",
                    path,
                    String::from_utf8_lossy(&head_commit_output.stderr)
                ),
            )));
        }
        let head_commit = String::from_utf8_lossy(&head_commit_output.stdout)
            .trim()
            .to_string();

        let workdir_status_args = &[
            OsStr::new("-C"),
            path.as_os_str(),
            OsStr::new("status"),
            OsStr::new("--porcelain"),
        ];
        let workdir_status_output = self.executor.execv(program, workdir_status_args, None)?;
        if !workdir_status_output.status.success() {
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!(
                "Failed to get workdir status for ?{:?}: ?{}",
                path,
                String::from_utf8_lossy(&workdir_status_output.stderr)
            );
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to get workdir status for ?{:?}: ?{}",
                    path,
                    String::from_utf8_lossy(&workdir_status_output.stderr)
                ),
            )));
        }
        let mut hasher = Sha1::new();
        hasher.update(&workdir_status_output.stdout);
        #[cfg(feature = "hex_enabled")]
        let workdir_hash = hex::encode(hasher.finalize());
        #[cfg(not(feature = "hex_enabled"))]
        let workdir_hash = format!("{:?}", hasher.finalize());

        Ok(SubmoduleStat {
            head_commit,
            workdir_hash,
        })
    }

    #[cfg(not(feature = "sha1_enabled"))]
    fn get_submodule_head_and_workdir_hash(&self, _path: &Path) -> Result<SubmoduleStat> {
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!("SystemGitExecutor::get_submodule_head_and_workdir_hash requires the 'sha1' feature, which is not enabled.");
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "SystemGitExecutor::get_submodule_head_and_workdir_hash requires the 'sha1' feature, which is not enabled.")));
    }

    fn get_file_git_info(
        &self,
        repo_path: &Path,
        file_path: &Path,
    ) -> Result<(bool, Option<String>)> {
        let program = self.git_executable_path.as_os_str();

        let args = &[
            OsStr::new("-C"),
            repo_path.as_os_str(),
            OsStr::new("ls-files"),
            OsStr::new("-s"),
            file_path.as_os_str(),
        ];
        let output = self.executor.execv(program, args, None)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let is_git_tracked = !stdout.trim().is_empty();
        let mut git_object_hash = None;

        if is_git_tracked {
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    git_object_hash = Some(parts[1].to_string());
                }
            }
        }

        Ok((is_git_tracked, git_object_hash))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GitAdapter for SystemGitExecutor {
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        GitExecutor::list_submodules(self, root_dir)
    }

    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        GitExecutor::get_submodule_head_and_workdir_hash(self, path)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
