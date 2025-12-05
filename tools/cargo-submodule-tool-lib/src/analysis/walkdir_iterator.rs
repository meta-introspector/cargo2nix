// cargo-submodule-tool-lib/src/analysis/walkdir_iterator.rs
use anyhow::Result;
use std::path::{Path, PathBuf};
use tool_traits_lib::WalkDirIterator; // Using anyhow for Result type

#[cfg(feature = "walkdir_enabled")]
use walkdir::WalkDir;

#[cfg(feature = "walkdir_enabled")]
pub struct RealWalkDirIterator {
    walkdir: WalkDir,
}

#[cfg(feature = "walkdir_enabled")]
impl WalkDirIterator for RealWalkDirIterator {
    fn new(path: &Path) -> Self {
        RealWalkDirIterator {
            walkdir: WalkDir::new(path),
        }
    }

    fn into_iter(self) -> Box<dyn Iterator<Item = std::result::Result<PathBuf, String>> + Send> {
        Box::new(self.walkdir.into_iter().map(|entry_result| {
            entry_result
                .map(|entry| entry.path().to_path_buf())
                .map_err(|e| e.to_string())
        }))
    }
}

#[cfg(not(feature = "walkdir_enabled"))]
pub struct DummyWalkDirIterator;

#[cfg(not(feature = "walkdir_enabled"))]
impl WalkDirIterator for DummyWalkDirIterator {
    fn new(_path: &Path) -> Self {
        DummyWalkDirIterator
    }

    fn into_iter(self) -> Box<dyn Iterator<Item = std::result::Result<PathBuf, String>> + Send> {
        Box::new(std::iter::empty()) // Dummy implementation returns an empty iterator
    }
}

#[cfg(feature = "walkdir_enabled")]
pub type CurrentWalkDirIterator = RealWalkDirIterator;
#[cfg(not(feature = "walkdir_enabled"))]
pub type CurrentWalkDirIterator = DummyWalkDirIterator;
