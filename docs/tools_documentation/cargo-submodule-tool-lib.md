# Crate: cargo-submodule-tool-lib

## Description
This crate serves as a library providing core functionalities for managing Cargo-related submodules. It encapsulates logic for submodule analysis, command-line interface (CLI) interactions, file system caching and writing, plan management, repository discovery, and synchronization. It's a foundational library for building robust submodule management tools.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects, particularly by tools like `cargo-submodule-tool`. It re-exports several modules (`analysis`, `cli`, `fs_cache`, `fs_writer`, `plan_manager`, `repo_discovery`, `repo_sync_lib`, `traits`).

## Dependencies
- `anyhow` (workspace, optional)
- `cargo2nix` (path = "../../crates/cargo2nix", optional)
- `cargo_metadata` (workspace, optional)
- `clap` (workspace, optional)
- `git-wrapper-lib` (path = "../git-wrapper-lib", optional)
- `git2` (workspace, optional)
- `hex` (workspace, optional)
- `lazy_static` (workspace, optional)
- `lru` (optional)
- `md5` (workspace, optional)
- `pathdiff` (workspace, optional)
- `regex` (workspace, optional)
- `serde` (workspace, optional, with `derive` feature)
- `serde_json` (workspace, optional)
- `sha1` (optional)
- `sha2` (workspace, optional)
- `syn` (optional)
- `toml` (workspace, optional)
- `toml_edit` (workspace, optional)
- `tool-traits-lib` (path = "../tool-traits-lib", optional)
- `url` (workspace, optional)
- `walkdir` (workspace, optional)
- `real-regex-adapter-lib` (path = "../real-regex-adapter-lib", optional)
- `cargo-repo-sync-lib` (path = "../cargo-repo-sync-lib", optional)

## Notes
The crate has an extensive list of optional dependencies and features, indicating a modular design that allows for selective inclusion of advanced functionalities. It acts as a central hub for submodule-related logic, making it a critical library in the project's dependency management ecosystem.