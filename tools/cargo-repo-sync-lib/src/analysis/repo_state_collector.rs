use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use git2::Repository;
use cargo_metadata::MetadataCommand;
use walkdir::WalkDir;

// --- Data Structures for RepoState ---

use crate::repo_state_types::{SubmoduleInfo, PackageInfo, DependencyInfo, CargoWorkspaceInfo, NixFlakeInfo, RepoState};

// --- RepoStateCollector Trait and Implementation ---

pub trait RepoStateCollector {
    fn collect_repo_state(&self, project_root: &Path) -> Result<RepoState>;
}

pub struct RealRepoStateCollector;

impl RepoStateCollector for RealRepoStateCollector {
    fn collect_repo_state(&self, project_root: &Path) -> Result<RepoState> {
        let mut repo_state = RepoState::default();

        // 1. Collect Submodule Info
        let repo = Repository::open(project_root)
            .context(format!("Failed to open git repository at {:?}", project_root))?;

        for submodule in repo.submodules().context("Failed to read submodules")? {
            let name = submodule.name().unwrap_or("unknown").to_string();
            let path = PathBuf::from(submodule.path()); // Relative to repo root
            let url = submodule.url().unwrap_or("unknown").to_string();
            let branch = submodule.branch().map(|s| s.to_string());
            let commit_id = submodule.head_id().map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string());

            repo_state.submodules.push(SubmoduleInfo {
                name,
                path,
                url,
                branch,
                commit_id,
            });
        }

        // 2. Collect Cargo Workspace/Package Info
        // Find all Cargo.toml files in the project
        for entry in WalkDir::new(project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
        {
            let manifest_path = entry.path().to_path_buf();
            
            // Try to get cargo metadata for each manifest
            let metadata = MetadataCommand::new()
                .manifest_path(&manifest_path)
                .no_deps() // We only need info about the package itself, not its dependencies
                .exec()
                .with_context(|| format!("Failed to get cargo metadata for manifest: {:?}", manifest_path))?;
            
            let mut packages_in_workspace = Vec::new();
            for pkg in metadata.packages {
                let mut deps_info = Vec::new();
                for dep in pkg.dependencies {
                    deps_info.push(DependencyInfo {
                        name: dep.name,
                        source: dep.source.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "path".to_string()), // Default to path if source is None
                        req: dep.req.to_string(),
                    });
                }
                packages_in_workspace.push(PackageInfo {
                    name: pkg.name.to_string(),
                    version: pkg.version.to_string(),
                    manifest_path: PathBuf::from(pkg.manifest_path),
                    dependencies: deps_info,
                });
            }

            // If it's a workspace root, store it as a CargoWorkspaceInfo
            let workspace_root_str = metadata.workspace_root.as_str();
            if !workspace_root_str.is_empty() {
                if Path::new(workspace_root_str) == manifest_path.parent().unwrap() {
                    repo_state.cargo_workspaces.push(CargoWorkspaceInfo {
                        manifest_path,
                        packages: packages_in_workspace,
                    });
                }
            } else {
                // If it's a single package, treat it as a workspace of one package
                repo_state.cargo_workspaces.push(CargoWorkspaceInfo {
                    manifest_path,
                    packages: packages_in_workspace,
                });
            }
        }

        // 3. Collect Nix Flake Info (simplified for now)
        // This part would involve parsing flake.nix and Cargo.nix files.
        // For now, let's just find them.
        for entry in WalkDir::new(project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && (e.file_name() == "flake.nix" || e.file_name() == "Cargo.nix"))
        {
            let flake_path = entry.path().to_path_buf();
            // Placeholder for actual parsing of flake inputs/outputs
            repo_state.nix_flakes.push(NixFlakeInfo {
                flake_path,
                inputs: HashMap::new(), // To be populated by actual parsing
                outputs: Vec::new(),    // To be populated by actual parsing
            });
        }

        Ok(repo_state)
    }
}
