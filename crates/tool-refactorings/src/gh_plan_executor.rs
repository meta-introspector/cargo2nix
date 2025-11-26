// crates/tool-refactorings/src/gh_plan_executor.rs

use gemini_utils::gemini_eprintln;
use std::path::PathBuf;

// Define a struct to represent an action from the JSON plan
#[derive(Debug, PartialEq)]
pub struct GhPlanAction {
    pub repo_url: String,
    pub owner: String,
    pub repo_name: String,
    pub submodule_path: PathBuf,
    pub target_org: String,
    pub target_branch: String,
}

/// Trait for executing GitHub and Git operations based on a plan.
pub trait GhPlanExecutor {
    /// Executes a series of Git and GitHub CLI commands based on the provided plan actions.
    ///
    /// `plan_actions`: A vector of actions to be executed.
    /// `dry_run`: If true, commands are only printed and not executed.
    ///
    /// Returns a vector of messages detailing the execution or simulation.
    fn execute_plan(&self, plan_actions: &[GhPlanAction], dry_run: bool) -> Vec<String>;
}

/// Dummy implementation of `GhPlanExecutor` for testing and simulation.
pub struct DefaultGhPlanExecutor;

impl DefaultGhPlanExecutor {
    // Helper to simulate git clone
    fn mock_git_clone(&self, repo_url: &str, submodule_path: &PathBuf, dry_run: bool) -> String {
        let cmd = format!("git clone \"{}\" \"{}\"", repo_url, submodule_path.display());
        if dry_run { format!("[DRY RUN] {}", cmd) } else { format!("Executing: {}", cmd) }
    }

    // Helper to simulate git remote operations
    fn mock_git_remote(&self, submodule_path: &PathBuf, operation: &str, dry_run: bool) -> String {
        let cmd = format!("git -C \"{}\" remote {}", submodule_path.display(), operation);
        if dry_run { format!("[DRY RUN] {}", cmd) } else { format!("Executing: {}", cmd) }
    }

    // Helper to simulate gh repo fork
    fn mock_gh_fork(&self, owner: &str, repo_name: &str, target_org: &str, dry_run: bool) -> String {
        let cmd = format!("gh repo fork \"{}/{}\" --org \"{}\" --remote --fork-name \"{}\"", owner, repo_name, target_org, repo_name);
        if dry_run { format!("[DRY RUN] {}", cmd) } else { format!("Skipping execution of: {} (manual review and execution required)", cmd) }
    }

    // Helper to simulate git checkout
    fn mock_git_checkout(&self, submodule_path: &PathBuf, target_branch: &str, dry_run: bool) -> String {
        let cmd = format!("git -C \"{}\" checkout \"{}\"", submodule_path.display(), target_branch);
        if dry_run { format!("[DRY RUN] {}", cmd) } else { format!("Executing: {}", cmd) }
    }
}

impl GhPlanExecutor for DefaultGhPlanExecutor {
    fn execute_plan(&self, plan_actions: &[GhPlanAction], dry_run: bool) -> Vec<String> {
        let mut messages = Vec::new();

        if dry_run {
            messages.push("--- DRY RUN MODE ACTIVE (Rust Impl) ---".to_string());
            messages.push("Commands will be printed but NOT executed.".to_string());
            gemini_eprintln!("Executing GitHub plan. Dry run: true");
        } else {
            gemini_eprintln!("Executing GitHub plan. Dry run: false");
        }

        for action in plan_actions {
            messages.push(format!("--- Processing {} ---", action.repo_url));

            // 1. Clone repository
            // In real impl, check if dir exists
            messages.push(self.mock_git_clone(&action.repo_url, &action.submodule_path, dry_run));

            // 2. Manage remotes
            messages.push(self.mock_git_remote(&action.submodule_path, "rename origin upstream", dry_run));
            messages.push(self.mock_git_remote(&action.submodule_path, &format!("add origin https://github.com/{}/{}.git", action.target_org, action.repo_name), dry_run));

            // 3. Fork repository
            messages.push(self.mock_gh_fork(&action.owner, &action.repo_name, &action.target_org, dry_run));

            // 4. Checkout target branch
            messages.push(self.mock_git_checkout(&action.submodule_path, &action.target_branch, dry_run));
            
            messages.push(format!("--- Finished processing {} ---", action.repo_url));
            messages.push("".to_string()); // Empty line for separation
        }
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_plan_actions() -> Vec<GhPlanAction> {
        vec![
            GhPlanAction {
                repo_url: "https://github.com/owner1/repo1.git".to_string(),
                owner: "owner1".to_string(),
                repo_name: "repo1".to_string(),
                submodule_path: PathBuf::from("./submodules/repo1"),
                target_org: "meta-introspector".to_string(),
                target_branch: "feature/CRQ-016-nixify".to_string(),
            },
            GhPlanAction {
                repo_url: "https://github.com/owner2/repo2.git".to_string(),
                owner: "owner2".to_string(),
                repo_name: "repo2".to_string(),
                submodule_path: PathBuf::from("./submodules/repo2"),
                target_org: "meta-introspector".to_string(),
                target_branch: "main".to_string(),
            },
        ]
    }

    #[test]
    fn test_execute_plan_dry_run() {
        let executor = DefaultGhPlanExecutor;
        let plan_actions = create_mock_plan_actions();
        let messages = executor.execute_plan(&plan_actions, true);

        assert!(messages[0].contains("DRY RUN MODE ACTIVE"));
        assert!(messages[2].contains("--- Processing https://github.com/owner1/repo1.git ---"));
        assert!(messages[3].contains("[DRY RUN] git clone"));
        assert!(messages[4].contains("[DRY RUN] git -C \"./submodules/repo1\" remote rename origin upstream"));
        assert!(messages[5].contains("[DRY RUN] git -C \"./submodules/repo1\" remote add origin https://github.com/meta-introspector/repo1.git"));
        assert!(messages[6].contains("[DRY RUN] gh repo fork"));
        assert!(messages[7].contains("[DRY RUN] git -C \"./submodules/repo1\" checkout"));
    }

    #[test]
    fn test_execute_plan_real_run() {
        let executor = DefaultGhPlanExecutor;
        let plan_actions = create_mock_plan_actions();
        let messages = executor.execute_plan(&plan_actions, false);

        assert!(!messages[0].contains("DRY RUN MODE ACTIVE")); // Check that dry run message is not present (this should be the first message now)
        assert!(messages[0].contains("--- Processing https://github.com/owner1/repo1.git ---"));
        assert!(messages[1].contains("Executing: git clone"));
        assert!(messages[2].contains("Executing: git -C \"./submodules/repo1\" remote rename origin upstream"));
        assert!(messages[3].contains("Executing: git -C \"./submodules/repo1\" remote add origin https://github.com/meta-introspector/repo1.git"));
        assert!(messages[4].contains("Skipping execution of: gh repo fork")); // gh fork is explicitly skipped in shell script too
        assert!(messages[5].contains("Executing: git -C \"./submodules/repo1\" checkout"));
    }

    #[test]
    fn test_plan_action_structure() {
        let plan = create_mock_plan_actions();
        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].repo_name, "repo1");
        assert_eq!(plan[1].target_branch, "main");
    }
}
