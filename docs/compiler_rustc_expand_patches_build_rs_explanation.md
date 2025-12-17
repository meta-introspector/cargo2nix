# `compiler/rustc_expand_patches/build.rs` Explanation

This `build.rs` script is responsible for automating various code modifications within the project, primarily targeting Rust source files and `Cargo.toml` manifests. It operates by reading configuration files (TOML format) from a `patches` directory (relative to `CARGO_MANIFEST_DIR`) and applying a series of defined "edit jobs" to specified target files.

## Functionality Overview

The script processes `.toml` files found in its `patches` directory. Each TOML file can define one or more `EditJob`s, which are then applied sequentially. It uses `syn` for robust Abstract Syntax Tree (AST) manipulation of Rust code and `toml_edit` for precise, formatted modifications to `Cargo.toml` files.

## Supported Edit Job Types

The `EditJob` enum defines the various types of modifications the script can perform:

1.  **`AddUse`**:
    *   **Purpose**: Inserts a `use` statement into a Rust file.
    *   **Details**: Prevents duplicate `use` statements. Can insert at the start, end, or after a specified existing `use` path.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `path`: The `use` statement string (e.g., `"use super::error::AppError;"`).
        *   `position`: Optional. `"start"`, `"end"`, or `{"AfterUsePath": "some::path"}`.

2.  **`RemoveUse`**:
    *   **Purpose**: Removes a specific `use` statement from a Rust file.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `use_path`: The path within the `use` statement (e.g., `"rocksdb::{Options, DB}"`).

3.  **`RemoveFunction`**:
    *   **Purpose**: Deletes a function from a Rust file by its name.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `function_name`: The identifier of the function to remove.

4.  **`ReplaceExpression`**:
    *   **Purpose**: Replaces a specific code snippet (expression) within a named function in a Rust file.
    *   **Details**: The replacement is performed on the AST level.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `function_name`: The function containing the expression to be replaced.
        *   `old_code_snippet`: The exact code snippet to find and replace.
        *   `new_code_snippet`: The code snippet to insert as a replacement.

5.  **`AddFunction`**:
    *   **Purpose**: Adds an entirely new function to a Rust file.
    *   **Details**: Checks if a function with the same name already exists to prevent duplicates.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `function_code`: The full source code of the function to add.

6.  **`AddItem`**:
    *   **Purpose**: Adds any arbitrary Rust item (e.g., `struct`, `enum`, `const`, `impl` block, `mod`) to a Rust file.
    *   **Details**: Attempts to detect and prevent adding duplicate items based on their names where applicable.
    *   **Configuration Fields**:
        *   `target_file`: Path to the Rust file.
        *   `item_code`: The full source code of the item to add.

7.  **`ReplaceFileContent`**:
    *   **Purpose**: Overwrites the entire content of a target file with a provided string.
    *   **Configuration Fields**:
        *   `target_file`: Path to any file.
        *   `new_content`: The string content to write to the file.

8.  **`ReplaceFileContentFromFile`**:
    *   **Purpose**: Overwrites the entire content of a target file with the content from another source file.
    *   **Configuration Fields**:
        *   `target_file`: Path to any file to be overwritten.
        *   `source_file`: Path to the file whose content will be used.

9.  **`RunSearch`**:
    *   **Purpose**: Executes a specified shell command (e.g., a `ripgrep` command) and optionally captures its output.
    *   **Details**: This job type does not modify source code directly but is useful for diagnostic or reporting purposes during the build process.
    *   **Configuration Fields**:
        *   `command`: The shell command string to execute.
        *   `output_file`: Optional path to a file where `stdout` and `stderr` of the command will be saved.

10. **`RemoveCargoDependency`**:
    *   **Purpose**: Removes a specified dependency entry from a `Cargo.toml` file.
    *   **Details**: Can remove dependencies from `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, `[workspace.dependencies]`, `[patch.crates-io]`, and entries from `[workspace.members]`.
    *   **Configuration Fields**:
        *   `target_file`: Path to the `Cargo.toml` file.
        *   `package_name`: The name of the dependency package or workspace member to remove.

## Workflow

1.  The script starts by looking for a `patches` directory in `CARGO_MANIFEST_DIR`.
2.  It then scans this `patches` directory for all `.toml` files, sorting them by name.
3.  For each TOML configuration file, it parses the `EditJobConfig` which contains a list of `EditJob`s.
4.  `RunSearch` jobs are executed first.
5.  Other `EditJob`s are grouped by their `target_file`.
6.  For each `target_file`, if it's a `Cargo.toml`, `RemoveCargoDependency` jobs are processed using `toml_edit`.
7.  For Rust files, the script reads the file, parses it into a `syn::File` AST, applies the relevant `EditJob`s (e.g., `AddUse`, `RemoveFunction`, `ReplaceExpression`, `AddFunction`, `AddItem`, `RemoveUse`), and then unparses and pretty-prints the modified AST back to the file.
8.  `ReplaceFileContent` and `ReplaceFileContentFromFile` jobs directly overwrite the target file's content.

This build script provides a powerful and flexible mechanism for applying programmatic patches and transformations to the codebase, ensuring consistency and enabling complex refactoring operations.