# Crate: real-toml-adapter-lib

## Description
This crate provides a library that acts as an adapter for TOML (Tom's Obvious, Minimal Language) file manipulation, specifically using the `toml_edit` crate for its implementation. It implements the `TomlAdapter` trait from `tool-traits-lib`, offering a concrete, "real" TOML editing solution that can be integrated with other components adhering to the same trait.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. It exposes `RealTomlAdapter` which implements `tool_traits_lib::toml_adapter::TomlAdapter`.

## Dependencies
- `tool-traits-lib` (path = "../tool-traits-lib")
- `toml_edit` (workspace, optional)
- Optional dependencies: `syn`, `clap`, `cargo-repo-sync-lib`, `git2`, `hex`, `cargo_metadata`, `sha1`, `anyhow`, `pathdiff`, `serde`, `serde_json`, `walkdir`, `git-wrapper-lib`.

## Notes
The crate provides a `RealTomlAdapter` that wraps `toml_edit`'s `Document` parsing functionality, allowing for robust parsing and manipulation of TOML documents. This is a crucial component for any tool requiring reliable and adaptable TOML configuration capabilities.