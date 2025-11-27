# Crate: dep2submodule

## Description
This crate provides a command-line tool, `dep2submodule`, designed to convert dependencies into submodules. It likely analyzes Cargo project dependencies (using `cargo_metadata`) and manages file system operations (using `walkdir`) to restructure dependencies as Git submodules. This tool is crucial for managing project dependencies in a highly controlled, submodule-based manner.

## Usage
This is an executable tool. Its usage would typically involve running `cargo run -p dep2submodule -- [OPTIONS]` or `dep2submodule [OPTIONS]` after building. It uses `clap` for command-line argument parsing.

## Dependencies
- `anyhow` (workspace)
- `cargo_metadata` (workspace)
- Optional dependencies: `syn`, `git2`, `hex`, `sha1`, `serde`, `serde_json`, `walkdir`, `toml_edit`, `clap`, `pathdiff`, `cargo-submodule-tool-lib`.

## Notes
The tool supports `walkdir` and `toml_edit` (optional), suggesting it traverses directories and modifies `Cargo.toml` files. Its primary purpose appears to be transforming a project's dependency model from a traditional Cargo registry approach to a submodule-based one.