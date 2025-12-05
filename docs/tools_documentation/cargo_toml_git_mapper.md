# Tool: cargo_toml_git_mapper.rs

## Description
This tool creates a mapping between Git modules and the `Cargo.toml` files contained within them. It aggregates `Cargo.toml` file paths under their respective Git module identifiers, useful for understanding the structure of multi-crate Git repositories.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::collections::HashMap`

## Notes
The `CargoTomlGitMapper` struct stores a `HashMap` where keys are Git module names and values are lists of `Cargo.toml` file paths, providing a clear association between Git repositories and their Rust components.