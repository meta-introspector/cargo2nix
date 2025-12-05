# Tool: cargo_dependency_mapper.rs

## Description
This tool is likely used to map and analyze dependencies of Rust projects by parsing `Cargo.toml` or `Cargo.lock` files. It extracts and represents cargo dependencies, potentially from various sources like git, crates.io, or local paths.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `CargoDependency` struct suggests a detailed representation of each dependency, including name, version, and source. It plays a crucial role in understanding the project's dependency graph.