# Crate: real-regex-adapter-lib

## Description
This crate provides a library that acts as an adapter for regular expression functionalities, specifically using the `regex` crate for its implementation. It implements the `RegexMatcher` trait from `tool-traits-lib`, offering a concrete, "real" regex matching solution that can be swapped or integrated with other components adhering to the same trait.

## Usage
This is a library crate, intended to be used as a dependency in other Rust projects. It exposes `RealRegexMatcher` which implements `tool_traits_lib::regex_adapter::RegexMatcher`.

## Dependencies
- `tool-traits-lib` (path = "../tool-traits-lib")
- `regex` (workspace, optional)
- `anyhow` (optional)
- Optional dependencies: `syn`, `toml_edit`, `clap`, `cargo-repo-sync-lib`, `git2`, `hex`, `cargo_metadata`, `sha1`, `pathdiff`, `serde`, `serde_json`, `walkdir`, `git-wrapper-lib`.

## Notes
The crate's default features (`anyhow_enabled`, `regex_enabled`) ensure that core functionality is available. It implements `RegexCaptures` and `RegexMatcher` traits, providing a standardized interface for regex operations. This is a crucial component for any tool requiring robust and adaptable regular expression capabilities.