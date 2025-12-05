# Crate: cargo-feature-adapter

## Description
This crate provides a tool for adapting Cargo features, likely to facilitate changes or transformations in how features are declared and used within Rust projects. It seems to process `Cargo.toml` files and source code (`src_adapter`) to adjust feature configurations, potentially for compatibility, optimization, or integration with other tools.

## Usage
This is an executable tool that uses `clap` for command-line argument parsing. It would typically be run via `cargo run -p cargo-feature-adapter -- [OPTIONS]` or after building, `cargo-feature-adapter [OPTIONS]`.

## Dependencies
- `anyhow` (workspace)
- `clap` (workspace, with `derive` feature)
- `fs_extra` (workspace)
- `toml_edit` (workspace)
- Optional dependencies: `cargo-submodule-tool-lib`, `cargo-repo-sync-lib`, `git2`, `cargo_metadata`, `pathdiff`, `git-wrapper-lib`, `syn`, `quote`, `proc-macro2`, `walkdir`, `sha1`, `hex`, `serde`, `serde_json`.

## Notes
The presence of `cargo_toml_adapter` and `src_adapter` modules suggests it modifies both `Cargo.toml` and potentially Rust source files to achieve its feature adaptation. This tool is valuable for automated refactoring or migration efforts related to Cargo features.