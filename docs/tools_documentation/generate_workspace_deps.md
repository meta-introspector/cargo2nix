# Crate: generate_workspace_deps

## Description
This crate provides a tool, `generate_workspace_deps`, designed to generate dependencies for Cargo workspaces. It likely traverses project directories using `walkdir` to find `Cargo.toml` files, gathers Cargo metadata, and then generates a dependency structure suitable for workspace configuration. It includes a `metadata_provider` module, suggesting it can collect metadata from local Cargo.toml files within submodules.

## Usage
This is an executable tool. Its usage would typically involve running `cargo run -p generate_workspace_deps -- [OPTIONS]` or `generate_workspace_deps [OPTIONS]` after building. It uses `clap` (optional) for command-line argument parsing.

## Dependencies
- `anyhow` (workspace)
- `walkdir` (workspace, optional)
- `cargo_metadata` (workspace, optional)
- `cargo-edit-lib` (path = "../cargo-edit-lib")
- Optional dependencies: `syn`, `toml_edit`, `clap`, `hex`, `sha1`, `pathdiff`, `serde`, `serde_json`, `git2`, `regex`, `cargo-submodule-tool-lib`, `cargo-repo-sync-lib`.

## Notes
The tool integrates with `metadata_provider` for collecting Cargo metadata locally, making it suitable for complex projects with submodules. It seems to play a crucial role in automating the setup and maintenance of large Cargo workspaces.