# Tool: `list_directory`

## Category: File System Management

## Purpose
Lists the names of files and subdirectories directly within a specified directory path. This tool is essential for exploring the project's structure, understanding directory contents, and identifying potential targets for further operations.

## Use Cases
*   Discovering files and subdirectories within a given path.
*   Exploring project structure to understand its organization.
*   Finding specific file types (e.g., all `.rs` files in a directory).
*   Verifying the contents of a newly created or modified directory.

## Key Parameters

*   **`dir_path`** (string, **required**):
    *   The path to the directory to list. Relative paths are resolved against the current working directory.
*   **`file_filtering_options`** (object, optional):
    *   `respect_gemini_ignore` (boolean, optional, defaults to `true`): Whether to respect `.geminiignore` patterns.
    *   `respect_git_ignore` (boolean, optional, defaults to `true`): Whether to respect `.gitignore` patterns (only in Git repositories).
*   **`ignore`** (array of strings, optional):
    *   A list of glob patterns to ignore when listing files.

## Examples

**1. List all items in the current directory:**
```
list_directory(dir_path=".")
```

**2. List items in a specific directory, ignoring .gitignored files:**
```
list_directory(dir_path="src/my_module", file_filtering_options={"respect_git_ignore": true})
```

**3. List items in a directory, ignoring specific glob patterns:**
```
list_directory(dir_path="docs", ignore=["*.tmp", "*.bak"])
```