use anyhow::{Context, Result};
use std::path::Path;
use crate::executors::{GitExecutor, GhExecutor, SystemGitExecutor, SystemGhExecutor};
use crate::traits::execv::Execv;
use crate::RepoAction; // Assuming RepoAction is in main.rs or a common module
use crate::AddSubmodulesArgs; // Assuming AddSubmodulesArgs is in main.rs
use crate::update_cargo_config::update_cargo_config; // Import update_cargo_config

pub fn execute_actions_plan<E: Execv>(
    actions_plan: &Vec<RepoAction>,
    args: &AddSubmodulesArgs,
    git_executor: &SystemGitExecutor<E>,
    gh_executor: &SystemGhExecutor<E>,
) -> Result<()> {
    println!("Executing actions plan...");

    for action in actions_plan {
        println!("Processing repository: {}", action.repo_name);

        // 1. Fork the repository if it doesn't exist in target_org
        let forked_repo_url = format!("https://github.com/{}/{}.git", args.target_org, action.repo_name);
        if !gh_executor.repo_view(&forked_repo_url)? {
            println!("Forking {} to {}...", action.repo_name, args.target_org);
            gh_executor.repo_fork(&action.repo_url, &args.target_org)?;
            println!("Successfully forked {}.", action.repo_name);
        } else {
            println!("Repository {} already exists in {}. Skipping fork.", action.repo_name, args.target_org);
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
        println!("Checking out branch '{}' in submodule {}...", action.target_branch, action.repo_name);
        git_executor.checkout_branch(&action.submodule_path, &action.target_branch)?;
        println!("Successfully checked out branch '{}' in submodule {}.", action.target_branch, action.repo_name);
    }

    update_cargo_config(&actions_plan, &args.root_dir)?;

    println!("Successfully executed all actions.");
    Ok(())
}
