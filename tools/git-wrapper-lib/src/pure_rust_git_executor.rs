#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
use std::error::Error; // For fallback Result
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn Error>>; // Fallback for Result
#[cfg(not(feature = "anyhow_enabled"))]
trait Context<T> {
    // Fallback for anyhow::Context
    fn context<C>(self, _context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}
#[cfg(not(feature = "anyhow_enabled"))]
impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, _context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::git_traits::GitExecutor;
use crate::git_types::{RollupLock, SubmoduleStat};
// use crate::repo_sync_lib::git_snapshot::create_snapshot; // Commented out

#[cfg(feature = "git2_enabled")]
use git2::{Repository, SubmoduleUpdateOptions};
#[cfg(feature = "hex_enabled")]
use hex;
#[cfg(feature = "sha1_enabled")]
use sha1::{Digest, Sha1};

pub struct PureRustGitExecutor {
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl PureRustGitExecutor {
    pub fn new(rollup_lock: Arc<Mutex<RollupLock>>, root_dir: PathBuf) -> Self {
        PureRustGitExecutor {
            rollup_lock,
            root_dir,
        }
    }
}

impl GitExecutor for PureRustGitExecutor {
    #[cfg(feature = "git2_enabled")]
    fn submodule_add(
        &self,
        _repo_url: &str,
        _submodule_path: &Path,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: &Path,
    ) -> Result<()> {
        println!(
            "Executing pure Rust git submodule add {} ?{:?}",
            repo_url, submodule_path
        );
        let repo = Repository::open_from_env().context("Failed to open current git repository")?;

        let _submodule = repo
            .submodule(repo_url, submodule_path, true) // true to initialize and update
            .context(format!(
                "Failed to add submodule {} at ?{:?}",
                repo_url, submodule_path
            ))?;

        let mut submodule = repo
            .find_submodule(submodule_path.to_str().unwrap())
            .context(format!(
                "Failed to find submodule after adding: ?{:?}",
                submodule_path
            ))?;

        submodule
            .update(true, Some(&mut SubmoduleUpdateOptions::new()))
            .context(format!("Failed to update submodule: ?{:?}", submodule_path))?;

        let mut index = repo.index().context("Failed to get repository index")?;
        index
            .add_path(Path::new(".gitmodules"))
            .context("Failed to add .gitmodules to index")?;
        index
            .add_path(submodule_path)
            .context("Failed to add submodule path to index")?;
        index.write().context("Failed to write index")?;

        println!(
            "Successfully added {} as submodule using pure Rust.",
            repo_url
        );
        // create_snapshot(root_dir, rollup_lock, Arc::new(self.clone()))?; // Commented out
        Ok(())
    }

    #[cfg(not(feature = "git2_enabled"))]
    fn submodule_add(
        &self,
        repo_url: &str,
        submodule_path: &Path,
        _rollup_lock: Arc<Mutex<RollupLock>>,
        _root_dir: &Path,
    ) -> Result<()> {
        #[cfg(feature = "with-trace")]
        println!(
            "TRACE: submodule_add called with repo_url: {}, submodule_path: {:?}",
            repo_url, submodule_path
        );
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!(
            "PureRustGitExecutor::submodule_add requires the 'git2' feature, which is not enabled."
        );
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "PureRustGitExecutor::submodule_add requires the 'git2' feature, which is not enabled.",
        )));
    }

    #[cfg(feature = "git2_enabled")]
    fn checkout_branch(
        &self,
        _submodule_path: &Path,
        _branch: &str,
        rollup_lock: Arc<Mutex<RollupLock>>,
        root_dir: &Path,
    ) -> Result<()> {
        println!(
            "Executing pure Rust git -C ?{:?} checkout ?{}",
            submodule_path, branch
        );
        let submodule_repo = Repository::open(submodule_path).context(format!(
            "Failed to open submodule repository at ?{:?}",
            submodule_path
        ))?;

        let (object, reference) = submodule_repo.revparse_ext(branch).context(format!(
            "Failed to find branch or commit '{}' in submodule ?{:?}",
            branch, submodule_path
        ))?;

        submodule_repo
            .checkout_tree(&object, None)
            .context(format!(
                "Failed to checkout tree for '{}' in submodule ?{:?}",
                branch, submodule_path
            ))?;

        match reference {
            Some(gref) => submodule_repo
                .set_head(gref.name().unwrap())
                .context(format!(
                    "Failed to set HEAD to reference '{}' in submodule ?{:?}",
                    branch, submodule_path
                ))?,
            None => submodule_repo
                .set_head_detached(object.id())
                .context(format!(
                    "Failed to set HEAD to detached commit '{}' in submodule ?{:?}",
                    branch, submodule_path
                ))?,
        };

        println!(
            "Successfully checked out branch '{}' in submodule ?{:?} using pure Rust.",
            branch, submodule_path
        );
        // create_snapshot(root_dir, rollup_lock, Arc::new(self.clone()))?; // Commented out
        Ok(())
    }

    #[cfg(not(feature = "git2_enabled"))]
    fn checkout_branch(
        &self,
        submodule_path: &Path,
        branch: &str,
        _rollup_lock: Arc<Mutex<RollupLock>>,
        _root_dir: &Path,
    ) -> Result<()> {
        #[cfg(feature = "with-trace")]
        println!(
            "TRACE: checkout_branch called with submodule_path: {:?}, branch: {}",
            submodule_path, branch
        );
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!("PureRustGitExecutor::checkout_branch requires the 'git2' feature, which is not enabled.");
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "PureRustGitExecutor::checkout_branch requires the 'git2' feature, which is not enabled.")));
    }

    #[cfg(feature = "git2_enabled")]
    fn status(&self, repo_path: &Path) -> Result<String> {
        println!(
            "Executing pure Rust git -C ?{:?} status (using cached fs stat)",
            repo_path
        );
        let repo = Repository::open(repo_path)
            .context(format!("Failed to open repository at ?{:?}", repo_path))?;

        let mut status_output = String::new();

        let head_tree = repo
            .head()
            .and_then(|head| head.resolve())
            .and_then(|head| head.peel_to_tree())
            .context("Failed to get HEAD tree")?;

        let index = repo.index().context("Failed to get repository index")?;

        let mut staged_changes: Vec<String> = Vec::new();
        let mut unstaged_changes: Vec<String> = Vec::new();
        let mut untracked_files: Vec<String> = Vec::new();

        let mut seen_in_index: std::collections::HashSet<PathBuf> =
            std::collections::HashSet::new(); // Moved to local scope
        let mut seen_in_head: std::collections::HashSet<PathBuf> = std::collections::HashSet::new(); // Moved to local scope

        for entry in index.iter() {
            let path_str = String::from_utf8_lossy(&entry.path);
            let path = PathBuf::from(&*path_str);
            seen_in_index.insert(path.clone());

            if let Ok(status_entry) = repo.status_file(&path) {
                if status_entry.is_wt_modified() {
                    unstaged_changes.push(format!("M  {}", path.display()));
                } else if status_entry.is_wt_deleted() {
                    unstaged_changes.push(format!("D  {}", path.display()));
                }
            }

            if let Ok(head_entry) = head_tree.get_path(&path) {
                seen_in_head.insert(path.clone());
                if head_entry.id().to_string() != entry.id.to_string() {
                    staged_changes.push(format!("M  {}", path.display()));
                }
            } else {
                staged_changes.push(format!("A  {}", path.display()));
            }
        }

        head_tree
            .walk(git2::TreeWalkMode::PreOrder, |root, entry| {
                let path = PathBuf::from(root).join(entry.name().unwrap());
                if !seen_in_index.contains(&path) && !path.to_string_lossy().is_empty() {
                    staged_changes.push(format!("D  {}", path.display()));
                }
                git2::TreeWalkResult::Ok
            })
            .context("Failed to walk HEAD tree for deleted staged files")?;

        let mut untracked_options = git2::StatusOptions::new();
        untracked_options.include_untracked(true);
        untracked_options.recurse_untracked_dirs(true);
        untracked_options.exclude_submodules(false);
        untracked_options.include_ignored(false);

        let statuses = repo
            .statuses(Some(&mut untracked_options))
            .context(format!(
                "Failed to get untracked statuses for ?{:?}",
                repo_path
            ))?;

        for entry in statuses.iter() {
            if entry.status().is_wt_new() {
                let path = entry.path().unwrap_or("unknown");
                untracked_files.push(format!("?? {}", path));
            }
        }

        if !staged_changes.is_empty() {
            status_output.push_str("Changes to be committed:\n");
            for change in staged_changes {
                status_output.push_str(&format!("{}\n", change));
            }
            status_output.push_str("\n");
        }

        if !unstaged_changes.is_empty() {
            status_output.push_str("Changes not staged for commit:\n");
            for change in unstaged_changes {
                status_output.push_str(&format!("{}\n", change));
            }
            status_output.push_str("\n");
        }

        if !untracked_files.is_empty() {
            status_output.push_str("Untracked files:\n");
            for file in untracked_files {
                status_output.push_str(&format!("{}\n", file));
            }
            status_output.push_str("\n");
        }

        if status_output.is_empty() {
            status_output.push_str("No changes detected.\n");
        }

        Ok(status_output)
    }

    #[cfg(not(feature = "git2_enabled"))]
    fn status(&self, _repo_path: &Path) -> Result<String> {
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!(
            "PureRustGitExecutor::status requires the 'git2' feature, which is not enabled."
        );
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "PureRustGitExecutor::status requires the 'git2' feature, which is not enabled.",
        )));
    }

    #[cfg(feature = "git2_enabled")]
    fn list_submodules(&self, root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        println!("Executing pure Rust git list submodules in ?{:?}", root_dir);
        let repo = Repository::open(root_dir)
            .context(format!("Failed to open repository at ?{:?}", root_dir))?;

        let mut submodules_list = Vec::new();
        for submodule in repo.submodules().context("Failed to get submodules")? {
            if let Some(url) = submodule.url() {
                if let Some(path) = submodule.path().to_str() {
                    submodules_list.push((url.to_string(), PathBuf::from(path)));
                }
            }
        }
        Ok(submodules_list)
    }

    #[cfg(not(feature = "git2_enabled"))]
    fn list_submodules(&self, _root_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!("PureRustGitExecutor::list_submodules requires the 'git2' feature, which is not enabled.");
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "PureRustGitExecutor::list_submodules requires the 'git2' feature, which is not enabled.")));
    }

    #[cfg(feature = "git2_enabled")]
    fn clone(&self, repo_url: &str, target_path: &Path) -> Result<()> {
        println!(
            "Executing pure Rust git clone {} ?{:?}",
            repo_url, target_path
        );
        git2::Repository::clone(repo_url, target_path).context(format!(
            "Failed to clone repository {} to ?{:?}",
            repo_url, target_path
        ))?;
        println!(
            "Successfully cloned {} to ?{:?} using pure Rust.",
            repo_url, target_path
        );
        Ok(())
    }

    fn clone(&self, _repo_url: &str, _target_path: &Path) -> Result<()> {
        #[cfg(feature = "with-trace")]
        println!(
            "TRACE: clone called with repo_url: {}, target_path: {:?}",
            repo_url, target_path
        );
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!(
            "PureRustGitExecutor::clone requires the 'git2' feature, which is not enabled."
        );
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "PureRustGitExecutor::clone requires the 'git2' feature, which is not enabled.",
        )));
    }

    #[cfg(feature = "git2_enabled")]
    fn get_file_git_info(
        &self,
        repo_path: &Path,
        file_path: &Path,
    ) -> Result<(bool, Option<String>)> {
        let repo = Repository::open(repo_path)
            .context(format!("Failed to open repository at ?{:?}", repo_path))?;

        let relative_path = file_path.strip_prefix(repo_path).unwrap_or(file_path);

        let mut is_git_tracked = false;
        let mut git_object_hash = None;

        if let Ok(status) = repo.status_file(relative_path) {
            if status.is_empty() {
                // No changes, so it's tracked and clean
                is_git_tracked = true;
                // Try to get the Oid for the file/blob
                if let Ok(object) =
                    repo.revparse_single(&format!("HEAD:{}", relative_path.display()))
                {
                    git_object_hash = Some(object.id().to_string());
                }
            } else if status.is_wt_new()
                || status.is_wt_modified()
                || status.is_wt_deleted()
                || status.is_index_new()
                || status.is_index_modified()
                || status.is_index_deleted()
            {
                // It's tracked but has changes, so we still consider it tracked
                is_git_tracked = true;
            }
        }

        Ok((is_git_tracked, git_object_hash))
    }

    #[cfg(not(feature = "git2_enabled"))]
    fn get_file_git_info(
        &self,
        _repo_path: &Path,
        _file_path: &Path,
    ) -> Result<(bool, Option<String>)> {
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!("PureRustGitExecutor::get_file_git_info requires the 'git2' feature, which is not enabled.");
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "PureRustGitExecutor::get_file_git_info requires the 'git2' feature, which is not enabled.")));
    }

    #[cfg(all(feature = "git2_enabled", feature = "sha1_enabled"))]
    fn get_submodule_head_and_workdir_hash(&self, path: &Path) -> Result<SubmoduleStat> {
        let repo =
            Repository::open(path).context(format!("Failed to open repository at ?{:?}", path))?;

        let head_commit = repo
            .head()?
            .target()
            .context("Failed to get HEAD target")?
            .to_string();

        let mut status_options = git2::StatusOptions::new();
        status_options.include_untracked(true);
        status_options.recurse_untracked_dirs(true);
        status_options.exclude_submodules(false);
        status_options.include_ignored(false);

        let statuses = repo
            .statuses(Some(&mut status_options))
            .context(format!("Failed to get statuses for ?{:?}", path))?;

        let mut workdir_status_string = String::new();
        for entry in statuses.iter() {
            if let Some(path_str) = entry.path() {
                workdir_status_string.push_str(&format!("{:?}{}\n", entry.status(), path_str));
            }
        }

        let mut hasher = Sha1::new();
        hasher.update(workdir_status_string.as_bytes());
        #[cfg(feature = "hex_enabled")]
        let workdir_hash = hex::encode(hasher.finalize());
        #[cfg(not(feature = "hex_enabled"))]
        let workdir_hash = format!("{:?}", hasher.finalize());

        Ok(SubmoduleStat {
            head_commit,
            workdir_hash,
        })
    }

    #[cfg(not(all(feature = "git2_enabled", feature = "sha1_enabled")))]
    fn get_submodule_head_and_workdir_hash(&self, _path: &Path) -> Result<SubmoduleStat> {
        #[cfg(feature = "anyhow_enabled")]
        anyhow::bail!("PureRustGitExecutor::get_submodule_head_and_workdir_hash requires both 'git2' and 'sha1' features, which are not enabled.");
        #[cfg(not(feature = "anyhow_enabled"))]
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "PureRustGitExecutor::get_submodule_head_and_workdir_hash requires both 'git2' and 'sha1' features, which are not enabled.")));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
