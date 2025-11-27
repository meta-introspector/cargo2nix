# Tool: git_graph_db.rs

## Description
This tool builds an in-memory database representing the Git graph of repositories. It stores information about each Git repository, including its path, URL, branch, submodules, and remotes. This database is fundamental for analyzing the interconnections and structure of the project's Git ecosystem.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::collections::HashMap`

## Notes
The `GitRepo` struct captures key details of a Git repository. This tool effectively creates a snapshot of the Git repository graph, which can be used for various analyses such as dependency mapping, submodule management, and architectural understanding.