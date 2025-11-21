use anyhow::{Result, Context};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::api::{CargoEditRequest, CargoEditResponse, CargoTomlPatch};

pub trait CargoEditExecutor: Send + Sync {
    fn read_cargo_toml(&self, path: &Path) -> Result<String>;
    fn apply_patches(&self, path: &Path, patches: Vec<CargoTomlPatch>) -> Result<()>;
}

pub struct RealCargoEditExecutor {
    binary_path: PathBuf,
}

impl RealCargoEditExecutor {
    pub fn new(binary_path: PathBuf) -> Self {
        RealCargoEditExecutor { binary_path }
    }

    fn execute_request(&self, request: CargoEditRequest) -> Result<CargoEditResponse> {
        let request_json = serde_json::to_string(&request).context("Failed to serialize request")?;

        let mut child = Command::new(&self.binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context(format!("Failed to spawn cargo-edit-tool binary at {:?}", self.binary_path))?;

        child.stdin.as_mut().context("Failed to open stdin for cargo-edit-tool")?.write_all(request_json.as_bytes())?;

        let output = child.wait_with_output().context("Failed to wait for cargo-edit-tool output")?;

        if !output.status.success() {
            anyhow::bail!("cargo-edit-tool failed with status: {:?}, stderr: {}", output.status, String::from_utf8_lossy(&output.stderr));
        }

        let response: CargoEditResponse = serde_json::from_slice(&output.stdout).context("Failed to deserialize response from cargo-edit-tool")?;

        Ok(response)
    }
}

impl CargoEditExecutor for RealCargoEditExecutor {
    fn read_cargo_toml(&self, path: &Path) -> Result<String> {
        let request = CargoEditRequest::ReadCargoToml { path: path.to_path_buf() };
        match self.execute_request(request)? {
            CargoEditResponse::CargoTomlContent { content } => Ok(content),
            CargoEditResponse::Error { message } => anyhow::bail!("Error from cargo-edit-tool: {}", message),
            _ => anyhow::bail!("Unexpected response from cargo-edit-tool for ReadCargoToml"),
        }
    }

    fn apply_patches(&self, path: &Path, patches: Vec<CargoTomlPatch>) -> Result<()> {
        let request = CargoEditRequest::ApplyPatches { path: path.to_path_buf(), patches };
        match self.execute_request(request)? {
            CargoEditResponse::ApplyPatchesResult { success, message } => {
                if success {
                    Ok(())
                } else {
                    anyhow::bail!("Failed to apply patches: {}", message.unwrap_or_else(|| "unknown error".to_string()))
                }
            },
            CargoEditResponse::Error { message } => anyhow::bail!("Error from cargo-edit-tool: {}", message),
            _ => anyhow::bail!("Unexpected response from cargo-edit-tool for ApplyPatches"),
        }
    }
}

pub struct DummyCargoEditExecutor;

impl CargoEditExecutor for DummyCargoEditExecutor {
    fn read_cargo_toml(&self, path: &Path) -> Result<String> {
        println!("DummyCargoEditExecutor: Reading Cargo.toml from {:?}", path);
        Ok(format!("# Dummy content for {:?}", path))
    }

    fn apply_patches(&self, path: &Path, patches: Vec<CargoTomlPatch>) -> Result<()> {
        println!("DummyCargoEditExecutor: Applying {} patches to {:?}", patches.len(), path);
        Ok(())
    }
}
