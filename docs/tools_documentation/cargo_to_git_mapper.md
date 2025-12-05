# Tool: cargo_to_git_mapper.rs

## Description
This tool is responsible for mapping Rust Cargo project information (from `Cargo.toml` files) to their corresponding Git module paths. It extracts details like crate name, version, and the git module path, suggesting its role in linking Rust codebases to their Git origins.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `CargoTomlInfo` struct captures essential details from a `Cargo.toml` file, including its path, crate name, version, and the critical `git_module_path`. This mapping is crucial for managing dependencies and understanding the provenance of code.