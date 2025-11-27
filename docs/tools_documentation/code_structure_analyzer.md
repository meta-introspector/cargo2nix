# Tool: code_structure_analyzer.rs

## Description
This tool analyzes the fundamental code structure of Rust crates. It identifies and categorizes key elements such as crate names, library/main files, declarations (functions, structs, enums, traits), and exported items. This deep insight into code structure is valuable for architectural analysis, documentation generation, or refactoring efforts.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `CodeStructure` struct captures a detailed view of a crate, including its `crate_name`, `libs`, `decls`, and `exports`. This forms a foundational input for other tools that require an understanding of Rust code organization.