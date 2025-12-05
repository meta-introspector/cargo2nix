# Tool: git_to_cargo_mapper.rs

## Description
This tool maps Git repositories to the Cargo modules (crates) they contain. It extracts details about each Cargo module, including its name, `Cargo.toml` path, version, and description, from the context of its Git repository. This is crucial for building a comprehensive understanding of the project's structure, correlating Git-level organization with Rust-level modularity.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `CargoModule` struct captures essential metadata for each Rust crate found within a Git repository. This tool serves as a bridge between the Git and Cargo ecosystems within the project.