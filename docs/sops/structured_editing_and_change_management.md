# SOP: Structured Editing and Change Management

## 1. Purpose

This Standard Operating Procedure (SOP) documents the methodology for applying structured, semantic code modifications and managing changes across the project's Rust codebase and Cargo configurations. It leverages TOML-based patch definitions processed by a custom `build.rs` script within the `rustc_expand_patches` crate, enabling declarative and repeatable alterations to source files and project metadata.

## 2. Key Concepts

### 2.1 Semantic Patches
Instead of traditional line-based diffs, semantic patches describe desired code changes at a higher level of abstraction (e.g., "remove a dependency," "add a `use` statement," "comment out a function"). These patches are defined in TOML files.

### 2.2 `rustc_expand_patches` Crate
This crate (`submodules/rust/compiler/rustc_expand_patches/`) houses a `build.rs` script that acts as the core "patch engine." This `build.rs` script:
- Reads TOML patch definition files from its `patches/` subdirectory.
- Parses these definitions into structured `EditJob` enums.
- Applies the specified transformations to target files (Rust source, `Cargo.toml`).

### 2.3 Edit Job Configuration (`EditJobConfig`, `EditJob` variants)
Patches are defined in TOML files using an array of `EditJob` structures. Each `EditJob` specifies a `type` of modification and associated `details`.

#### Supported `EditJob` Types:

-   **`AddUse`**: Adds a `use` statement to a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `path`: The `use` statement path (e.g., `"std::collections::HashMap;"`).
    -   `position` (optional): `"start"`, `"end"`, or `"after_use_path(<existing_use>)"`.
-   **`RemoveUse`**: Removes a `use` statement from a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `use_path`: The exact `use` statement path to remove (e.g., `"rocksdb::{Options, DB}"`).
-   **`RemoveFunction`**: Removes a function from a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `function_name`: Name of the function to remove.
-   **`ReplaceExpression`**: Replaces a code snippet within a function in a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `function_name`: Function containing the expression.
    -   `old_code_snippet`: The exact code to be replaced.
    -   `new_code_snippet`: The replacement code.
-   **`AddFunction`**: Adds a new function to a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `function_code`: Full code of the function.
-   **`AddItem`**: Adds any top-level Rust `Item` (struct, enum, const, etc.) to a Rust file.
    -   `target_file`: Path to the Rust file.
    -   `item_code`: Full code of the item.
-   **`ReplaceFileContent`**: Replaces the entire content of a file.
    -   `target_file`: Path to the file.
    -   `new_content`: New content for the file.
-   **`ReplaceFileContentFromFile`**: Replaces file content from another source file.
    -   `target_file`: Path to the file.
    -   `source_file`: Path to the file whose content will be used.
-   **`RunSearch`**: Executes a shell command (e.g., `ripgrep`) and optionally saves its output.
    -   `command`: The shell command string.
    -   `output_file` (optional): Path to save `stdout`/`stderr`.
-   **`RemoveCargoDependency`**: Removes a specified package dependency or workspace member from a `Cargo.toml` file.
    -   `target_file`: Path to the `Cargo.toml` file.
    -   `package_name`: Name of the package to remove (e.g., `"rocksdb"`, `"librocksdb-sys"`, or a workspace member path like `"submodules/rocksdb"`). This applies to `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, `[workspace.dependencies]`, `[patch.crates-io]`, and `[workspace].members` arrays.

## 3. Workflow

### 3.1 Defining Patches
1.  Create a new `.toml` file (e.g., `my_feature_patch.toml`) in the `submodules/rust/compiler/rustc_expand_patches/patches/` directory.
2.  Define one or more `EditJob` entries within this TOML file, specifying the `type` and `details` for each desired modification.
    -   **Important:** `target_file` paths within patch files **must be relative to the `CARGO_MANIFEST_DIR` of the `rustc_expand_patches` crate.** For files in the main project root, this typically means `../../../../<path_from_workspace_root>`.

### 3.2 Applying Patches
1.  Navigate to the project's root directory.
2.  Execute a Cargo command that triggers the `build.rs` script:
    ```bash
    cargo check
    # or
    cargo build
    ```
3.  The `rustc_expand_patches/build.rs` script will automatically:
    -   Discover all `.toml` files in its `patches/` directory.
    -   Process each `EditJob` defined within these files.
    -   Apply the specified modifications to the target files.
    -   `eprintln!` statements in `build.rs` provide debug information on the patching process.

### 3.3 Verification
1.  After `cargo check` (or `cargo build`) completes, inspect the affected target files manually to confirm that the changes have been applied correctly.
2.  Run `git status` to observe modified files.
3.  Re-run `cargo check` (or `cargo build`) to ensure no new compilation errors or warnings are introduced by the patches.

## 4. Change Management and Iteration

-   **Atomic Patches**: Each `.toml` patch file should ideally target a single logical change or a closely related set of changes.
-   **Iterative Application**: The patching process is iterative. Define a patch, apply it via `cargo check`, observe the results (including new compilation errors/warnings), refine the patch or create new patches, and repeat.
-   **Knowledge Base Integration**: The output of `RunSearch` tasks should be considered as input for a knowledge base, helping to identify common issues and inform future patch development. Future enhancements will integrate this caching mechanism directly.

## 5. Example: Disabling `rocksdb` (Recap)

The process of disabling `rocksdb` involved:
1.  **Search:** A `RunSearch` job identified all `Cargo.toml` files that depend on `rocksdb` or `librocksdb-sys`, and also identified `rocksdb` and `librocksdb-sys` as workspace members.
    (Example `search_rocksdb_and_disable.toml` snippet with `RunSearch`):
    ```toml
    [[edits]]
    type = "RunSearch"
    command = "rg -l \"rocksdb\" --glob \"*.toml\" --glob \"!target/*\""
    ```
2.  **Removal from `Cargo.toml`s:** `RemoveCargoDependency` jobs were defined to remove `librocksdb-sys` from `submodules/rocksdb/Cargo.toml` and to remove `rocksdb` and `librocksdb-sys` from relevant dependency sections and `members` array in the root `Cargo.toml`.
    (Example `search_rocksdb_and_disable.toml` snippet with `RemoveCargoDependency`):
    ```toml
    [[edits]]
    type = "RemoveCargoDependency"
    target_file = "../../../../submodules/rocksdb/Cargo.toml"
    package_name = "librocksdb-sys"

    [[edits]]
    type = "RemoveCargoDependency"
    target_file = "../../../../Cargo.toml"
    package_name = "submodules/rocksdb" # Example for workspace member removal
    ```
3.  **Removal from Rust Source:** A `RemoveUse` job was defined to remove the `use rocksdb::{Options, DB};` statement from `tools/rust-src-scanner/src/main.rs`.
    (Example `search_rocksdb_and_disable.toml` snippet with `RemoveUse`):
    ```toml
    [[edits]]
    type = "RemoveUse"
    target_file = "../../../../tools/rust-src-scanner/src/main.rs"
    use_path = "rocksdb::{Options, DB}"
    ```

This iterative process of defining patches, applying them, and refining based on build feedback is central to this structured editing approach.
