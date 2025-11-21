use std::path::{Path, PathBuf};
use git2;

// Struct to hold submodule information
#[derive(Debug)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: Option<String>,
}

// Define a trait for Git repository operations
pub trait GitRepositoryOperations {
    fn open_repository(path: &Path) -> Result<Self, String> where Self: Sized;
    fn submodules(&self) -> Result<Vec<SubmoduleInfo>, String>;
}

// Implement the trait for git2::Repository
impl GitRepositoryOperations for git2::Repository {
    fn open_repository(path: &Path) -> Result<Self, String> {
        git2::Repository::open(path)
            .map_err(|e| format!("Failed to open repository at {:?}: {}", path, e))
    }

    fn submodules(&self) -> Result<Vec<SubmoduleInfo>, String> {
        let mut submodules_info = Vec::new();
        for submodule in self.submodules()
            .map_err(|e| format!("Failed to iterate submodules: {}", e))?
        {
            submodules_info.push(SubmoduleInfo {
                name: submodule.name().unwrap_or("unknown").to_string(),
                path: submodule.path().to_path_buf(),
                url: submodule.url().map(|s| s.to_string()),
            });
        }
        Ok(submodules_info)
    }
}