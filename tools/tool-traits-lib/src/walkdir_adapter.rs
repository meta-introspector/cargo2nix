use std::io;
use std::path::{Path, PathBuf};

pub trait WalkDirAdapter: Send + Sync {
    fn new(path: &Path) -> Self
    where
        Self: Sized;
    fn into_iter(&self) -> Box<dyn Iterator<Item = Result<PathBuf, io::Error>> + '_>;
}

#[cfg(not(feature = "walkdir_enabled"))]
pub struct DummyWalkDirAdapter;

#[cfg(not(feature = "walkdir_enabled"))]
impl WalkDirAdapter for DummyWalkDirAdapter {
    fn new(_path: &Path) -> Self {
        DummyWalkDirAdapter
    }

    fn into_iter(&self) -> Box<dyn Iterator<Item = Result<PathBuf, io::Error>> + '_> {
        Box::new(std::iter::empty())
    }
}

#[cfg(all(
    feature = "walkdir_enabled",
    feature = "real_walkdir_adapter_lib_enabled"
))]
pub type CurrentWalkDirAdapter = real_walkdir_adapter_lib::RealWalkDirAdapter;
#[cfg(not(all(
    feature = "walkdir_enabled",
    feature = "real_walkdir_adapter_lib_enabled"
)))]
pub type CurrentWalkDirAdapter = DummyWalkDirAdapter;
