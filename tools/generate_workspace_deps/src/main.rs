use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use cargo_repo_sync_lib::workspace_deps_generator::{RealWorkspaceDepsGenerator, WorkspaceDepsGenerator};


fn main() -> Result<()> {
    let root_dir = PathBuf::from("/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix");
    let submodules_dir = root_dir.join("submodules");

    println!("[workspace.dependencies]");

    let generator = RealWorkspaceDepsGenerator;
    let submodule_path_map = generator.generate_submodule_path_map(&submodules_dir)?;

    // Sort the keys for consistent output
    let mut sorted_keys: Vec<&String> = submodule_path_map.keys().collect();
    sorted_keys.sort();

    for crate_name in sorted_keys {
        if let Some(relative_path) = submodule_path_map.get(crate_name) {
            // The relative_path from generate_submodule_path_map is relative to the project root.
            // We need to make it relative to the Cargo.toml where [workspace.dependencies] is defined.
            // In this case, it's the project root itself, so we just need to prefix with "./"
            let path_str = format!("./{}", relative_path.display());
            println!("{} = {{ path = \"{}\" }}", crate_name, path_str);
        }
    }

    Ok(())
}