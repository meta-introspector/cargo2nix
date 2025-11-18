use anyhow::{Context, Result};
use std::{
    path::{Path, PathBuf},

    sync::Arc,
};
use super::submodule_stat::SubmoduleStat;
use super::submodule_stat_provider::SubmoduleStatProvider;
use crate::traits::execv::Execv;
use md5;

// RealSubmoduleStatProvider will interact with git commands
pub struct RealSubmoduleStatProvider {
    pub git_executable_path: PathBuf,
    pub base_executor: Arc<dyn Execv>,
}

impl SubmoduleStatProvider for RealSubmoduleStatProvider {
    fn get_submodule_stat(&self, path: &Path) -> Result<SubmoduleStat> {
        // Get HEAD commit
        let head_commit_output = self.base_executor.execute(
            self.git_executable_path.to_str().unwrap(),
            &["rev-parse", "HEAD"],
            Some(path),
        ).context(format!("Failed to get HEAD commit for {:?}", path))?;

        if !head_commit_output.status.success() {
            anyhow::bail!(
                "git rev-parse HEAD failed for {:?}: {}",
                path,
                String::from_utf8_lossy(&head_commit_output.stderr)
            );
        }
        let head_commit = String::from_utf8_lossy(&head_commit_output.stdout).trim().to_string();

        // Get workdir hash (simplified: hash of git status --porcelain output)
        let workdir_status_output = self.base_executor.execute(
            self.git_executable_path.to_str().unwrap(),
            &["status", "--porcelain"],
            Some(path),
        ).context(format!("Failed to get workdir status for {:?}", path))?;

        if !workdir_status_output.status.success() {
            anyhow::bail!(
                "git status --porcelain failed for {:?}: {}",
                path,
                String::from_utf8_lossy(&workdir_status_output.stderr)
            );
        }
        let workdir_hash = format!("{:x}", md5::compute(workdir_status_output.stdout));

        Ok(SubmoduleStat {
            head_commit,
            workdir_hash,
        })
    }

    fn update_submodule_stat(&self, _path: PathBuf, _stat: SubmoduleStat) -> Result<()> {
        // This implementation doesn't update anything, as it's the "real" provider
        Ok(())
    }
}
