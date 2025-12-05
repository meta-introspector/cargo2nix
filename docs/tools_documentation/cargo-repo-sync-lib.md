# Crate: cargo-repo-sync-lib

## Description
This crate is a library (`cargo-repo-sync-lib`) intended to provide functionality for synchronizing repositories. It's designed to be a core component for tools that manage and update Git repositories, possibly in the context of Cargo projects.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. It exposes a public function `hello_repo_sync()` as a placeholder or example.

## Dependencies
- `anyhow` (workspace, optional)
- `clap` (workspace, optional)
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
- `git-wrapper-lib` (path = "../git-wrapper-lib", optional)

## Notes
The `lib.rs` file explicitly states it's a "Minimal lib.rs for cargo-repo-sync-lib" and that "This crate is likely intended to be a library." Its numerous optional dependencies suggest a flexible design, allowing consumers to enable specific features as needed. It may serve as a foundational layer for more complex repository synchronization tools.