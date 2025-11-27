# Crate: cargo-edit-lib

## Description
This crate appears to be a library providing functionality related to editing or manipulating Cargo projects. Based on its `Cargo.toml` dependencies, it can interact with Git repositories, parse TOML files (like `Cargo.toml`), and handle Cargo metadata. It might be a core library for tools that modify `Cargo.toml` files or analyze Cargo project structures.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. Specific usage details would depend on the public API it exposes.

## Dependencies
- `anyhow` (workspace)
- `git_wrapper_lib` (path = "../git-wrapper-lib")
- Optional dependencies: `cargo-submodule-tool-lib`, `syn`, `clap`, `git2`, `hex`, `sha1`, `pathdiff`, `cargo_metadata`, `serde`, `serde_json`, `regex`, `lazy_static`, `toml_edit`, `walkdir`.

## Notes
The crate's features (`syn_enabled`, `clap_enabled`, `git2_enabled`, etc.) indicate a modular design, allowing selective compilation of functionalities. It appears to be a foundational component for advanced Cargo project management and analysis.