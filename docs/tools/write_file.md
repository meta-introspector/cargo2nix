# Tool: `write_file`

## Category: File System Management

## Purpose
Writes content to a specified file in the local filesystem. This tool is used to create new files or overwrite the entire content of existing files.

## Use Cases
*   Generating new source code files (e.g., Rust modules, Nix expressions).
*   Creating or updating configuration files (e.g., `Cargo.toml`, `.envrc`).
*   Writing documentation (e.g., new Markdown files).
*   Saving generated reports or output from analysis tools.
*   Creating temporary scripts or data files.

## Key Parameters

*   **`file_path`** (string, **required**):
    *   The path to the file to write to. If the file does not exist, it will be created. If it exists, its contents will be completely overwritten. Relative paths are resolved against the current working directory.
*   **`content`** (string, **required**):
    *   The complete content to write into the file.

## Examples

**1. Create a new Rust module with some basic code:**
```python
write_file(
    file_path="src/new_module.rs",
    content="""
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
"""
)
```

**2. Update a configuration file with new settings:**
```python
write_file(
    file_path="config.toml",
    content="""
[database]
host = "localhost"
port = 5432
username = "admin"
password = "supersecretpassword"

[server]
port = 8080
"""
)
```

**3. Create a new markdown documentation file:**
```python
write_file(
    file_path="docs/new_feature_guide.md",
    content="# New Feature Guide\n\nThis document describes how to use the exciting new feature."
)
```
