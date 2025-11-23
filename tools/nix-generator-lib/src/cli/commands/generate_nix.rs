use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::nix_adapters::{CrateInfo, NixAdapter};
use cargo_edit_lib::{CargoEditAdapter, CargoMetadataProvider, WorkspaceInfoProvider}; // Corrected import
use git_wrapper_lib::git_adapters::GitAdapter;

pub fn generate_nix(
    project_root: &Path,
    output_path: &Path,
    git_adapter: &dyn GitAdapter,
    cargo_metadata_provider: &dyn CargoMetadataProvider,
    nix_adapter: &dyn NixAdapter,
    cargo_edit_adapter: &dyn CargoEditAdapter,
    workspace_info_provider: &dyn WorkspaceInfoProvider, // Added workspace_info_provider
) -> Result<()> {
    println!("Generating Nix expressions...");

    let workspace_info = workspace_info_provider
        .parse_members_file(git_adapter, cargo_metadata_provider, project_root)
        .context("Failed to parse members file for Nix generation")?;

    // Ensure the output directory exists
    let output_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(output_dir).map_err(|e| {
        anyhow::anyhow!("Failed to create output directory {:?}: {}", output_dir, e)
    })?;

    for info in workspace_info {
        for member_name in info.member_crates {
            let crate_path = project_root
                .join(&info.submodule_base_path_rel)
                .join(&member_name);

            // For now, we'll create a dummy CrateInfo. This should be populated with real data.
            let crate_info = CrateInfo {
                name: member_name.clone(),
                version: "0.1.0".to_string(), // Placeholder
                path: crate_path.clone(),
            };

            let nix_content = nix_adapter.generate_nix_expression(&crate_info)?;
            let crate_output_path = output_path.join(format!("{}.nix", member_name));
            nix_adapter.write_nix_expression(&crate_output_path, &nix_content)?;
            println!(
                "Generated Nix expression for {} at {:?}",
                member_name, crate_output_path
            );
        }
    }

    println!("Nix expression generation complete.");
    Ok(())
}
