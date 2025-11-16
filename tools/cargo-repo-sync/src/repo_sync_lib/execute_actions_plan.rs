use anyhow::{Result, Context};
use std::path::Path;

use crate::RepoSyncConfig;
use crate::repo_sync_lib::repo_action::RepoAction;
use crate::executors::{GitExecutor, SystemGhExecutor, GhExecutor};
use crate::fs_writer::FileSystemWriter;
use crate::update_cargo_config;

pub fn execute_actions_plan(
    actions_plan: &Vec<RepoAction>,
    root_dir: &Path,
    target_org: &str,
    target_branch: &str,
    git_executor: &Box<dyn GitExecutor>,
    gh_executor: &SystemGhExecutor,
    file_system_writer: &dyn FileSystemWriter,
) -> Result<()> {
    println!("Executing actions plan...");

    for action in actions_plan {
        println!("Processing repository: {}", action.repo_name);

        // 1. Fork the repository if it doesn't exist in target_org
        let forked_repo_url = format!("https://github.com/{}/{}.git", target_org, action.repo_name);
        if !gh_executor.repo_view(&forked_repo_url)? {
            println!("Forking {} to {}...", action.repo_name, target_org);
            gh_executor.repo_fork(&action.repo_url, target_org)?;
            println!("Successfully forked {}.", action.repo_name);
        } else {
            println!("Repository {} already exists in {}. Skipping fork.", action.repo_name, target_org);
        }

        // 2. Add as git submodule
        if !action.submodule_path.exists() {
            println!("Adding {} as submodule...", action.repo_name);
            git_executor.submodule_add(&forked_repo_url, &action.submodule_path)?;
            println!("Successfully added {} as submodule.", action.repo_name);
        } else {
            println!("Submodule {} already exists at {:?}. Skipping add.", action.repo_name, action.submodule_path);
        }

        // 3. Checkout target branch in submodule
        println!("Checking out branch '{}' in submodule {}...", target_branch, action.repo_name);
        git_executor.checkout_branch(&action.submodule_path, target_branch)?;
        println!("Successfully checked out branch '{}' in submodule {}.", target_branch, action.repo_name);
    }

    update_cargo_config(&actions_plan, root_dir, file_system_writer)?;

    println!("Successfully executed all actions.");
    Ok(())
}
