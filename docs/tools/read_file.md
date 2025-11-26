# Tool: `read_file`

## Category: File System Management

## Purpose
Reads and returns the content of a specified file. This tool is fundamental for understanding existing code, configuration, or documentation. It can handle various file types and offers pagination for large text files.

## Use Cases
*   Inspecting the contents of source code files (e.g., `.rs`, `.toml`, `.nix`).
*   Reading configuration files (e.g., `Cargo.toml`, `.env`).
*   Reviewing documentation files (e.g., `.md`).
*   Examining log files for debugging purposes.
*   Paginating through large text files to view specific sections.

## Key Parameters

*   **`file_path`** (string, **required**):
    *   The path to the file to read. Relative paths are resolved against the current working directory.
*   **`limit`** (number, optional):
    *   For text files, this specifies the maximum number of lines to read. Use in conjunction with `offset` for pagination. If omitted, the entire file (up to a default limit) is read.
*   **`offset`** (number, optional):
    *   For text files, this specifies the 0-based line number to start reading from. Requires `limit` to be set. Useful for paginating through large files.

## Examples

**1. Read the entire content of a small file:**
```
read_file(file_path="src/main.rs")
```

**2. Read the first 20 lines of a log file:**
```
read_file(file_path="build.log", limit=20)
```

**3. Read lines 50 to 100 (inclusive) of a documentation file:**
```
read_file(file_path="docs/README.md", offset=50, limit=51)
```