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
use std::sync::Arc; // Added

use crate::git_traits::Execv;
use crate::git_types::SubmoduleStat; // SubmoduleInfo is not used

#[cfg(feature = "git2")]
use git2::Repository; // Submodule is not used

/// A unified trait for Git operations, abstracting different execution modes.
pub trait GitAdapter: Send + Sync {
    /// Lists submodules in the given root directory.
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>>;

    /// Gets the head and workdir hash for a submodule.
    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat>;

    /// Returns a reference to `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Mock implementation of `GitAdapter` for dry-run or testing.
pub struct MockGitAdapter {
    // You might want to store predefined mock data here
    pub mock_submodules: Vec<(String, PathBuf)>,
    pub mock_submodule_stat: SubmoduleStat,
}

impl MockGitAdapter {
    pub fn new() -> Self {
        MockGitAdapter {
            mock_submodules: vec![],
            mock_submodule_stat: SubmoduleStat {
                head_commit: "mock_head".to_string(),
                workdir_hash: "mock_workdir".to_string(),
            },
        }
    }
}

impl GitAdapter for MockGitAdapter {
    fn list_submodules(&self, _root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        println!("[MockGitAdapter] Listing submodules (mock data)");
        Ok(self.mock_submodules.clone())
    }

    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        println!("[MockGitAdapter] Getting submodule stat for {:?}", path);
        Ok(self.mock_submodule_stat.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Shell implementation of `GitAdapter` that uses external `git` commands.
pub struct ShellGitAdapter {
    execv: Arc<dyn Execv + Send + Sync>, // Changed from Box to Arc
}

impl ShellGitAdapter {
    pub fn new(execv: Arc<dyn Execv + Send + Sync>) -> Self {
        // Changed from Box to Arc
        ShellGitAdapter { execv }
    }
}

impl GitAdapter for ShellGitAdapter {
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        println!("[ShellGitAdapter] Listing submodules in {:?}", root_dir);
        let output = self.execv.execv(
            OsStr::new("git"),
            &[
                OsStr::new("submodule"),
                OsStr::new("status"),
                OsStr::new("--recursive"),
            ],
            Some(root_dir),
        )?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut submodules = Vec::new();
        for line in stdout.lines() {
            // Example line: ` 0a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t path/to/submodule (v1.0.0)`
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let path_str = parts[1];
                let path = PathBuf::from(path_str);
                // For simplicity, we'll use the path as the "URL" for now,
                // as the original `list_submodules` returned (url, path).
                // A more robust solution would parse .gitmodules.
                submodules.push((path_str.to_string(), path));
            }
        }
        Ok(submodules)
    }

    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        println!("[ShellGitAdapter] Getting submodule stat for {:?}", path);
        let head_output = self.execv.execv(
            OsStr::new("git"),
            &[OsStr::new("rev-parse"), OsStr::new("HEAD")],
            Some(path),
        )?;
        let head_commit = String::from_utf8_lossy(&head_output.stdout)
            .trim()
            .to_string();

        let status_output = self.execv.execv(
            OsStr::new("git"),
            &[OsStr::new("status"), OsStr::new("--porcelain")],
            Some(path),
        )?;
        let dirty = !status_output.stdout.is_empty();

        let workdir_hash = if dirty {
            format!("{}-dirty", head_commit)
        } else {
            head_commit.clone()
        };

        Ok(SubmoduleStat {
            head_commit,
            workdir_hash,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(feature = "git2")]
pub struct LibGitAdapter;

#[cfg(feature = "git2")]
impl LibGitAdapter {
    pub fn new() -> Self {
        LibGitAdapter
    }
}

#[cfg(feature = "git2")]
impl GitAdapter for LibGitAdapter {
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        println!("[LibGitAdapter] Listing submodules in {:?}", root_dir);
        let repo = Repository::open(root_dir)
            .context(format!("Failed to open git repository at {:?}", root_dir))?;

        let mut submodules_info = Vec::new();
        for submodule in repo.submodules()? {
            let path = submodule.path().to_path_buf();
            let url = submodule.url().unwrap_or_default().to_string();
            submodules_info.push((url, path));
        }
        Ok(submodules_info)
    }

    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        println!("[LibGitAdapter] Getting submodule stat for {:?}", path);
        let repo = Repository::open(path)
            .context(format!("Failed to open git repository at {:?}", path))?;

        let head = repo.head()?;
        let head_oid = head.target().context("Failed to get HEAD OID")?;
        let head_commit = head_oid.to_string();

        let statuses = repo.statuses(None)?;
        let dirty = statuses.iter().any(|s| s.status() != git2::Status::empty());

        let workdir_hash = if dirty {
            format!("{}-dirty", head_commit)
        } else {
            head_commit.clone()
        };

        Ok(SubmoduleStat {
            head_commit,
            workdir_hash,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
