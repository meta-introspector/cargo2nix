# Tool: enhanced_content_mapper.rs

## Description
This tool maps various content entities within the project, associating content hashes with names, versions, and their originating Git repositories. It provides a comprehensive view of content provenance and uniqueness, crucial for managing a large and complex codebase.

## Usage
[How to use the tool, including any command-line arguments or configuration.]

## Dependencies
- `std::fs`
- `std::process::Command`
- `std::collections::HashMap`

## Notes
The `ContentEntry` struct, similar to its "_fixed" counterpart, captures `content_hash`, `name`, `version`, and `git_repo`. This tool is fundamental for content-addressable storage and managing content integrity.