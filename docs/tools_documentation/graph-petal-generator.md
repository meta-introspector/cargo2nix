# Crate: graph-petal-generator

## Description
This crate provides a command-line tool, `graph-petal-generator`, designed to generate and analyze graph structures, particularly dependency graphs. It leverages the `petgraph` library for graph algorithms and `clap` for command-line parsing. It seems to work with `CargoEntry` and `DependencyGraphData` structures, suggesting its role in visualizing or processing Cargo-related dependency information.

## Usage
This is an executable tool. Its usage would typically involve running `cargo run -p graph-petal-generator -- [OPTIONS]` or `graph-petal-generator [OPTIONS]` after building. It uses `clap` for command-line argument parsing.

## Dependencies
- `petgraph` ("0.6")
- `clap` ("4.0", with `derive` feature)
- `serde` ("1.0", with `derive` feature)
- `serde_json` ("1.0")

## Notes
The tool re-defines `CargoEntry` and `DependencyGraphData` to match the output of `rust-src-scanner`, indicating integration with other project components. It utilizes `petgraph` for creating directed graphs (`DiGraph`) and performing topological sorting (`toposort`), making it a powerful utility for analyzing complex dependency relationships. The term "petal" might refer to a specific pattern or sub-structure within the generated graphs.