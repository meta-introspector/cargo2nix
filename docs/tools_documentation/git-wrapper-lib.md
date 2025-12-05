# Crate: git-wrapper-lib

## Description
This crate provides a library that acts as a wrapper for Git functionalities. It offers various modules for Git adapters, executors (including dummy and pure Rust implementations), traits for Git operations, and type definitions related to Git. This library aims to abstract Git interactions, allowing other tools to perform Git operations through a consistent interface.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects that need to interact with Git. It re-exports `Execv` from `git_traits`.

## Dependencies
- `anyhow` (workspace, optional)
- Optional dependencies: `syn`, `clap`, `cargo_metadata`, `git2`, `walkdir`, `regex`, `toml_edit`, `pathdiff`, `sha1`, `hex`, `serde`, `serde_json`, `tool-traits-lib`.

## Notes
The crate's features (`git2_enabled`, `sha1_enabled`, `hex_enabled`, `with-trace`) indicate a modular design to enable specific Git functionalities and debugging options. The existence of `dummy_git_executor` and `pure_rust_git_executor` suggests flexibility in choosing how Git commands are executed. This is a foundational library for any Git-aware tools within the project.