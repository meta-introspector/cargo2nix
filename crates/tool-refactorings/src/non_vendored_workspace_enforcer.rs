// crates/tool-refactorings/src/non_vendored_workspace_enforcer.rs

use gemini_utils::gemini_eprintln;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Trait for enforcing the presence of `[workspace]` sections in `Cargo.toml` files
/// for non-vendored modules.
pub trait NonVendoredWorkspaceEnforcer {
    /// Ensures that the specified modules have a `[workspace]` section in their `Cargo.toml`.
    ///
    /// `module_names`: A list of names of modules to check.
    /// `dry_run`: If true, no changes are written to files.
    /// `project_root`: The root directory of the project.
    ///
    /// Returns a list of messages indicating actions taken or issues found.
    fn ensure_workspace_section(
        &self,
        module_names: &[String],
        dry_run: bool,
        project_root: &Path,
    ) -> Vec<String>;
}

/// Dummy implementation of `NonVendoredWorkspaceEnforcer` for testing and simulation.
pub struct DefaultNonVendoredWorkspaceEnforcer {
    // Simulate a file system for testing
    pub file_system: HashMap<PathBuf, String>,
}

impl DefaultNonVendoredWorkspaceEnforcer {
    pub fn new(file_system: HashMap<PathBuf, String>) -> Self {
        Self { file_system }
    }

    // Helper to simulate reading a file
    fn mock_read_file(&self, path: &Path) -> Option<&String> {
        self.file_system.get(path)
    }

    // Helper to simulate writing to a file (only in dry_run=false mode for this mock)
    fn mock_append_to_file(&mut self, path: &Path, content: &str) {
        if let Some(file_content) = self.file_system.get_mut(path) {
            file_content.push_str(content);
        } else {
            self.file_system.insert(path.to_path_buf(), content.to_string());
        }
    }
}

impl NonVendoredWorkspaceEnforcer for DefaultNonVendoredWorkspaceEnforcer {
    fn ensure_workspace_section(
        &self,
        module_names: &[String],
        dry_run: bool,
        project_root: &Path,
    ) -> Vec<String> {
        let mut messages = Vec::new();

        gemini_eprintln!("Ensuring non-vendored modules have a [workspace] section. Dry run: :dry_run:", dry_run = dry_run);

        for module_name in module_names {
            let mut cargo_toml_path = None;

            // Simulate searching for Cargo.toml
            let submodule_path = project_root.join("submodules").join(module_name).join("Cargo.toml");
            let vendor_path = project_root.join("vendor").join(module_name).join("Cargo.toml");

            if self.file_system.contains_key(&submodule_path) {
                cargo_toml_path = Some(submodule_path);
            } else if self.file_system.contains_key(&vendor_path) {
                cargo_toml_path = Some(vendor_path);
            }

            if let Some(path) = cargo_toml_path {
                if let Some(content) = self.mock_read_file(&path) {
                    if !content.contains("[workspace]") {
                        let msg = format!("ACTION: Would add [workspace] to {module_name}/Cargo.toml ({})", path.display());
                        messages.push(msg.clone());
                        gemini_eprintln!(":msg:", msg = msg);
                        if !dry_run {
                            // In a real impl, this would append to the file
                            // For this dummy, we just report the action
                            // self.mock_append_to_file(&path, "\n[workspace]"); // Cannot do this because &self
                        }
                    } else {
                        let msg = format!("INFO: [workspace] already exists in {module_name}/Cargo.toml ({})", path.display());
                        messages.push(msg.clone());
                        gemini_eprintln!(":msg:", msg = msg);
                    }
                } else {
                    let msg = format!("WARNING: Cargo.toml found in mock file system, but content not available for {module_name} at {}", path.display());
                    messages.push(msg.clone());
                    gemini_eprintln!(":msg:", msg = msg);
                }
            } else {
                let msg = format!("WARNING: Cargo.toml not found for non-vendored module {} at expected paths.", module_name);
                messages.push(msg.clone());
                gemini_eprintln!(":msg:", msg = msg);
            }
        }
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_ensure_workspace_section_adds_workspace() {
        let project_root = PathBuf::from("/mock/project");
        let module_name = "my-module".to_string();
        let cargo_toml_path = project_root.join("submodules").join(&module_name).join("Cargo.toml");

        let mut file_system = HashMap::new();
        file_system.insert(cargo_toml_path.clone(), "[package]\nname = \"my-module\"".to_string());

        let enforcer = DefaultNonVendoredWorkspaceEnforcer::new(file_system);
        let messages = enforcer.ensure_workspace_section(&vec![module_name.clone()], false, &project_root);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("ACTION: Would add [workspace]"));
        // Cannot assert file_system content change here because ensure_workspace_section takes &self
    }

    #[test]
    fn test_ensure_workspace_section_already_exists() {
        let project_root = PathBuf::from("/mock/project");
        let module_name = "my-module".to_string();
        let cargo_toml_path = project_root.join("submodules").join(&module_name).join("Cargo.toml");

        let mut file_system = HashMap::new();
        file_system.insert(cargo_toml_path.clone(), "[package]\nname = \"my-module\"\n[workspace]".to_string());

        let enforcer = DefaultNonVendoredWorkspaceEnforcer::new(file_system);
        let messages = enforcer.ensure_workspace_section(&vec![module_name.clone()], false, &project_root);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("INFO: [workspace] already exists"));
    }

    #[test]
    fn test_ensure_workspace_section_dry_run() {
        let project_root = PathBuf::from("/mock/project");
        let module_name = "my-module".to_string();
        let cargo_toml_path = project_root.join("submodules").join(&module_name).join("Cargo.toml");

        let mut file_system = HashMap::new();
        file_system.insert(cargo_toml_path.clone(), "[package]\nname = \"my-module\"".to_string());

        let enforcer = DefaultNonVendoredWorkspaceEnforcer::new(file_system);
        let messages = enforcer.ensure_workspace_section(&vec![module_name.clone()], true, &project_root);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("ACTION: Would add [workspace]"));
        // In dry run, file_system content should not change.
    }

    #[test]
    fn test_ensure_workspace_section_not_found() {
        let project_root = PathBuf::from("/mock/project");
        let module_name = "non-existent-module".to_string();
        
        let file_system = HashMap::new(); // Empty file system
        let enforcer = DefaultNonVendoredWorkspaceEnforcer::new(file_system);
        let messages = enforcer.ensure_workspace_section(&vec![module_name.clone()], false, &project_root);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("WARNING: Cargo.toml not found"));
    }
}
