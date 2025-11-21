use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::any::Any;

use crate::git_traits::GitExecutor;
use crate::git_types::{RollupLock, SubmoduleStat};

pub struct DummyGitExecutor;

impl DummyGitExecutor {
    pub fn new() -> Self {
        DummyGitExecutor
    }
}

impl GitExecutor for DummyGitExecutor {
    fn submodule_add(&self, _repo_url: &str, _submodule_path: &Path, _rollup_lock: Arc<Mutex<RollupLock>>, _root_dir: &Path) -> Result<()> {
        anyhow::bail!("DummyGitExecutor: submodule_add is not implemented.");
    }

    fn checkout_branch(&self, _submodule_path: &Path, _branch: &str, _rollup_lock: Arc<Mutex<RollupLock>>, _root_dir: &Path) -> Result<()> {
        anyhow::bail!("DummyGitExecutor: checkout_branch is not implemented.");
    }

    fn status(&self, _submodule_path: &Path) -> Result<String> {
        anyhow::bail!("DummyGitExecutor: status is not implemented.");
    }

    fn list_submodules(&self, _root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        anyhow::bail!("DummyGitExecutor: list_submodules is not implemented.");
    }

    fn clone(&self, _repo_url: &str, _target_path: &Path) -> Result<()> {
        anyhow::bail!("DummyGitExecutor: clone is not implemented.");
    }

    fn get_file_git_info(&self, _repo_path: &Path, _file_path: &Path) -> Result<(bool, Option<String>)> {
        anyhow::bail!("DummyGitExecutor: get_file_git_info is not implemented.");
    }

    fn get_submodule_head_and_workdir_hash(&self, _path: &Path) -> Result<SubmoduleStat> {
        anyhow::bail!("DummyGitExecutor: get_submodule_head_and_workdir_hash is not implemented.");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
