# Tool: simple_crate_scanner.rs

## Description
This tool provides a simple scanner for Rust crates within submodules. It iterates through the `submodules` directory, identifies `Cargo.toml` files, and generates a report of the crates found. This is a basic utility for quickly inventorying Rust projects within a submodule-based repository.

## Usage
The `main` function scans the `submodules` directory.
Example: `cargo run --bin simple_crate_scanner` (assuming it's a binary crate)

## Dependencies
- `std::fs`

## Notes
The tool generates a `# Submodule Crate Report`, indicating its use for basic overview and auditing of the project's Rust crates.