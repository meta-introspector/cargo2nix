use std::io;
use std::path::{Path, PathBuf};

pub trait WalkDirAdapter: Send + Sync {
    fn new(path: &Path) -> Self
    where
        Self: Sized;
    fn into_iter(&self) -> Box<dyn Iterator<Item = Result<PathBuf, io::Error>> + '_>;
}
