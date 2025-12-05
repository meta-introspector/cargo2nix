#[cfg(feature = "anyhow_enabled")]
use anyhow::Result;
#[cfg(not(feature = "anyhow_enabled"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
#[cfg(feature = "walkdir_enabled")]
use walkdir::WalkDir;

use crate::git_traits::GitExecutor;
#[cfg(feature = "cargo_metadata_enabled")]
use tool_traits_lib::cargo_metadata_provider::CargoMetadataProvider;

use tool_traits_lib::types::{
    CargoWorkspaceInfo, DependencyInfo, NixFlakeInfo, PackageInfo, RepoState, SubmoduleInfo,
};

// --- RepoStateCollector Trait and Implementation ---

pub trait RepoStateCollector {
    fn collect_repo_state(&self, project_root: &Path) -> Result<RepoState>;
}

pub struct RealRepoStateCollector {
    git_executor: Arc<dyn GitExecutor + Send + Sync>,
    cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync>,
}

impl RealRepoStateCollector {
    pub fn new(
        git_executor: Arc<dyn GitExecutor + Send + Sync>,
        cargo_metadata_provider: Box<dyn CargoMetadataProvider + Send + Sync>,
    ) -> Self {
        RealRepoStateCollector {
            git_executor,
            cargo_metadata_provider,
        }
    }
}

impl RepoStateCollector for RealRepoStateCollector {
    fn collect_repo_state(&self, project_root: &Path) -> Result<RepoState> {
        let mut repo_state = RepoState::default();

        // 1. Collect Submodule Info
        for (url, path) in self.git_executor.list_submodules(project_root)? {
            let name = path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("unknown")
                .to_string();

            repo_state.submodules.push(SubmoduleInfo {
                name,
                path,
                url,
                branch: None, // GitExecutor::list_submodules does not provide branch info
                commit_id: "unknown".to_string(), // GitExecutor::list_submodules does not provide commit_id
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
            let metadata = self
                .cargo_metadata_provider
                .provide_metadata(&manifest_path)?;

            let mut packages_in_workspace = Vec::new();
            for pkg in metadata.packages {
                let mut deps_info = Vec::new();
                for dep in pkg.dependencies {
                    deps_info.push(DependencyInfo {
                        name: dep.name,
                        source: dep
                            .source
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "path".to_string()), // Default to path if source is None
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
            .filter(|e| {
                e.file_type().is_file()
                    && (e.file_name() == "flake.nix" || e.file_name() == "Cargo.nix")
            })
        {
            let flake_path = entry.path().to_path_buf();
            // Placeholder for actual parsing of flake inputs/outputs
            repo_state.nix_flakes.push(NixFlakeInfo {
                flake_path,
                inputs: BTreeMap::new(), // To be populated by actual parsing
                outputs: Vec::new(),     // To be populated by actual parsing
            });
        }

        Ok(repo_state)
    }
}
