use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

// Helper function to check if a file is git-ignored
pub fn is_git_ignored(
    repo_path: &Path,
    file_path: &Path,
    executor: Arc<dyn Execv + Send + Sync>,
) -> Result<bool, String> {
    let git_executable_path = PathBuf::from("git"); // Assuming 'git' is in PATH
    let output = executor
        .execv(
            git_executable_path.as_os_str(),
            &[
                OsStr::new("-C"),
                repo_path.as_os_str(),
                OsStr::new("check-ignore"),
                file_path.as_os_str(),
            ],
            None,
        )
        .map_err(|e| format!("Failed to execute git check-ignore: {}", e))?;

    Ok(output.status.success())
}
