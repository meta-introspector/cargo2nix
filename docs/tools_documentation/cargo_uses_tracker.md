# Tool: cargo_uses_tracker.rs

## Description
This tool tracks the usage of crates, building a comprehensive graph that shows which crates utilize others. It provides a detailed view of the interdependencies within a Rust workspace or project, which can be vital for refactoring, understanding impact of changes, or optimizing build processes.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::collections::HashMap`

## Notes
The `CargoUsesTracker` struct maintains a `uses_graph`, mapping a crate name to a vector of crates that it depends on. This implies it focuses on direct dependencies and could be used for constructing a more granular dependency tree.