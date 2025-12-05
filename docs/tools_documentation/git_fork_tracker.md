# Tool: git_fork_tracker.rs

## Description
This tool tracks Git fork relationships, building a comprehensive view of how different modules relate to their original repositories and forks. It manages a map of Git modules and their fork relationships, providing insights into code lineage and divergence.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::collections::HashMap`

## Notes
The `GitForkTracker` struct stores `modules` (mapping module names to `GitModule` details) and `fork_relationships` (mapping a fork URL to its original URL). This tool is crucial for managing the complex network of Git forks and submodules.