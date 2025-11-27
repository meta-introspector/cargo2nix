# Crate: cargo-toml-editor-lib

## Description
This crate provides a library for programmatically editing `Cargo.toml` files. It exposes modules for an API, an executor, and a patcher, suggesting a structured approach to modifying `Cargo.toml` content, potentially for automated dependency management, feature toggling, or manifest manipulation.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. It re-exports modules `api`, `executor`, and `patcher`.

## Dependencies
- `anyhow` (workspace)
- Optional dependencies: `syn`, `clap`, `cargo-repo-sync-lib`, `git2`, `hex`, `cargo_metadata`, `sha1`, `pathdiff`, `walkdir`, `git-wrapper-lib`, `toml_edit`, `serde`, `serde_json`.

## Notes
The crate's features, especially `real_toml_edit`, indicate its reliance on `toml_edit` for actual TOML manipulation, while providing an abstraction layer for these operations. It is a core utility for any tool that needs to modify Cargo manifests reliably.