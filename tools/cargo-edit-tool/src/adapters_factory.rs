#[cfg(not(feature = "real_cargo_metadata"))]
use crate::cargo_metadata_provider::DummyCargoMetadataProvider;
#[cfg(not(feature = "cargo_metadata"))]
#[cfg(feature = "real_cargo_metadata")]
use crate::cargo_metadata_provider::RealCargoMetadataProvider;
#[cfg(feature = "cargo_metadata")]
use crate::cargo_metadata_provider::{MockCargoMetadataProvider, RealCargoMetadataProvider};
use anyhow::Result;
use cargo_edit_lib::CargoMetadataProvider;
use cargo_toml_editor_lib::executor::{
    CargoEditExecutor, DummyCargoEditExecutor, RealCargoEditExecutor,
};
use git_wrapper_lib::executors::system_execv::SystemExecv;
use git_wrapper_lib::git_adapters::{GitAdapter, LibGitAdapter, MockGitAdapter, ShellGitAdapter};
use nix_generator_lib::nix_adapters::{MockNixAdapter, NixAdapter, ShellNixAdapter};
use syn_adapter_lib::{LibSynAdapter, MockSynAdapter, SynAdapter};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    DryRun,
    Shell,
    Lib,
}

pub fn get_adapters(
    mode: Mode,
) -> Result<(
    Box<dyn GitAdapter + Send + Sync>,
    Box<dyn CargoMetadataProvider + Send + Sync>,
    Box<dyn NixAdapter + Send + Sync>,
    Box<dyn SynAdapter + Send + Sync>,
    Box<dyn CargoEditExecutor + Send + Sync>, // Changed to CargoEditExecutor
)> {
    let git_adapter: Box<dyn GitAdapter> = match mode {
        Mode::DryRun => Box::new(MockGitAdapter::new()),
        Mode::Shell => Box::new(ShellGitAdapter::new(Box::new(SystemExecv))),
        Mode::Lib => {
            #[cfg(feature = "git2")]
            {
                Box::new(LibGitAdapter::new())
            }
            #[cfg(not(feature = "git2"))]
            {
                anyhow::bail!(
                    "'lib' mode for GitAdapter requires the 'git2' feature to be enabled."
                );
            }
        }
    };

    let cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync> =
        if cfg!(feature = "real_cargo_metadata") {
            Box::new(RealCargoMetadataProvider)
        } else {
            Box::new(DummyCargoMetadataProvider)
        };

    let nix_adapter: Box<dyn NixAdapter> = match mode {
        Mode::DryRun => Box::new(MockNixAdapter::new()),
        Mode::Shell => Box::new(ShellNixAdapter::new()),
        Mode::Lib => Box::new(ShellNixAdapter::new()), // For now, LibNixAdapter is same as ShellNixAdapter
    };

    let syn_adapter: Box<dyn SynAdapter> = match mode {
        Mode::DryRun => Box::new(MockSynAdapter::new()),
        Mode::Shell | Mode::Lib => {
            #[cfg(feature = "syn-parsing")]
            {
                Box::new(LibSynAdapter::new())
            }
            #[cfg(not(feature = "syn-parsing"))]
            {
                anyhow::bail!("'shell' or 'lib' mode for SynAdapter requires the 'syn-parsing' feature to be enabled.");
            }
        }
    };

    let cargo_edit_executor: Box<dyn CargoEditExecutor + Send + Sync> = // Changed to CargoEditExecutor
        if cfg!(feature = "real_toml_edit") {
            Box::new(RealCargoEditExecutor::new(PathBuf::from("cargo-toml-editor-tool"))) // Assuming a binary name
        } else {
            Box::new(DummyCargoEditExecutor)
        };

    Ok((
        git_adapter,
        cargo_metadata_provider,
        nix_adapter,
        syn_adapter,
        cargo_edit_executor,
    ))
}
