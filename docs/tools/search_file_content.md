# Tool: `search_file_content`

## Category: Codebase Search & Analysis

## Purpose
Performs fast, optimized content search within files using `ripgrep`. This tool is preferred over standard `grep` for its performance, advanced features, and automatic output limiting. It is crucial for quickly locating code, strings, or specific patterns across the codebase.

## Use Cases
*   Finding definitions of functions, variables, or types.
*   Locating all occurrences of a specific string or phrase.
*   Searching for code patterns (e.g., `TODO` comments, specific API calls).
*   Debugging by tracing usage of a particular component.
*   Exploring dependencies and cross-references within the project.

## Key Parameters

*   **`pattern`** (string, **required**):
    *   The pattern to search for. By default, treated as a Rust-flavored regular expression. Use `\b` for precise symbol matching (e.g., `\bMyFunction\b`).
*   **`dir_path`** (string, optional):
    *   Directory or file to search. Directories are searched recursively. Relative paths are resolved against the current working directory. Defaults to the current working directory (`.`) if omitted.
*   **`case_sensitive`** (boolean, optional, defaults to `false`):
    *   If `true`, the search is case-sensitive.
*   **`fixed_strings`** (boolean, optional, defaults to `false`):
    *   If `true`, treats the `pattern` as a literal string instead of a regular expression.
*   **`include`** (string, optional):
    *   Glob pattern to filter files to include in the search (e.g., `'*.rs'`, `'src/**/*.toml'`). Recommended for large repositories to reduce noise. Defaults to all files if omitted.
*   **`no_ignore`** (boolean, optional, defaults to `false`):
    *   If `true`, searches all files including those usually ignored (like in `.gitignore`, `build/`, `dist/`, etc.).
*   **`before`** (integer, optional, defaults to `0`):
    *   Show this many lines before each match (equivalent to `grep -B`).
*   **`after`** (integer, optional, defaults to `0`):
    *   Show this many lines after each match (equivalent to `grep -A`).
*   **`context`** (integer, optional, defaults to `0`):
    *   Show this many lines of context around each match (equivalent to `grep -C`).

## Examples

**1. Find all occurrences of "my_function" (case-insensitive) in the current directory:**
```
search_file_content(pattern="\bmy_function\b")
```

**2. Find case-sensitive occurrences of "ERROR:" in all `.log` files in `logs/` directory, showing 2 lines of context:**
```
search_file_content(dir_path="logs/", pattern="ERROR:", include="*.log", case_sensitive=true, context=2)
```

**3. Search for the literal string "struct MyData {" in `src/models.rs`:**
```
search_file_content(dir_path="src/models.rs", pattern="struct MyData {", fixed_strings=true)
```