use anyhow::Result;
use anyhow::Context;
use git_wrapper_lib::git_traits::Execv;
use std::{ffi::OsStr, fs::File, io::Write, path::Path, sync::Arc};

// Helper function to rename .cargo/config.toml
pub fn rename_cargo_config(
    current_dir: &Path,
    log_file: &mut File,
    executor: Arc<dyn Execv + Send + Sync>,
) -> anyhow::Result<bool> {
    let config_path = current_dir.join(".cargo/config.toml");
    let config_bak_path = current_dir.join(".cargo/config.toml.bak");

    if config_path.exists() {
        writeln!(
            log_file,
            "Renaming {:?} to {:?}...",
            config_path, config_bak_path
        )
        .context("Failed to write to log file")?;

        let output = executor
            .execv(
                OsStr::new("mv"),
                &[config_path.as_os_str(), config_bak_path.as_os_str()],
                None,
            )
            .context("Failed to execute mv command")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to rename {:?} to {:?}: {}",
                config_path,
                config_bak_path,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
