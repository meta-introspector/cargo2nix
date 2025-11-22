use std::{
    fs::File,
    io::Write,
    path::Path,
    ffi::OsStr,
    sync::Arc,
};
use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;

// Helper function to rename .cargo/config.toml
pub fn rename_cargo_config(current_dir: &Path, log_file: &mut File, executor: Arc<dyn Execv + Send + Sync>) -> Result<bool, String> {
    let config_path = current_dir.join(".cargo/config.toml");
    let config_bak_path = current_dir.join(".cargo/config.toml.bak");

    if config_path.exists() {
        writeln!(log_file, "Renaming {:?} to {:?}...", config_path, config_bak_path)
            .map_err(|e| format!("Failed to write to log file: {}", e))?;
        
        let output = executor.execv(
            OsStr::new("mv"),
            &[config_path.as_os_str(), config_bak_path.as_os_str()],
            None,
        ).map_err(|e| format!("Failed to execute mv command: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to rename {:?} to {:?}: {}", config_path, config_bak_path, String::from_utf8_lossy(&output.stderr)));
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
