use std::io;
use std::path::{Path, PathBuf};
use tool_traits_lib::walkdir_adapter::WalkDirAdapter;
use walkdir::{DirEntry, WalkDir};

pub struct RealWalkDirAdapter {
    walkdir: WalkDir,
}

impl WalkDirAdapter for RealWalkDirAdapter {
    fn new(path: &Path) -> Self {
        RealWalkDirAdapter {
            walkdir: WalkDir::new(path),
        }
    }

    fn into_iter(&self) -> Box<dyn Iterator<Item = Result<PathBuf, io::Error>> + '_> {
        Box::new(self.walkdir.clone().into_iter().map(|entry_result| {
            entry_result
                .map(|entry| entry.into_path())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        }))
    }
}
