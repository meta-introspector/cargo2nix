use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};


use crate::git_operations::{GitRepositoryOperations, RealGitRepositoryOperations};

#[derive(Debug)]
struct CrateInfo {
    name: String,
    #[allow(dead_code)]
    path: PathBuf,
    repository_url: Option<String>,
}

// Helper function to check if a given path is already a git submodule
fn is_submodule(repo_root: &Path, submodule_path: &Path) -> Result<bool, String> {
    let repo = git2::Repository::open(repo_root)
        .map_err(|e| format!("Failed to open repository at {:?}: {}", repo_root, e))?;

    let relative_submodule_path = submodule_path.strip_prefix(repo_root)
        .map_err(|e| format!("Failed to get relative path for {:?} from {:?}: {}", submodule_path, repo_root, e))?;

    for submodule in repo.submodules()
        .map_err(|e| format!("Failed to iterate submodules: {}", e))?
    {
        if let Some(path) = submodule.path().to_str() {
            if Path::new(path) == relative_submodule_path {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// Helper function to manage remotes (rename origin to upstream, add new origin)
fn manage_remotes(repo_path: &Path, original_repo_url: &str, target_org: &str, log_file: &mut File, dry_run: bool) -> Result<(), String> {
    writeln!(log_file, "Managing remotes for repository at {:?}", repo_path).map_err(|e| e.to_string())?;
    println!("Managing remotes for repository at {:?}", repo_path);

    let repo = git2::Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository at {:?}: {}", repo_path, e))?;

    // 1. Rename 'origin' to 'upstream'
    if repo.find_remote("origin").is_ok() {
        if dry_run {
            writeln!(log_file, "[DRY RUN] Would rename remote 'origin' to 'upstream' in {:?}", repo_path).map_err(|e| e.to_string())?;
            println!("[DRY RUN] Would rename remote 'origin' to 'upstream' in {:?}", repo_path);
        } else {
            repo.remote_rename("origin", "upstream")
                .map_err(|e| format!("Failed to rename remote 'origin' to 'upstream' in {:?}: {}", repo_path, e))?;
            writeln!(log_file, "Renamed remote 'origin' to 'upstream' in {:?}", repo_path).map_err(|e| e.to_string())?;
            println!("Renamed remote 'origin' to 'upstream' in {:?}", repo_path);
        }
    }

    // 2. Add new 'origin' pointing to meta-introspector fork
    // Assuming original_repo_url is like "https://github.com/owner/repo.git" or "git@github.com:owner/repo.git"
    let new_origin_url = if original_repo_url.starts_with("https://") {
        let parts: Vec<&str> = original_repo_url.split('/').collect();
        if parts.len() >= 5 {
            format!("https://github.com/{}/{}.git", target_org, parts[4].trim_end_matches(".git"))
        } else {
            return Err(format!("Invalid original repository URL format: {}", original_repo_url));
        }
    } else if original_repo_url.starts_with("git@") {
        let parts: Vec<&str> = original_repo_url.split(':').collect();
        if parts.len() >= 2 {
            let repo_name_parts: Vec<&str> = parts[1].split('/').collect();
            if repo_name_parts.len() >= 2 {
                format!("git@github.com:{}/{}.git", target_org, repo_name_parts[1].trim_end_matches(".git"))
            } else {
                return Err(format!("Invalid original repository URL format: {}", original_repo_url));
            }
        } else {
            return Err(format!("Invalid original repository URL format: {}", original_repo_url));
        }
    } else {
        return Err(format!("Unsupported repository URL scheme: {}", original_repo_url));
    };

    if repo.find_remote("origin").is_err() { // Only add if 'origin' doesn't exist (after renaming the old one)
        if dry_run {
            writeln!(log_file, "[DRY RUN] Would add new remote 'origin' with URL '{}' in {:?}", new_origin_url, repo_path).map_err(|e| e.to_string())?;
            println!("[DRY RUN] Would add new remote 'origin' with URL '{}' in {:?}", new_origin_url, repo_path);
        } else {
            repo.remote("origin", &new_origin_url)
                .map_err(|e| format!("Failed to add new remote 'origin' with URL '{}' in {:?}: {}", new_origin_url, repo_path, e))?;
            writeln!(log_file, "Added new remote 'origin' with URL '{}' in {:?}", new_origin_url, repo_path).map_err(|e| e.to_string())?;
            println!("Added new remote 'origin' with URL '{}' in {:?}", new_origin_url, repo_path);
        }
    } else {
        writeln!(log_file, "Remote 'origin' already exists in {:?}. Skipping adding new origin.", repo_path).map_err(|e| e.to_string())?;
        println!("Remote 'origin' already exists in {:?}. Skipping adding new origin.", repo_path);
    }

    Ok(())
}

// Helper function to clone a repository
fn clone_repository(repo_url: &str, target_path: &Path, log_file: &mut File, dry_run: bool) -> Result<(), String> {
    writeln!(log_file, "Attempting to clone {} to {:?}", repo_url, target_path).map_err(|e| e.to_string())?;
    println!("Attempting to clone {} to {:?}", repo_url, target_path);

    if dry_run {
        writeln!(log_file, "[DRY RUN] Would clone {} to {:?}", repo_url, target_path).map_err(|e| e.to_string())?;
        println!("[DRY RUN] Would clone {} to {:?}", repo_url, target_path);
        return Ok(());
    }

    if target_path.exists() {
        writeln!(log_file, "Target path {:?} already exists. Skipping clone.", target_path).map_err(|e| e.to_string())?;
        println!("Target path {:?} already exists. Skipping clone.", target_path);
        return Ok(());
    }

    git2::Repository::clone(repo_url, target_path)
        .map_err(|e| format!("Failed to clone repository {}: {}", repo_url, e))?;

    writeln!(log_file, "Successfully cloned {} to {:?}", repo_url, target_path).map_err(|e| e.to_string())?;
    println!("Successfully cloned {} to {:?}", repo_url, target_path);
    Ok(())
}

use std::process::Command; // Add this import

// Helper function to checkout a specific branch
fn checkout_branch(repo_path: &Path, branch_name: &str, log_file: &mut File, dry_run: bool) -> Result<(), String> {
    writeln!(log_file, "Attempting to checkout branch '{}' in {:?}", branch_name, repo_path).map_err(|e| e.to_string())?;
    println!("Attempting to checkout branch '{}' in {:?}", branch_name, repo_path);

    if dry_run {
        writeln!(log_file, "[DRY RUN] Would checkout branch '{}' in {:?}", branch_name, repo_path).map_err(|e| e.to_string())?;
        println!("[DRY RUN] Would checkout branch '{}' in {:?}", branch_name, repo_path);
        return Ok(());
    }

    let repo = git2::Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository at {:?}: {}", repo_path, e))?;

    let (object, reference) = repo.revparse_ext(branch_name)
        .map_err(|e| format!("Failed to revparse branch '{}' in {:?}: {}", branch_name, repo_path, e))?;

    repo.checkout_tree(&object, None)
        .map_err(|e| format!("Failed to checkout tree for branch '{}' in {:?}: {}", branch_name, repo_path, e))?;

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap())
            .map_err(|e| format!("Failed to set HEAD to branch '{}' in {:?}: {}", branch_name, repo_path, e))?,
        // this was a commit, not a reference
        None => repo.set_head_detached(object.id())
            .map_err(|e| format!("Failed to set HEAD detached to commit '{}' in {:?}: {}", object.id(), repo_path, e))?,
    };

    writeln!(log_file, "Successfully checked out branch '{}' in {:?}", branch_name, repo_path).map_err(|e| e.to_string())?;
    println!("Successfully checked out branch '{}' in {:?}", branch_name, repo_path);
    Ok(())
}

// Helper function to extract repository name from URL
fn extract_repo_name_from_url(repo_url: &str) -> Result<String, String> {
    if repo_url.starts_with("https://github.com/") {
        repo_url.rsplit_once('/')
            .and_then(|(_, name_with_git)| name_with_git.strip_suffix(".git"))
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Could not extract repository name from HTTPS URL: {}", repo_url))
    } else if repo_url.starts_with("git@github.com:") {
        repo_url.rsplit_once('/')
            .and_then(|(_, name_with_git)| name_with_git.strip_suffix(".git"))
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Could not extract repository name from SSH URL: {}", repo_url))
    } else {
        Err(format!("Unsupported repository URL format: {}", repo_url))
    }
}

// Helper function to fork a repository using `gh repo fork`
fn fork_repository_with_gh(repo_path: &Path, target_org: &str, original_repo_name: &str, log_file: &mut File, dry_run: bool) -> Result<(), String> {
    writeln!(log_file, "Attempting to fork repository '{}' to organization '{}' using 'gh' CLI.", original_repo_name, target_org).map_err(|e| e.to_string())?;
    println!("Attempting to fork repository '{}' to organization '{}' using 'gh' CLI.", original_repo_name, target_org);

    if dry_run {
        writeln!(log_file, "[DRY RUN] Would execute: gh repo fork {} --org {} --remote --clone=false --repo-host github.com", original_repo_name, target_org).map_err(|e| e.to_string())?;
        println!("[DRY RUN] Would execute: gh repo fork {} --org {} --remote --clone=false --repo-host github.com", original_repo_name, target_org);
        return Ok(());
    }

    let output = Command::new("gh")
        .arg("repo")
        .arg("fork")
        .arg(original_repo_name)
        .arg("--org")
        .arg(target_org)
        .arg("--remote")
        .arg("--clone=false") // We've already cloned or will clone separately
        .arg("--repo-host") // Explicitly set repo host to avoid issues with gh config
        .arg("github.com")
        .current_dir(repo_path) // Run gh command in the context of the repository
        .output()
        .map_err(|e| format!("Failed to execute 'gh repo fork': {}", e))?;

    if output.status.success() {
        writeln!(log_file, "Successfully forked repository '{}' to organization '{}'.", original_repo_name, target_org).map_err(|e| e.to_string())?;
        println!("Successfully forked repository '{}' to organization '{}'.", original_repo_name, target_org);
    } else {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        writeln!(log_file, "Failed to fork repository '{}':\nStdout: {}\nStderr: {}", original_repo_name, stdout_str, stderr_str).map_err(|e| e.to_string())?;
        return Err(format!("Failed to fork repository '{}':\nStdout: {}\nStderr: {}", original_repo_name, stdout_str, stderr_str));
    }

    Ok(())
}

// Helper function to update Cargo.toml dependencies
fn update_cargo_toml_dependencies(
    root_dir: &Path,
    vendor_crates: &[CrateInfo],
    target_org: &str,
    target_branch: &str,
    log_file: &mut File,
    dry_run: bool,
) -> Result<(), String> {
    writeln!(log_file, "Updating Cargo.toml dependencies...").map_err(|e| e.to_string())?;
    println!("Updating Cargo.toml dependencies...");

    let mut crate_name_to_fork_url: HashMap<String, String> = HashMap::new();
    for crate_info in vendor_crates {
        if let Some(original_repo_url) = &crate_info.repository_url {
            let original_repo_name = extract_repo_name_from_url(original_repo_url)?;
            let fork_url = if original_repo_url.starts_with("https://") {
                format!("https://github.com/{}/{}.git", target_org, original_repo_name)
            } else if original_repo_url.starts_with("git@") {
                format!("git@github.com:{}/{}.git", target_org, original_repo_name)
            } else {
                return Err(format!("Unsupported repository URL scheme: {}", original_repo_url));
            };
            crate_name_to_fork_url.insert(crate_info.name.clone(), fork_url);
        }
    }

    // Find all Cargo.toml files in the project
    for entry in walkdir::WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        writeln!(log_file, "  Processing Cargo.toml: {:?}", cargo_toml_path).map_err(|e| e.to_string())?;

        let content = fs::read_to_string(&cargo_toml_path)
            .map_err(|e| format!("Failed to read Cargo.toml at {:?}: {}", cargo_toml_path, e))?;
        let doc = content.parse::<toml_edit::Document<String>>()
            .map_err(|e| format!("Failed to parse Cargo.toml at {:?}: {}", cargo_toml_path, e))?;

        let mut changed = false;

        // Extract the root table from the document
        let mut root_table = doc.into_table();

        // Helper to process a dependency table
        let mut process_deps_table = |table_name: &str| -> Result<(), String> {
            if let Some(deps_table) = root_table.get_mut(table_name).and_then(|item| item.as_table_mut()) {
                for (dep_name, dep_value) in deps_table.iter_mut() {
                    if let Some(fork_url) = crate_name_to_fork_url.get(&dep_name.to_string()) {
                        // Check if it's already a git dependency pointing to our fork and branch
                        let is_already_correct_git_dep = dep_value.as_table()
                            .and_then(|table| table.get("git").and_then(|g| g.as_str()))
                            .map_or(false, |git_url| git_url == fork_url)
                            && dep_value.as_table()
                                .and_then(|table| table.get("branch").and_then(|b| b.as_str()))
                                .map_or(false, |branch| branch == target_branch);

                        if !is_already_correct_git_dep {
                            writeln!(log_file, "    Updating dependency '{}' in {:?}.", dep_name, cargo_toml_path).map_err(|e| e.to_string())?;
                            println!("    Updating dependency '{}' in {:?}.", dep_name, cargo_toml_path);

                            let mut new_dep_table = toml_edit::Table::new();
                            new_dep_table.insert("git", toml_edit::value(fork_url.clone()));
                            new_dep_table.insert("branch", toml_edit::value(target_branch.to_string()));
                            *dep_value = toml_edit::Item::Table(new_dep_table);
                            changed = true;
                        } else {
                            writeln!(log_file, "    Dependency '{}' in {:?} is already correctly set to fork. Skipping.", dep_name, cargo_toml_path).map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
            Ok(())
        };

        process_deps_table("dependencies")?;
        process_deps_table("dev-dependencies")?;
        process_deps_table("build-dependencies")?;

        if changed {
            if dry_run {
                writeln!(log_file, "[DRY RUN] Would write updated Cargo.toml to {:?}", cargo_toml_path).map_err(|e| e.to_string())?;
                println!("[DRY RUN] Would write updated Cargo.toml to {:?}", cargo_toml_path);
                writeln!(log_file, "--- Updated Cargo.toml (Dry Run) for {:?} ---", cargo_toml_path).map_err(|e| e.to_string())?;
                writeln!(log_file, "{}", root_table.to_string()).map_err(|e| e.to_string())?;
                writeln!(log_file, "----------------------------------------------------").map_err(|e| e.to_string())?;
            } else {
                fs::write(&cargo_toml_path, root_table.to_string())
                    .map_err(|e| format!("Failed to write updated Cargo.toml to {:?}: {}", cargo_toml_path, e))?;
                writeln!(log_file, "  Successfully updated Cargo.toml: {:?}", cargo_toml_path).map_err(|e| e.to_string())?;
                println!("  Successfully updated Cargo.toml: {:?}", cargo_toml_path);
            }
        } else {
            writeln!(log_file, "  No changes needed for Cargo.toml: {:?}", cargo_toml_path).map_err(|e| e.to_string())?;
        }
    }

    writeln!(log_file, "Finished updating Cargo.toml dependencies.").map_err(|e| e.to_string())?;
    Ok(())
}

// Helper function to commit and push changes in a single submodule
pub fn commit_and_push_submodule(
    submodule_path: &Path,
    commit_message: &str,
    log_file: &mut File,
    dry_run: bool,
) -> Result<(), String> {
    writeln!(log_file, "--- Processing submodule: {:?} ---", submodule_path).map_err(|e| e.to_string())?;
    println!("--- Processing submodule: {:?} ---", submodule_path);

    let repo = git2::Repository::open(submodule_path)
        .map_err(|e| format!("Failed to open submodule repository at {:?}: {}", submodule_path, e))?;

    // Check for local changes
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))
        .map_err(|e| format!("Failed to get submodule status: {}", e))?;

    if statuses.is_empty() {
        writeln!(log_file, "No local changes in submodule {:?}. Skipping.", submodule_path).map_err(|e| e.to_string())?;
        println!("No local changes in submodule {:?}. Skipping.", submodule_path);
        return Ok(())
    }

    writeln!(log_file, "Local changes detected in submodule {:?}.", submodule_path).map_err(|e| e.to_string())?;
    println!("Local changes detected in submodule {:?}.", submodule_path);

    if dry_run {
        writeln!(log_file, "[DRY RUN] Would stage all changes in {:?}.", submodule_path).map_err(|e| e.to_string())?;
        writeln!(log_file, "[DRY RUN] Would commit with message: '{}' in {:?}.", commit_message, submodule_path).map_err(|e| e.to_string())?;
        writeln!(log_file, "[DRY RUN] Would push changes to remote in {:?}.", submodule_path).map_err(|e| e.to_string())?;
        println!("[DRY RUN] Would stage all changes in {:?}.", submodule_path);
        println!("[DRY RUN] Would commit with message: '{}' in {:?}.", commit_message, submodule_path);
        println!("[DRY RUN] Would push changes to remote in {:?}.", submodule_path);
        return Ok(())
    }

    // Stage all changes
    let mut index = repo.index()
        .map_err(|e| format!("Failed to get submodule index: {}", e))?;
    index.add_all([""].iter(), git2::IndexAddOption::all(), None)
        .map_err(|e| format!("Failed to stage changes in submodule: {}", e))?;
    index.write()
        .map_err(|e| format!("Failed to write submodule index: {}", e))?;
    writeln!(log_file, "Staged all changes in submodule {:?}.", submodule_path).map_err(|e| e.to_string())?;
    println!("Staged all changes in submodule {:?}.", submodule_path);

    // Commit changes
    let tree_id = index.write_tree()
        .map_err(|e| format!("Failed to write submodule tree: {}", e))?;
    let tree = repo.find_tree(tree_id)
        .map_err(|e| format!("Failed to find submodule tree: {}", e))?;
    let signature = repo.signature()
        .map_err(|e| format!("Failed to get submodule signature: {}", e))?;
    let parent_commit = repo.head()
        .and_then(|head| head.peel_to_commit())
        .map_err(|e| format!("Failed to get submodule HEAD commit: {}", e))?;

    repo.commit(
        Some("HEAD"), // Update HEAD
        &signature,
        &signature,
        commit_message,
        &tree,
        &[&parent_commit],
    )
    .map_err(|e| format!("Failed to commit changes in submodule: {}", e))?;
    writeln!(log_file, "Committed changes in submodule {:?} with message: '{}'.", submodule_path, commit_message).map_err(|e| e.to_string())?;
    println!("Committed changes in submodule {:?} with message: '{}'.", submodule_path, commit_message);

    // Verify remote origin and branch (simplified check)
    let remote = repo.find_remote("origin")
        .map_err(|e| format!("Failed to find 'origin' remote in submodule {:?}: {}", submodule_path, e))?;
    let remote_url = remote.url().unwrap_or("unknown");
    writeln!(log_file, "Submodule {:?} remote origin URL: {}", submodule_path, remote_url).map_err(|e| e.to_string())?;
    println!("Submodule {:?} remote origin URL: {}", submodule_path, remote_url);

    // Basic check for meta-introspector org (can be made more robust)
    if !remote_url.contains("meta-introspector") {
        writeln!(log_file, "[WARNING] Submodule {:?} remote URL does not contain 'meta-introspector'.", submodule_path).map_err(|e| e.to_string())?;
        println!("[WARNING] Submodule {:?} remote URL does not contain 'meta-introspector'.", submodule_path);
    }

    let head_ref = repo.head()
        .map_err(|e| format!("Failed to get HEAD reference in submodule {:?}: {}", submodule_path, e))?;
    let current_branch = head_ref.shorthand().unwrap_or("unknown");
    writeln!(log_file, "Submodule {:?} current branch: {}", submodule_path, current_branch).map_err(|e| e.to_string())?;
    println!("Submodule {:?} current branch: {}", submodule_path, current_branch);

    // Basic check for feature/CRQ-016-nixify branch (can be made more robust)
    if current_branch != "feature/CRQ-016-nixify" {
        writeln!(log_file, "[WARNING] Submodule {:?} is not on 'feature/CRQ-016-nixify' branch.", submodule_path).map_err(|e| e.to_string())?;
        println!("[WARNING] Submodule {:?} is not on 'feature/CRQ-016-nixify' branch.", submodule_path);
    }

    // Push changes
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        // You might need to implement a more robust credential helper here
        // For now, assuming credentials are handled by git config or agent
        Err(git2::Error::from_str("No credential callback configured"))
    });

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")
        .map_err(|e| format!("Failed to find 'origin' remote for push in submodule {:?}: {}", submodule_path, e))?;
    
    let refspec = format!("HEAD:refs/heads/{}", current_branch);
    remote.push(&[refspec], Some(&mut push_options))
        .map_err(|e| format!("Failed to push changes in submodule {:?}: {}", submodule_path, e))?;
    writeln!(log_file, "Pushed changes in submodule {:?}.", submodule_path).map_err(|e| e.to_string())?;
    println!("Pushed changes in submodule {:?}.", submodule_path);

    Ok(())
}

// Function to generate [patch.crates-io] entries for submodules
pub fn generate_submodule_patches(
    current_dir: &Path,
    output_file: &str, // This will now be the base name for generated config files
    dry_run: bool,
    overwrite: bool,
    _git_url_template: Option<&String>, // Not used in this iteration
    _branch_template: Option<&String>, // Not used in this iteration
    recursive: bool,
) -> Result<String, String> {
    let repo = git2::Repository::open(current_dir).map_err(|e| e.to_string())?;

    let mut generated_files_info = Vec::new();

    let mut submodule_name_to_path: HashMap<String, PathBuf> = HashMap::new();
    let git_ops = RealGitRepositoryOperations;
    for sm_info in git_ops.submodules(current_dir).map_err(|e| e.to_string())? {
        submodule_name_to_path.insert(sm_info.name.clone(), current_dir.join(&sm_info.path));
    }

    for submodule_info in git_ops.submodules(current_dir).map_err(|e| e.to_string())?
    {
        let submodule_path = current_dir.join(&submodule_info.path);
        let submodule_name = &submodule_info.name;

        println!("Processing submodule: {}", submodule_name);

        let submodule_cargo_dir = submodule_path.join(".cargo");
        let submodule_config_path = submodule_cargo_dir.join(output_file); // output_file is now the config filename

        // Determine package name (used for logging/fallback)
        let package_name = {
            let cargo_toml_path = submodule_path.join("Cargo.toml");
            if cargo_toml_path.exists() {
                let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
                    .map_err(|e| format!("Failed to read Cargo.toml for submodule {:?}: {}", submodule_path, e))?;
                let cargo_toml_doc = cargo_toml_content.parse::<toml_edit::Document<String>>()
                    .map_err(|e| format!("Failed to parse Cargo.toml for submodule {:?}: {}", submodule_path, e))?;

                cargo_toml_doc.get("package")
                    .and_then(|item| item.as_table())
                    .and_then(|table| table.get("name"))
                    .and_then(|item| item.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| submodule_name.to_string()) // Fallback to submodule name
            } else {
                submodule_name.to_string() // Fallback if Cargo.toml doesn't exist
            }
        };
        println!("  Submodule package name: {}", package_name);


        if !submodule_cargo_dir.exists() {
            if dry_run {
                println!("[DRY RUN] Would create directory: {:?}", submodule_cargo_dir);
            } else {
                fs::create_dir_all(&submodule_cargo_dir)
                    .map_err(|e| format!("Failed to create .cargo directory for submodule {:?}: {}", submodule_path, e))?;
                println!("  Created directory: {:?}", submodule_cargo_dir);
            }
        }

        let mut config_content = String::new();
        config_content.push_str("[source.crates-io]\n");
        config_content.push_str("replace-with = \"vendored-sources\"\n\n");
        config_content.push_str("[source.vendored-sources]\n");
        config_content.push_str("directory = \"vendor\"\n");

        // Collect patch entries for this submodule
        let mut patch_entries = String::new();
        patch_entries.push_str("[patch.crates-io]\n");

        // Read Cargo.toml of the submodule to get its dependencies
        let cargo_toml_path = submodule_path.join("Cargo.toml");
        if cargo_toml_path.exists() {
            let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
                .map_err(|e| format!("Failed to read Cargo.toml for submodule {:?}: {}", submodule_path, e))?;
            let cargo_toml_doc = cargo_toml_content.parse::<toml_edit::Document<String>>()
                .map_err(|e| format!("Failed to parse Cargo.toml for submodule {:?}: {}", submodule_path, e))?;

            // Iterate over dependencies and add patch entries if they are also submodules
            if let Some(dependencies) = cargo_toml_doc.get("dependencies").and_then(|item| item.as_table()) {
                for (dep_name, _dep_value) in dependencies.iter() {
                    if let Some(matching_submodule_abs_path) = submodule_name_to_path.get(dep_name) {
                        let relative_path = pathdiff::diff_paths(matching_submodule_abs_path, &submodule_path)
                            .ok_or_else(|| format!("Failed to get relative path from {:?} to {:?}", submodule_path, matching_submodule_abs_path))?
                            .to_string_lossy()
                            .to_string();
                        patch_entries.push_str(&format!("{} = {{ path = \"{}\" }}\n", dep_name, relative_path));
                        println!("  Added patch for dependency '{}' -> '{}'", dep_name, relative_path);
                    }
                }
            }
        }
        // Append patch_entries to config_content (this was the missing part)
        // Append patch_entries to config_content (this was the missing part)
        config_content.push_str(&patch_entries);

        if dry_run {
            println!("\n--- Generated .cargo/config.toml for {} (Dry Run) ---", submodule_name);
            println!("{}", config_content);
            println!("----------------------------------------------------\n");
            generated_files_info.push(format!("[DRY RUN] Would generate config for {} at {:?}", submodule_name, submodule_config_path));
        } else {
            if submodule_config_path.exists() && !overwrite {
                println!("  Config file {:?} already exists. Skipping (use --overwrite to force).", submodule_config_path);
                generated_files_info.push(format!("Skipped config for {} at {:?}", submodule_name, submodule_config_path));
            } else {
                fs::write(&submodule_config_path, config_content)
                    .map_err(|e| format!("Failed to write config file for submodule {:?}: {}", submodule_path, e))?;
                println!("  Generated config file: {:?}", submodule_config_path);
                generated_files_info.push(format!("Generated config for {} at {:?}", submodule_name, submodule_config_path));
            }
        }

        // Handle recursive submodules if enabled
        if recursive {
            // TODO: Implement recursive submodule processing
            println!("  Recursive processing for nested submodules is not yet implemented.");
        }
    } // This brace closes the 'for submodule_info in ...' loop

    // After generating all submodule configs, update the main Cargo.toml metadata
    update_main_cargo_toml_metadata(current_dir, &generated_files_info, dry_run)?;

    Ok(format!("Submodule config generation completed.\n{}", generated_files_info.join("\n")))
}

// Helper function to update the main Cargo.toml metadata
fn update_main_cargo_toml_metadata(
    current_dir: &Path,
    generated_configs: &[String],
    dry_run: bool,
) -> Result<(), String> {
    let main_cargo_toml_path = current_dir.join("Cargo.toml");
    let main_cargo_toml_content = fs::read_to_string(&main_cargo_toml_path)
        .map_err(|e| format!("Failed to read main Cargo.toml: {}", e))?;
    let main_cargo_toml_doc = main_cargo_toml_content.parse::<toml_edit::Document<String>>()
        .map_err(|e| format!("Failed to parse main Cargo.toml: {}", e))?;

    // Extract the root table
    let mut root_table = main_cargo_toml_doc.into_table(); // This consumes the Document

    // Ensure 'package' table exists and get a mutable reference to it
    let package_table = root_table
        .entry("package")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()))
        .as_table_mut()
        .expect("package should be a table");

    // Ensure 'metadata' table exists within 'package' and get a mutable reference
    let metadata_table = package_table
        .entry("metadata")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()))
        .as_table_mut()
        .expect("metadata should be a table");

    // Ensure 'cargo-git-manage' table exists within 'metadata' and get a mutable reference
    let cargo_git_manage_table = metadata_table
        .entry("cargo-repo-sync")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()))
        .as_table_mut()
        .expect("cargo-git-manage metadata should be a table");

    let config_paths_value = toml_edit::value(toml_edit::Array::from_iter(
        generated_configs.iter().map(|s| toml_edit::Value::from(s.as_str()))
    ));
    cargo_git_manage_table.insert("generated-submodule-configs", config_paths_value);

    // Reconstruct the Document from the modified table
    let updated_toml = root_table.to_string();

    if dry_run {
        println!("\n--- Updated Main Cargo.toml Metadata (Dry Run) ---");
        println!("{}", updated_toml);
        println!("----------------------------------------------------");
    } else {
        fs::write(&main_cargo_toml_path, updated_toml)
            .map_err(|e| format!("Failed to write to main Cargo.toml: {}", e))?;
    }
    Ok(())
}

pub fn fork_and_patch_submodules(
    root_dir: &Path,
    target_org: &str,
    target_branch: &str,
    log_file: &mut File,
    dry_run: bool,
) -> Result<(), String> {
    writeln!(log_file, "--- Starting Fork and Patch Process ---").map_err(|e| e.to_string())?;
    writeln!(log_file, "Root Directory: {:?}", root_dir).map_err(|e| e.to_string())?;
    writeln!(log_file, "Target Organization: {}", target_org).map_err(|e| e.to_string())?;
    writeln!(log_file, "Target Branch: {}", target_branch).map_err(|e| e.to_string())?;
    writeln!(log_file, "Dry Run: {}", dry_run).map_err(|e| e.to_string())?;

    let vendor_dir = root_dir.join("vendor");
    if !vendor_dir.exists() {
        return Err(format!("Vendor directory not found at {:?}", vendor_dir));
    }

    let mut vendor_crates: Vec<CrateInfo> = Vec::new();

    writeln!(log_file, "Scanning vendor directory: {:?}", vendor_dir).map_err(|e| e.to_string())?;

    for entry in walkdir::WalkDir::new(&vendor_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        let crate_path = cargo_toml_path.parent().unwrap().to_path_buf(); // Path to the crate directory

        match fs::read_to_string(&cargo_toml_path) {
            Ok(content) => {
                match content.parse::<toml_edit::Document<String>>() {
                    Ok(doc) => {
                        let name = doc.get("package")
                            .and_then(|item| item.as_table())
                            .and_then(|table| table.get("name"))
                            .and_then(|item| item.as_str())
                            .map(|s| s.to_string());

                        let repository_url = doc.get("package")
                            .and_then(|item| item.as_table())
                            .and_then(|table| table.get("repository"))
                            .and_then(|item| item.as_str())
                            .map(|s| s.to_string());

                        if let Some(name) = name {
                            writeln!(log_file, "Found crate: {} at {:?}, Repository: {:?}", name, crate_path, repository_url).map_err(|e| e.to_string())?;
                            vendor_crates.push(CrateInfo {
                                name,
                                path: crate_path,
                                repository_url,
                            });
                        } else {
                            writeln!(log_file, "Skipping Cargo.toml at {:?} due to missing package name.", cargo_toml_path).map_err(|e| e.to_string())?;
                        }
                    },
                    Err(e) => {
                        writeln!(log_file, "Failed to parse Cargo.toml at {:?}: {}", cargo_toml_path, e).map_err(|e| e.to_string())?;
                    }
                }
            },
            Err(e) => {
                writeln!(log_file, "Failed to read Cargo.toml at {:?}: {}", cargo_toml_path, e).map_err(|e| e.to_string())?;
            }
        }
    }

    writeln!(log_file, "Found {} vendor crates.", vendor_crates.len()).map_err(|e| e.to_string())?;

    for crate_info in &vendor_crates {
        writeln!(log_file, "Processing crate: {}", crate_info.name).map_err(|e| e.to_string())?;

        if let Some(repo_url) = &crate_info.repository_url {
            let submodule_target_path = root_dir.join("submodules").join(&crate_info.name);

            match is_submodule(root_dir, &submodule_target_path) {
                Ok(true) => {
                    writeln!(log_file, "  Crate {} is already a submodule at {:?}. Managing remotes.", crate_info.name, submodule_target_path).map_err(|e| e.to_string())?;
                    manage_remotes(&submodule_target_path, repo_url, target_org, log_file, dry_run)?;
                    let original_repo_name = extract_repo_name_from_url(repo_url)?;
                    fork_repository_with_gh(&submodule_target_path, target_org, &original_repo_name, log_file, dry_run)?;
                    checkout_branch(&submodule_target_path, target_branch, log_file, dry_run)?;
                },
                Ok(false) => {
                    writeln!(log_file, "  Crate {} is not a submodule. Needs cloning from {}.", crate_info.name, repo_url).map_err(|e| e.to_string())?;
                    clone_repository(repo_url, &submodule_target_path, log_file, dry_run)?;
                    manage_remotes(&submodule_target_path, repo_url, target_org, log_file, dry_run)?;
                    let original_repo_name = extract_repo_name_from_url(repo_url)?;
                    fork_repository_with_gh(&submodule_target_path, target_org, &original_repo_name, log_file, dry_run)?;
                    checkout_branch(&submodule_target_path, target_branch, log_file, dry_run)?;
                },
                Err(e) => {
                    writeln!(log_file, "  Error checking submodule status for {}: {}", crate_info.name, e).map_err(|e| e.to_string())?;
                    return Err(e);
                }
            }
        } else {
            writeln!(log_file, "  Crate {} has no repository URL. Skipping.", crate_info.name).map_err(|e| e.to_string())?;
        }
    }

    update_cargo_toml_dependencies(root_dir, &vendor_crates, target_org, target_branch, log_file, dry_run)?;

    writeln!(log_file, "--- Fork and Patch Process Completed ---").map_err(|e| e.to_string())?;
    Ok(())
}

