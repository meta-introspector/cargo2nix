# Tool: `glob`

## Category: Codebase Search & Analysis

## Purpose
Efficiently finds files matching specific glob patterns. This tool is ideal for quickly locating files based on their name or path structure, especially in large codebases, and returns absolute paths sorted by modification time (newest first).

## Use Cases
*   Finding all source code files of a specific type (e.g., all `.rs` files).
*   Locating all documentation files in a project.
*   Identifying configuration files across different modules.
*   Preparing lists of files for batch processing or analysis.

## Key Parameters

*   **`pattern`** (string, **required**):
    *   The glob pattern to match against (e.g., `'src/**/*.rs'`, `'docs/*.md'`, `'**/*.toml'`).
*   **`dir_path`** (string, optional):
    *   The absolute path to the directory to search within. If omitted, searches the root directory.
*   **`case_sensitive`** (boolean, optional, defaults to `false`):
    *   Whether the search should be case-sensitive.
*   **`respect_gemini_ignore`** (boolean, optional, defaults to `true`):
    *   Whether to respect `.geminiignore` patterns when finding files.
*   **`respect_git_ignore`** (boolean, optional, defaults to `true`):
    *   Whether to respect `.gitignore` patterns when finding files. Only available in Git repositories.

## Examples

**1. Find all Rust source files in the `src` directory and its subdirectories:**
```
glob(pattern="src/**/*.rs")
```

**2. Find all Markdown files in the `docs` directory, case-sensitively:**
```
glob(pattern="docs/*.md", case_sensitive=true)
```

**3. Find all `Cargo.toml` files anywhere in the project, ignoring `.gitignore` rules:**
```
glob(pattern="**/Cargo.toml", respect_git_ignore=false)
```