use anyhow::Result;
use std::path::{Path, PathBuf};
use super::submodule_stat::SubmoduleStat;

pub trait SubmoduleStatProvider: Send + Sync {
    fn get_submodule_stat(&self, path: &Path) -> Result<SubmoduleStat>;
    fn update_submodule_stat(&self, path: PathBuf, stat: SubmoduleStat) -> Result<()>;
}
