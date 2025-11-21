use anyhow::Result;
use git_wrapper_lib::git_adapters::{GitAdapter, MockGitAdapter, ShellGitAdapter, LibGitAdapter};
use git_wrapper_lib::executors::system_execv::SystemExecv;
use cargo_edit_lib::CargoMetadataProvider;
#[cfg(feature = "cargo_metadata")]
use crate::cargo_metadata_provider::{MockCargoMetadataProvider, RealCargoMetadataProvider};
#[cfg(not(feature = "cargo_metadata"))]
use crate::cargo_metadata_provider::NoopCargoMetadataProvider;
use nix_generator_lib::nix_adapters::{NixAdapter, MockNixAdapter, ShellNixAdapter};
use syn_adapter_lib::{SynAdapter, MockSynAdapter, LibSynAdapter};
use cargo_edit_lib::CargoEditAdapter;
use crate::cargo_edit_adapter_impl::CargoEditAdapterImpl;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    DryRun,
    Shell,
    Lib,
}

pub fn get_adapters(mode: Mode) -> Result<(Box<dyn GitAdapter>, Box<dyn CargoMetadataProvider>, Box<dyn NixAdapter>, Box<dyn SynAdapter>, Box<dyn CargoEditAdapter>)> {
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
                anyhow::bail!("'lib' mode for GitAdapter requires the 'git2' feature to be enabled.");
            }
        }
    };

    let cargo_metadata_provider: Box<dyn CargoMetadataProvider> = match mode {
        Mode::DryRun => {
            #[cfg(feature = "cargo_metadata")]
            {
                Box::new(MockCargoMetadataProvider::new())
            }
            #[cfg(not(feature = "cargo_metadata"))]
            {
                Box::new(NoopCargoMetadataProvider)
            }
        },
        // For Shell and Lib modes, we'll use RealCargoMetadataProvider as it's already a library-based approach
        Mode::Shell | Mode::Lib => {
            #[cfg(feature = "cargo_metadata")]
            {
                Box::new(RealCargoMetadataProvider)
            }
            #[cfg(not(feature = "cargo_metadata"))]
            {
                Box::new(NoopCargoMetadataProvider)
            }
        }
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

    let cargo_edit_adapter: Box<dyn CargoEditAdapter> = match mode {
        Mode::DryRun | Mode::Shell | Mode::Lib => Box::new(CargoEditAdapterImpl),
    };

    Ok((git_adapter, cargo_metadata_provider, nix_adapter, syn_adapter, cargo_edit_adapter))
}
