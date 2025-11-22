use std::{
    fs::File,
    path::Path,
    process::Output,
    sync::Arc,
};

use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;

// Define a trait for cargo commands
pub trait CargoCommand {
    fn needs_execution(&self, current_dir: &Path, executor: Arc<dyn Execv + Send + Sync>) -> Result<bool, String>;
    fn execute(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<Output, String>;
    fn dry_run(&self, current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<(), String>; // New method
}

// Helper function to map command strings to CargoCommand trait objects
pub fn get_cargo_command(command_str: &str) -> Option<Box<dyn CargoCommand>> {
    match command_str {
        "cargo update" => Some(Box::new(super::cargo_update_command::CargoUpdateCommand)),
        "cargo vendor" => Some(Box::new(super::cargo_vendor_command::CargoVendorCommand)),
        "cargo2nix" => Some(Box::new(super::cargo2nix_command::Cargo2NixCommand)),
        "remove rust version constraints" => Some(Box::new(super::remove_rust_version_command::RemoveRustVersionCommand)), // Add this line
        _ => None, // Unknown command
    }
}
