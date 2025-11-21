use anyhow::{Result, Context as AnyhowContext};
use anyhow::Context as _;
use std::path::PathBuf;
use cargo_repo_sync_lib::cargo_config_generator::{
    parse_members_file, generate_patch_entries, update_config_toml,
};

fn main() -> Result<()> {
    println!("Running cargo_repo_sync_lib_test_pkg to generate .cargo/config.toml...");

    let project_root = PathBuf::from("../../"); // Main cargo2nix project root
    let config_toml_path = PathBuf::from("./.cargo/config.toml"); // Relative to workspaces/

    // Ensure the .cargo directory exists
    let cargo_dir = config_toml_path.parent().unwrap();
    std::fs::create_dir_all(cargo_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create directory {:?}: {}", cargo_dir, e))?;

    let workspace_info = parse_members_file(&project_root)
        .map_err(|e| anyhow::anyhow!("Failed to parse members file: {}", e))?;
    let generated_patches = generate_patch_entries(&project_root, &workspace_info);

    update_config_toml(&config_toml_path, &generated_patches)
        .map_err(|e| anyhow::anyhow!("Failed to update config.toml: {}", e))?;

    println!("Successfully generated .cargo/config.toml in workspaces/.");
    Ok(())
}