use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf, // Relative path from project root
    pub url: String,
    pub branch: Option<String>,
    pub commit_id: String,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf, // Absolute path to Cargo.toml
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub source: String, // e.g., "crates.io", "git+https://...", "path+../..", "registry+https://github.com/rust-lang/crates.io-index"
    pub req: String,    // version requirement or path
}

#[derive(Debug, Clone)]
pub struct CargoWorkspaceInfo {
    pub manifest_path: PathBuf, // Absolute path to workspace Cargo.toml
    pub packages: Vec<PackageInfo>, // All packages within this workspace
}

#[derive(Debug, Clone)]
pub struct NixFlakeInfo {
    pub flake_path: PathBuf, // Path to flake.nix or Cargo.nix
    pub inputs: HashMap<String, String>, // For flake.nix inputs
    pub outputs: Vec<String>, // What the flake provides (e.g., packages.x86_64-linux.default) - simplified for now
}

#[derive(Debug, Default, Clone)]
pub struct RepoState {
    pub submodules: Vec<SubmoduleInfo>,
    pub cargo_workspaces: Vec<CargoWorkspaceInfo>,
    pub nix_flakes: Vec<NixFlakeInfo>,
}
