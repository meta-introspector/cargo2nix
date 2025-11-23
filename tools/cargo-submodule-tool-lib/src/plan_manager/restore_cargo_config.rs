use anyhow::Result;
use git_wrapper_lib::git_traits::Execv;
use std::{ffi::OsStr, fs::File, io::Write, path::Path, sync::Arc};

// Helper function to restore .cargo/config.toml
pub fn restore_cargo_config(
    current_dir: &Path,
    log_file: &mut File,
    was_renamed: bool,
    executor: Arc<dyn Execv + Send + Sync>,
) -> Result<(), String> {
    if was_renamed {
        let config_path = current_dir.join(".cargo/config.toml");
        let config_bak_path = current_dir.join(".cargo/config.toml.bak");
        writeln!(
            log_file,
            "Restoring {:?} from {:?}...",
            config_path, config_bak_path
        )
        .map_err(|e| format!("Failed to write to log file: {}", e))?;

        let output = executor
            .execv(
                OsStr::new("mv"),
                &[config_bak_path.as_os_str(), config_path.as_os_str()],
                None,
            )
            .map_err(|e| format!("Failed to execute mv command: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to restore {:?} from {:?}: {}",
                config_bak_path,
                config_path,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(())
}
