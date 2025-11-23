use anyhow::{anyhow, Result};
use std::path::Path;

pub trait WorkspaceRemover {
    fn remove_workspace_members(
        &self,
        cargo_toml_path: &Path,
        members_to_remove: &[String],
    ) -> Result<()>;
}

#[cfg(feature = "cargo-toml-editor-lib")]
pub struct RealWorkspaceRemover;

#[cfg(feature = "cargo-toml-editor-lib")]
impl WorkspaceRemover for RealWorkspaceRemover {
    fn remove_workspace_members(
        &self,
        cargo_toml_path: &Path,
        members_to_remove: &[String],
    ) -> Result<()> {
        // This implementation would typically use cargo_toml_editor_lib to modify Cargo.toml
        // For now, we'll just return an error if the feature is not enabled.
        anyhow::bail!(
            "`RealWorkspaceRemover` is not implemented without `cargo-toml-editor-lib` feature."
        );
    }
}

#[cfg(not(feature = "cargo-toml-editor-lib"))]
pub struct RealWorkspaceRemover;

#[cfg(not(feature = "cargo-toml-editor-lib"))]
impl WorkspaceRemover for RealWorkspaceRemover {
    fn remove_workspace_members(
        &self,
        _cargo_toml_path: &Path,
        _members_to_remove: &[String],
    ) -> Result<()> {
        Err(anyhow!(
            "`WorkspaceRemover` requires the `cargo-toml-editor-lib` feature to be enabled."
        ))
    }
}
