use cargo_edit_lib::{CargoEditAdapter, CargoMetadataProvider};
use git_wrapper_lib::git_adapters::GitAdapter;
use git_wrapper_lib::system_git_executor::SystemGitExecutor;
use git_wrapper_lib::git_types::RollupLock;
use git_wrapper_lib::executors::system_execv::SystemExecv;
use anyhow::Result;
use std::path::{Path, PathBuf};
use cargo_metadata::Metadata; // Only import Metadata
use cargo_metadata::camino::Utf8PathBuf; // Corrected import
use serde_json;

// Dummy implementation of CargoMetadataProvider for testing
struct DummyCargoMetadataProvider;

impl CargoMetadataProvider for DummyCargoMetadataProvider {
    fn get_metadata(&self, _cargo_toml_path: &Path) -> Result<Metadata> {
        // Return a minimal, valid Metadata object by deserializing a JSON string
        let json_str = r#"{
            "packages": [],
            "workspace_members": [],
            "resolve": null,
            "workspace_root": "/",
            "target_directory": "/tmp",
            "version": 1,
            "workspace_default_members": [],
            "build_script_overrides": {},
            "build_directory": null,
            "custom_metadata": null
        }"#;
        let metadata: Metadata = serde_json::from_str(json_str)?;
        Ok(metadata)
    }
}

fn main() -> Result<()> {
    println!("cargo_repo_sync_lib_test_pkg is a test package.");

    let root_dir = PathBuf::from("../../");
    let git_executable_path = PathBuf::from("git"); // Assuming git is in PATH
    let execv_executor = std::sync::Arc::new(SystemExecv);
    let rollup_lock = std::sync::Arc::new(std::sync::Mutex::new(RollupLock::new()));

    let git_adapter: Box<dyn GitAdapter> = Box::new(SystemGitExecutor::new(
        git_executable_path,
        execv_executor,
        rollup_lock,
        root_dir.clone(),
    ));
    let cargo_metadata_provider: Box<dyn CargoMetadataProvider> = Box::new(DummyCargoMetadataProvider);

    // Test GitAdapter
    println!("\n--- Testing GitAdapter ---");
    let submodules = git_adapter.list_submodules(&root_dir)?;
    if submodules.is_empty() {
        println!("No submodules found.");
    } else {
        println!("Found submodules:");
        for (url, path) in submodules {
            println!("  URL: {}, Path: {:?}", url, path);
            let stat = git_adapter.get_submodule_head_and_workdir_hash(&path)?;
            println!("    Head Commit: {}, Workdir Hash: {}", stat.head_commit, stat.workdir_hash);
        }
    }

    // Test CargoMetadataProvider
    println!("\n--- Testing CargoMetadataProvider ---");
    // Assuming the current directory is a cargo workspace or package
    let cargo_toml_path = PathBuf::from("./Cargo.toml");
    let metadata = cargo_metadata_provider.get_metadata(&cargo_toml_path)?;
    println!("Cargo Metadata for {:?}: {:?}", cargo_toml_path, metadata);


    Ok(())
}
