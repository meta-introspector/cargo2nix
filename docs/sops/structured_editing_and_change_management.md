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

### 3.4 Leveraging Structured Build Output

The `cargo build` command, when invoked with `--message-format=json`, provides a structured, machine-readable output in JSON Lines format. This output is invaluable for detailed analysis of the build process, especially when combined with semantic patching and change management. It allows for programmatically inspecting compiler messages, artifact paths, and build script executions, offering deeper insights than traditional console output.

#### 3.4.1 Generating Structured Build Output

To capture the JSON output, redirect `stdout` to a file. Errors and warnings, often in plain text, can be redirected to a separate file for review.

```bash
cargo build --message-format=json > build.json 2> err.txt
```

-   `build.json`: Contains one JSON object per line, detailing various build events (e.g., `compiler-artifact`, `compiler-message`, `build-script-executed`).
-   `err.txt`: Captures `stderr`, which typically includes human-readable warnings and errors not part of the JSON stream.

#### 3.4.2 Analyzing Structured Output with `jq`

The `jq` command-line JSON processor is highly effective for filtering, transforming, and summarizing the `build.json` output.

**Basic Inspection:**
To view individual JSON objects, you can pipe the output through `jq .`:

```bash
head build.json | jq .
```

**Filtering by Message Type (`reason`):**
Messages are categorized by a `reason` field. To focus on specific types, such as `compiler-artifact` (compiled crates) or `compiler-message` (warnings/errors):

```bash
jq -c 'select(.reason == "compiler-artifact")' build.json
jq -c 'select(.reason == "compiler-message")' build.json
```

**Grouping and Summarizing:**
To understand the distribution of messages, you can group them by relevant fields, like `reason` and `target.name` (the name of the compiled crate or build target). This helps identify frequently occurring messages or patterns.

```bash
jq -c '{reason: .reason, target_name: (.target.name // "N/A")}' build.json | sort | uniq -c | sort -nr
```
This command extracts the `reason` and `target_name`, groups identical combinations, counts their occurrences, and sorts by frequency. The `// "N/A"` handles cases where `target.name` might be absent.

**Extracting Specific Information:**
You can extract any field or combination of fields for further processing. For example, to list all compiled artifact filenames:

```bash
jq -r 'select(.reason == "compiler-artifact") | .filenames[]' build.json
```

#### 3.4.3 Use Cases in Structured Editing and Change Management

-   **Impact Analysis of Patches**: After applying semantic patches, use the structured output to verify that expected artifacts are built, or to detect new `compiler-message` entries (warnings/errors) that indicate unintended side effects.
-   **Identifying Problematic Areas**: Frequent `compiler-message` entries for a particular `package_id` or `target_name` can highlight modules that are sensitive to changes or require refactoring.
-   **Automated Reporting**: Integrate `jq` commands into scripts to generate automated reports on build health, dependency changes, or compiler diagnostics, providing actionable insights into the codebase's state.
-   **Refinement of Semantic Patches**: The detailed information from the JSON output can inform the creation of more precise semantic patches by pinpointing exact locations or types of modifications needed.

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
