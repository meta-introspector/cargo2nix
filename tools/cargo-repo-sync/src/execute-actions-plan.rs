pub fn execute_actions_plan(
    actions_plan: &Vec<RepoAction>,
    args: &Args,
    git_executable_path: &Path,
    gh_executable_path: &Path,
) -> Result<()> {
    println!("Executing actions plan...");

    for action in actions_plan {
        println!("Processing repository: {}", action.repo_name);

        // 1. Fork the repository if it doesn't exist in target_org
        let forked_repo_url = format!("https://github.com/{}/{}.git", args.target_org, action.repo_name);
        let gh_repo_check_output = Command::new(gh_executable_path)
            .arg("repo")
            .arg("view")
            .arg(&forked_repo_url)
            .arg("--json")
            .arg("name")
            .output()
            .context(format!("Failed to check if {} exists in {}", action.repo_name, args.target_org))?;

        if !gh_repo_check_output.status.success() || String::from_utf8_lossy(&gh_repo_check_output.stdout).trim().is_empty() {
            println!("Forking {} to {}...", action.repo_name, args.target_org);
            let fork_output = Command::new(gh_executable_path)
                .arg("repo")
                .arg("fork")
                .arg(&action.repo_url)
                .arg("--org")
                .arg(&args.target_org)
                .arg("--remote") // Add remote to the forked repo
                .arg("--clone=false") // Don't clone immediately, we'll add as submodule
                .output()
                .context(format!("Failed to fork {} to {}", action.repo_name, args.target_org))?;

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
            println!("Repository {} already exists in {}. Skipping fork.", action.repo_name, args.target_org);
        }

        // 2. Add as git submodule
        if !action.submodule_path.exists() {
            println!("Adding {} as submodule...", action.repo_name);
            let add_submodule_output = Command::new(git_executable_path)
                .arg("submodule")
                .arg("add")
                .arg(&forked_repo_url)
                .arg(&action.submodule_path)
                .output()
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
            println!("Submodule {} already exists at {:?}. Skipping add.", action.repo_name, action.submodule_path);
        }

        // 3. Checkout target branch in submodule
        println!("Checking out branch '{}' in submodule {}...", action.target_branch, action.repo_name);
        let checkout_output = Command::new(git_executable_path)
            .arg("-C")
            .arg(&action.submodule_path)
            .arg("checkout")
            .arg(&action.target_branch)
            .output()
            .context(format!("Failed to checkout branch {} in submodule {}", action.target_branch, action.repo_name))?;

        if !checkout_output.status.success() {
            eprintln!(
                "Failed to checkout branch '{}' in submodule {}: {}",
                action.target_branch,
                action.repo_name,
                String::from_utf8_lossy(&checkout_output.stderr)
            );
            anyhow::bail!("Branch checkout failed for {}", action.repo_name);
        }
        println!("Successfully checked out branch '{}' in submodule {}.", action.target_branch, action.repo_name);
    }

    update_cargo_config(&actions_plan, &args.root_dir)?;

    println!("Successfully executed all actions.");
    Ok(())
}
