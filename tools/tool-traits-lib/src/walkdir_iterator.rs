use std::path::{Path, PathBuf};

pub trait WalkDirIterator: Send + Sync {
    fn new(path: &Path) -> Self;
    fn into_iter(self) -> Box<dyn Iterator<Item = std::result::Result<PathBuf, String>> + Send>;
}
