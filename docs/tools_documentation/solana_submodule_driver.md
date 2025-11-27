# Tool: solana_submodule_driver.rs

## Description
This tool acts as a "Solana Submodule Driver," designed to build `rustc` (the Rust compiler) exclusively using submodules and avoiding the Cargo registry. It manages the `submodules_path`, builds a `build_graph` of crates, and generates a `nix_build_order`, indicating a highly controlled and reproducible build process for Solana-specific `rustc` environments.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::path::Path`
- `std::process::Command`
- `std::collections::{HashMap, HashSet}`

## Notes
The module-level doc comment explicitly states its purpose: "builds rustc using only submodules, no cargo registry." The `SolanaSubmoduleDriver` struct maintains complex state including `submodules_path`, `build_graph`, and `nix_build_order`. This tool is a cornerstone for ensuring a deterministic and auditable build of the Rust compiler for Solana projects using Nix.