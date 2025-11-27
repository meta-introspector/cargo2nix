# Tool: git_fork_mapper.rs

## Description
This tool maps fork relationships between Git repositories. It identifies a `submodule_path`, its `fork_url`, and its potential `upstream_url`, along with a `git_object`. This is crucial for understanding the origin and evolution of code within a complex project structure involving many forks and submodules.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `ForkRelation` struct captures essential details about a fork. This tool helps in managing and visualizing the branching and merging history across different repositories.