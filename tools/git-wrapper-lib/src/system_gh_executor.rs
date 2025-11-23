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

use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

use crate::git_traits::Execv;
use crate::git_traits::GhExecutor; // Corrected import

pub struct SystemGhExecutor {
    gh_executable_path: PathBuf,
    executor: Arc<dyn Execv + Send + Sync>,
}

impl SystemGhExecutor {
    pub fn new(gh_executable_path: PathBuf, executor: Arc<dyn Execv + Send + Sync>) -> Self {
        SystemGhExecutor {
            gh_executable_path,
            executor,
        }
    }
}

impl GhExecutor for SystemGhExecutor {
    fn repo_fork(&self, repo_url: &str, target_org: &str) -> Result<()> {
        println!("Executing gh repo fork {} --org ?{}", repo_url, target_org);
        let program = self.gh_executable_path.as_os_str();
        let args = &[
            OsStr::new("repo"),
            OsStr::new("fork"),
            OsStr::new(repo_url),
            OsStr::new("--org"),
            OsStr::new(target_org),
            OsStr::new("--remote"),
            OsStr::new("--clone=false"),
        ];
        let fork_output = self.executor.execv(program, args, None)?;

        if !fork_output.status.success() {
            eprintln!(
                "Failed to fork {}: {}",
                repo_url,
                String::from_utf8_lossy(&fork_output.stderr)
            );
            #[cfg(feature = "anyhow_enabled")]
            anyhow::bail!("Forking failed for {}", repo_url);
            #[cfg(not(feature = "anyhow_enabled"))]
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Forking failed for {}", repo_url),
            )));
        }
        println!("Successfully forked {}.", repo_url);
        Ok(())
    }

    fn repo_view(&self, forked_repo_url: &str) -> Result<bool> {
        println!("Executing gh repo view {} --json name", forked_repo_url);
        let program = self.gh_executable_path.as_os_str();
        let args = &[
            OsStr::new("repo"),
            OsStr::new("view"),
            OsStr::new(forked_repo_url),
            OsStr::new("--json"),
            OsStr::new("name"),
        ];
        let gh_repo_check_output = self.executor.execv(program, args, None)?;

        Ok(gh_repo_check_output.status.success()
            && !String::from_utf8_lossy(&gh_repo_check_output.stdout)
                .trim()
                .is_empty())
    }
}
pub mod mock_gh_executor;
