# Crate: nix-generator-lib

## Description
This crate provides a library for generating Nix expressions. It includes modules for command-line interface (CLI) related functionalities (`cli`) and various Nix adapters (`nix_adapters`), suggesting it's a foundational component for tools that automate the creation of Nix build configurations for Rust projects.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. It exposes `cli` and `nix_adapters` modules.

## Dependencies
- `anyhow` (workspace)
- `cargo_metadata` (workspace)
- `cargo-edit-lib` (path = "../cargo-edit-lib")
- `git-wrapper-lib` (path = "../git-wrapper-lib")
- `toml_edit`
- Optional dependencies: `syn`, `clap`, `git2`, `hex`, `sha1`, `pathdiff`, `serde_json`, `walkdir`, `serde`.

## Notes
The dependency on `toml_edit` and `cargo_metadata` suggests it parses Cargo manifests to extract information needed for Nix generation. This crate is crucial for integrating Rust projects into the Nix ecosystem.