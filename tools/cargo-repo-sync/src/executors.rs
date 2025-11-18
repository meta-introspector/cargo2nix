use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::traits::execv::Execv;
use git2::{Repository, SubmoduleUpdateOptions};
use crate::fs_cache::FileSystemStat;
use crate::RollupLock;
use crate::repo_sync_lib::git_snapshot::create_snapshot;
use std::collections::HashSet;

// --- GitExecutor Trait ---
pub trait GitExecutor {
    fn submodule_add(&self, repo_url: &str, submodule_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()>;
    fn checkout_branch(&self, submodule_path: &Path, branch: &str, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()>;
    fn status(&self, submodule_path: &Path) -> Result<String>; // For submodule status
}

pub struct SystemGitExecutor {
    git_executable_path: PathBuf,
    executor: Arc<dyn Execv>,
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl SystemGitExecutor {
    pub fn new(git_executable_path: PathBuf, executor: Arc<dyn Execv>, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: PathBuf) -> Self {
        SystemGitExecutor {
            git_executable_path,
            executor,
            rollup_lock,
            root_dir,
        }
    }
}

impl GitExecutor for SystemGitExecutor {
    fn submodule_add(&self, repo_url: &str, submodule_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()> {
        println!("Executing git submodule add {} {:?}", repo_url, submodule_path);
        let output = self.executor.execv(
            &self.git_executable_path,
            &["submodule", "add", repo_url, &submodule_path.to_string_lossy()],
            None, // Added missing argument
        )?;

        if !output.status.success() {
            eprintln!(
                "Failed to add submodule {}: {}", // Corrected format string
                repo_url,
                String::from_utf8_lossy(&output.stderr)
            );
            anyhow::bail!("Adding submodule failed for {}", repo_url); // Corrected format string
        }
        println!("Successfully added {} as submodule.", repo_url);
        create_snapshot(root_dir, rollup_lock)?;
        Ok(())
    }

    fn checkout_branch(&self, submodule_path: &Path, branch: &str, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()> {
        println!("Executing git -C {:?} checkout {}", submodule_path, branch);
        let output = self.executor.execv(
            &self.git_executable_path,
            &["-C", &submodule_path.to_string_lossy(), "checkout", branch],
            None, // Added missing argument
        )?;

        if !output.status.success() {
            eprintln!(
                "Failed to checkout branch '{}' in submodule {:?}: {}", // Corrected format string
                branch,
                submodule_path,
                String::from_utf8_lossy(&output.stderr)
            );
            anyhow::bail!("Branch checkout failed for {:?}", submodule_path); // Corrected format string
        }
        println!("Successfully checked out branch '{}' in submodule {:?}.", branch, submodule_path);
        create_snapshot(root_dir, rollup_lock)?;
        Ok(())
    }

    fn status(&self, submodule_path: &Path) -> Result<String> {
        println!("Executing git -C {:?} status", submodule_path);
        let output = self.executor.execv(
            &self.git_executable_path,
            &["-C", &submodule_path.to_string_lossy(), "status"],
            None, // Added missing argument
        )?;

        if !output.status.success() {
            eprintln!(
                "Failed to get status for submodule {:?}: {}", // Corrected format string
                submodule_path,
                String::from_utf8_lossy(&output.stderr)
            );
            anyhow::bail!("Failed to get status for {:?}", submodule_path); // Corrected format string
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

// --- PureRustGitExecutor Implementation ---
pub struct PureRustGitExecutor {
    file_system_stat: Arc<dyn FileSystemStat>,
    rollup_lock: Arc<Mutex<RollupLock>>,
    root_dir: PathBuf,
}

impl PureRustGitExecutor {
    pub fn new(file_system_stat: Arc<dyn FileSystemStat>, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: PathBuf) -> Self {
        PureRustGitExecutor { file_system_stat, rollup_lock, root_dir }
    }
}

impl GitExecutor for PureRustGitExecutor {
    fn submodule_add(&self, repo_url: &str, submodule_path: &Path, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()> {
        println!("Executing pure Rust git submodule add {} {:?}", repo_url, submodule_path);
        let repo = Repository::open_from_env()
            .context("Failed to open current git repository")?;

        // Removed RepoBuilder related lines
        let _submodule = repo.submodule(repo_url, submodule_path, true) // true to initialize and update
            .context(format!("Failed to add submodule {} at {:?}", repo_url, submodule_path))?;

        // Initialize and update the submodule
        let mut submodule = repo.find_submodule(submodule_path.to_str().unwrap())
            .context(format!("Failed to find submodule after adding: {:?}", submodule_path))?;
        
        submodule.update(true, Some(&mut SubmoduleUpdateOptions::new()))
            .context(format!("Failed to update submodule: {:?}", submodule_path))?;

        // Stage changes to .gitmodules and the new submodule's gitlink
        let mut index = repo.index().context("Failed to get repository index")?;
        index.add_path(Path::new(".gitmodules")).context("Failed to add .gitmodules to index")?;
        index.add_path(submodule_path).context("Failed to add submodule path to index")?;
        index.write().context("Failed to write index")?;

        println!("Successfully added {} as submodule using pure Rust.", repo_url);
        create_snapshot(root_dir, rollup_lock)?;
        Ok(())
    }

    fn checkout_branch(&self, submodule_path: &Path, branch: &str, rollup_lock: Arc<Mutex<RollupLock>>, root_dir: &Path) -> Result<()> {
        println!("Executing pure Rust git -C {:?} checkout {}", submodule_path, branch);
        let submodule_repo = Repository::open(submodule_path)
            .context(format!("Failed to open submodule repository at {:?}", submodule_path))?;

        let (object, reference) = submodule_repo.revparse_ext(branch)
            .context(format!("Failed to find branch or commit '{}' in submodule {:?}", branch, submodule_path))?;

        submodule_repo.checkout_tree(&object, None)
            .context(format!("Failed to checkout tree for '{}' in submodule {:?}", branch, submodule_path))?;

        match reference {
            // gref is an actual reference like a branch or a tag
            Some(gref) => submodule_repo.set_head(gref.name().unwrap())
                .context(format!("Failed to set HEAD to reference '{}' in submodule {:?}", branch, submodule_path))?,
            // object is a detached commit (e.g., a SHA)
            None => submodule_repo.set_head_detached(object.id())
                .context(format!("Failed to set HEAD to detached commit '{}' in submodule {:?}", branch, submodule_path))?,
        };

        println!("Successfully checked out branch '{}' in submodule {:?} using pure Rust.", branch, submodule_path);
        create_snapshot(root_dir, rollup_lock)?;
        Ok(())
    }

    fn status(&self, repo_path: &Path) -> Result<String> {
        println!("Executing pure Rust git -C {:?} status (using cached fs stat)", repo_path);
        let repo = Repository::open(repo_path)
            .context(format!("Failed to open repository at {:?}", repo_path))?;

        let mut status_output = String::new();

        // Get HEAD tree
        let head_tree = repo.head()
            .and_then(|head| head.resolve())
            .and_then(|head| head.peel_to_tree())
            .context("Failed to get HEAD tree")?;

        // Get Git Index
        let index = repo.index().context("Failed to get repository index")?;

        let mut staged_changes: Vec<String> = Vec::new();
        let mut unstaged_changes: Vec<String> = Vec::new();
        let mut untracked_files: Vec<String> = Vec::new();

        // Keep track of paths seen in index and HEAD to avoid duplicates and detect deletions
        let mut seen_in_index: HashSet<PathBuf> = HashSet::new();
        let mut seen_in_head: HashSet<PathBuf> = HashSet::new();

        // Iterate through index entries to find staged and unstaged changes
        for entry in index.iter() {
            let path_str = String::from_utf8_lossy(&entry.path); // Removed .as_bytes()
            let path = PathBuf::from(&*path_str);
            seen_in_index.insert(path.clone());

            // Check for changes in working directory relative to index
            let cached_metadata_result = self.file_system_stat.get_metadata(&repo_path.join(&path));

            match cached_metadata_result {
                Ok(cached_metadata) => {
                    // File exists in working directory
                    let wd_hash = cached_metadata.hash;
                    let index_hash = entry.id.to_string();

                    if wd_hash != index_hash {
                        unstaged_changes.push(format!("M  {}", path.display()));
                    }
                },
                Err(_) => {
                    // File exists in index but not in working directory (deleted unstaged)
                    unstaged_changes.push(format!("D  {}", path.display()));
                }
            }

            // Check for changes in index relative to HEAD
            // Use head_tree directly as it's already unwrapped
            if let Ok(head_entry) = head_tree.get_path(&path) { // Corrected TreeWalkMode
                seen_in_head.insert(path.clone());
                // File exists in HEAD
                if head_entry.id().to_string() != entry.id.to_string() {
                    staged_changes.push(format!("M  {}", path.display()));
                }
            } else {
                // File exists in index but not in HEAD (added staged)
                staged_changes.push(format!("A  {}", path.display()));
            }
        }

        // Detect deleted (staged) files (exist in HEAD but not in index)
        // Use head_tree directly as it's already unwrapped
        head_tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let path = PathBuf::from(root).join(entry.name().unwrap());
            if !seen_in_index.contains(&path) && !path.to_string_lossy().is_empty() {
                // File exists in HEAD but not in index
                staged_changes.push(format!("D  {}", path.display()));
            }
            git2::TreeWalkResult::Ok
        }).context("Failed to walk HEAD tree for deleted staged files")?;


        // Detect untracked files using git2's status API (pragmatic approach)
        let mut untracked_options = git2::StatusOptions::new();
        untracked_options.include_untracked(true);
        untracked_options.recurse_untracked_dirs(true);
        untracked_options.exclude_submodules(false);
        untracked_options.include_ignored(false); // Don't show ignored files
        // untracked_options.flags(Status::OPT_INCLUDE_UNTRACKED | Status::OPT_EXCLUDE_SUBMODULES); // Removed flags method

        let statuses = repo.statuses(Some(&mut untracked_options))
            .context(format!("Failed to get untracked statuses for {:?}", repo_path))?;

        for entry in statuses.iter() {
            if entry.status().is_wt_new() {
                let path = entry.path().unwrap_or("unknown");
                untracked_files.push(format!("?? {}", path));
            }
        }

        // Format output
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
}

// --- GhExecutor Trait ---
pub trait GhExecutor {
    fn repo_fork(&self, repo_url: &str, target_org: &str) -> Result<()>;
    fn repo_view(&self, forked_repo_url: &str) -> Result<bool>; // Returns true if repo exists
}

pub struct SystemGhExecutor {
    gh_executable_path: PathBuf,
    executor: Arc<dyn Execv>,
}

impl SystemGhExecutor {
    pub fn new(gh_executable_path: PathBuf, executor: Arc<dyn Execv>) -> Self {
        SystemGhExecutor {
            gh_executable_path,
            executor,
        }
    }
}

impl GhExecutor for SystemGhExecutor {
    fn repo_fork(&self, repo_url: &str, target_org: &str) -> Result<()> {
        println!("Executing gh repo fork {} --org {}", repo_url, target_org);
        let fork_output = self.executor.execv(
            &self.gh_executable_path,
            &["repo", "fork", repo_url, "--org", target_org, "--remote", "--clone=false"],
            None, // Added missing argument
        )?;

        if !fork_output.status.success() {
            eprintln!(
                "Failed to fork {}: {}", // Corrected format string
                repo_url,
                String::from_utf8_lossy(&fork_output.stderr)
            );
            anyhow::bail!("Forking failed for {}", repo_url);
        }
        println!("Successfully forked {}.", repo_url);
        Ok(())
    }

    fn repo_view(&self, forked_repo_url: &str) -> Result<bool> {
        println!("Executing gh repo view {} --json name", forked_repo_url);
        let gh_repo_check_output = self.executor.execv(
            &self.gh_executable_path,
            &["repo", "view", forked_repo_url, "--json", "name"],
            None, // Added missing argument
        )?;

        Ok(gh_repo_check_output.status.success() && !String::from_utf8_lossy(&gh_repo_check_output.stdout).trim().is_empty())
    }
}
