# Crate: cargo-submodule-tool

## Description
This crate provides a command-line tool designed for managing Git submodules within Cargo projects. It likely offers functionalities to interact with submodules, synchronize their states, and possibly integrate their dependencies into the Cargo build system.

## Usage
This is an executable tool. Its usage would typically involve running it via `cargo run -p cargo-submodule-tool -- [OPTIONS]` or after building, `cargo-submodule-tool [OPTIONS]`. It uses `clap` for command-line argument parsing.

## Dependencies
- `anyhow` (workspace, optional)
- `clap` (workspace, optional)
- `clap_derive` (workspace, optional)
- `cargo_metadata` (workspace, optional)
- `git2` (workspace, optional)
- `hex` (workspace, optional)
- `lazy_static` (workspace, optional)
- `pathdiff` (workspace, optional)
- `serde` (workspace, optional, with `derive` feature)
- `serde_json` (workspace, optional)
- `sha1` (optional)
- `syn` (optional)
- `toml` (workspace, optional)
- `toml_edit` (workspace, optional)
- `walkdir` (workspace, optional)
- Local dependencies: `cargo-repo-sync-lib` (path = "../cargo-repo-sync-lib", optional), `cargo-submodule-tool-lib` (path = "../cargo-submodule-tool-lib"), `git-wrapper-lib` (path = "../git-wrapper-lib", optional).

## Notes
The tool integrates with `cargo-submodule-tool-lib` for core submodule logic and `git-wrapper-lib` for Git operations. Its `package.metadata.cargo-repo-sync` suggests it might be involved in bootstrapping Cargo paths or other repo synchronization tasks.