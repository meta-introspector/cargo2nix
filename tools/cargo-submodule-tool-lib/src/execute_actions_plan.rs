use crate::update_cargo_config;
use crate::Args;
use crate::RepoAction;
use anyhow::Context;
use git_wrapper_lib::Execv; // Import Execv trait
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command; // Keep Command for now, as it's used in update_cargo_config
use std::sync::Arc;

pub fn execute_actions_plan(
    actions_plan: &Vec<RepoAction>,
    args: &Args,
    git_executable_path: &Path,
    gh_executable_path: &Path,
    executor: Arc<dyn Execv + Send + Sync>,
) -> Result<()> {
    println!("Executing actions plan...");

    for action in actions_plan {
        println!("Processing repository: {}", action.repo_name);

        // 1. Fork the repository if it doesn't exist in target_org
        let forked_repo_url = format!(
            "https://github.com/{}/{}.git",
            args.target_org, action.repo_name
        );
        let gh_repo_check_output = executor
            .execv(
                gh_executable_path.as_os_str(),
                &[
                    OsStr::new("repo"),
                    OsStr::new("view"),
                    OsStr::new(&forked_repo_url),
                    OsStr::new("--json"),
                    OsStr::new("name"),
                ],
                None,
            )
            .context(format!(
                "Failed to check if {} exists in {}",
                action.repo_name, args.target_org
            ))?;

        if !gh_repo_check_output.status.success()
            || String::from_utf8_lossy(&gh_repo_check_output.stdout)
                .trim()
                .is_empty()
        {
            println!("Forking {} to {}...", action.repo_name, args.target_org);
            let fork_output = executor
                .execv(
                    gh_executable_path.as_os_str(),
                    &[
                        OsStr::new("repo"),
                        OsStr::new("fork"),
                        OsStr::new(&action.repo_url),
                        OsStr::new("--org"),
                        OsStr::new(&args.target_org),
                        OsStr::new("--remote"), // Add remote to the forked repo
                        OsStr::new("--clone=false"), // Don't clone immediately, we'll add as submodule
                    ],
                    None,
                )
                .context(format!(
                    "Failed to fork {} to {}",
                    action.repo_name, args.target_org
                ))?;

            if !fork_output.status.success() {
                eprintln!(
                    "Failed to fork {}: {}",
                    action.repo_name,
                    String::from_utf8_lossy(&fork_output.stderr)
                );
                anyhow::bail!("Forking failed for {}", action.repo_name);
            }
            println!("Successfully forked {}.", action.repo_name);
        } else {
            println!(
                "Repository {} already exists in {}. Skipping fork.",
                action.repo_name, args.target_org
            );
        }

        // 2. Add as git submodule
        if !action.submodule_path.exists() {
            println!("Adding {} as submodule...", action.repo_name);
            let add_submodule_output = executor
                .execv(
                    git_executable_path.as_os_str(),
                    &[
                        OsStr::new("submodule"),
                        OsStr::new("add"),
                        OsStr::new(&forked_repo_url),
                        action.submodule_path.as_os_str(),
                    ],
                    None,
                )
                .context(format!("Failed to add {} as submodule", action.repo_name))?;

            if !add_submodule_output.status.success() {
                eprintln!(
                    "Failed to add submodule {}: {}",
                    action.repo_name,
                    String::from_utf8_lossy(&add_submodule_output.stderr)
                );
                anyhow::bail!("Adding submodule failed for {}", action.repo_name);
            }
            println!("Successfully added {} as submodule.", action.repo_name);
        } else {
            println!(
                "Submodule {} already exists at {:?}. Skipping add.",
                action.repo_name, action.submodule_path
            );
        }

        // 3. Checkout target branch in submodule
        println!(
            "Checking out branch '{}' in submodule {}...",
            action.target_branch, action.repo_name
        );
        let checkout_output = executor
            .execv(
                git_executable_path.as_os_str(),
                &[
                    OsStr::new("-C"),
                    action.submodule_path.as_os_str(),
                    OsStr::new("checkout"),
                    OsStr::new(&action.target_branch),
                ],
                None,
            )
            .context(format!(
                "Failed to checkout branch {} in submodule {}",
                action.target_branch, action.repo_name
            ))?;

        if !checkout_output.status.success() {
            eprintln!(
                "Failed to checkout branch '{}' in submodule {}: {}",
                action.target_branch,
                action.repo_name,
                String::from_utf8_lossy(&checkout_output.stderr)
            );
            anyhow::bail!("Branch checkout failed for {}", action.repo_name);
        }
        println!(
            "Successfully checked out branch '{}' in submodule {}.",
            action.target_branch, action.repo_name
        );
    }

    update_cargo_config(&actions_plan, &args.root_dir)?;

    println!("Successfully executed all actions.");
    Ok(())
}
