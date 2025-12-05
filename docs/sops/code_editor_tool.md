# SOP: Using the `code_editor` Tool

This document outlines the purpose, installation, usage, and configuration of the `code_editor` tool, located in the `submodules/claude-code-mux/code_editor/` directory.

## 1. Purpose

The `code_editor` is a Rust-based command-line utility designed to automate structured modifications within Rust source code files. It processes TOML-formatted configuration files that specify a series of "edit jobs," allowing for precise and repeatable changes such as adding/removing `use` statements, functions, or replacing code snippets. This tool is invaluable for refactoring, applying consistent code patterns, or performing bulk edits across a codebase.

## 2. Installation/Availability

The `code_editor` tool is compiled and installed from its source directory within the project's Git submodules. It is expected to be available in your Cargo binary path after successful compilation.

To compile and install (if not already done):

```bash
cd submodules/claude-code-mux/code_editor
cargo install --path .
```

After installation, the `code_editor` executable will typically be located at `~/.cargo/bin/code_editor`. Ensure that `~/.cargo/bin` is in your system's `PATH` environment variable.

## 3. Usage

The `code_editor` tool is invoked with the `--config-path` argument, specifying either a single TOML configuration file or a directory containing multiple `.toml` files.

```bash
# To run a single edit job configuration file
code_editor --config-path path/to/your_edit_job_config.toml

# To run multiple edit job configuration files within a directory
code_editor --config-path path/to/your/config_files_directory
```

The tool will process each `EditJob` defined in the specified configuration(s) and apply the changes to the target Rust source files.

## 4. Configuration (`EditJobConfig` TOML Structure)

`code_editor` uses TOML files to define the modifications it should perform. The root of the TOML file must be an array named `edits`, where each element is an `EditJob` specifying a particular type of code modification.

**Basic `EditJobConfig` Structure:**

```toml
# example_config.toml
[[edits]]
# ... first EditJob details ...

[[edits]]
# ... second EditJob details ...
```

---

### `EditJob` Variants (Types of Modifications)

Each `EditJob` is an enum identified by a `type` field. Below are the supported types and their required parameters:

#### `type = "AddUse"`: Add a `use` statement.

This job adds a `use` declaration to a specified Rust file.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify (e.g., `"src/my_module/mod.rs"`).
*   **`path`**: `String` - The `use` statement itself (e.g., `"use tokio::time::Duration;"`).
*   **`position`** (Optional): `String` - Specifies where to insert the `use` statement.
    *   `"start"`: At the beginning of the file.
    *   `"end"`: After the last existing `use` statement (default if omitted).
    *   `"after_use_path"`: After a specific existing `use` statement, identified by its `path` string (e.g., `"use anyhow::Result;"`).

**Example:**

```toml
[[edits]]
type = "AddUse"
target_file = "src/server/mod.rs" # Targets a file like mod.rs.tmp
path = "use anyhow::anyhow;"
position = "end"
```

---

#### `type = "RemoveFunction"`: Remove a function.

This job removes a function with a specified name from a Rust file.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`function_name`**: `String` - The name of the function to remove (e.g., `"health_check"`).

**Example:**

```toml
[[edits]]
type = "RemoveFunction"
target_file = "src/server/handlers.rs" # Targets a file like handlers_full.rs.tmp
function_name = "health_check"
```

---

#### `type = "ReplaceExpression"`: Replace a specific code snippet within a function.

This job finds and replaces an exact code snippet within a specified function in a Rust file.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`function_name`**: `String` - The name of the function containing the expression.
*   **`old_code_snippet`**: `String` - The exact code snippet to be replaced.
*   **`new_code_snippet`**: `String` - The code snippet to replace the old one with.

**Example:**

```toml
[[edits]]
type = "ReplaceExpression"
target_file = "src/server/mod.rs" # Targets a file like mod.rs.tmp
function_name = "shutdown_server"
old_code_snippet = "std::process::exit(0);"
new_code_snippet = "std::process::exit(1);"
```

---

#### `type = "AddFunction"`: Add a new function.

This job adds a new function to a Rust file. The function's code is provided directly in the configuration.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`function_code`**: `String` - The full code of the function to add, typically as a multi-line string.

**Example:**

```toml
[[edits]]
type = "AddFunction"
target_file = "src/server/utils.rs" # Targets a file like utils.rs.tmp
function_code = """
pub fn debug_log_state(state: &AppState) {
    tracing::debug!(\"Current log buffer size: {}\", state.log_state.log_buffer.read().unwrap().len());
}
"""
```

---

#### `type = "AddItem"`: Add any Rust `Item` (struct, enum, constant, etc.).

This job adds a new top-level item (e.g., struct, enum, trait, const) to a Rust file.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`item_code`**: `String` - The full code of the item to add, typically as a multi-line string.

**Example:**

```toml
[[edits]]
type = "AddItem"
target_file = "src/server/state.rs" # Targets a file like state.rs.tmp
item_code = """
#[derive(Debug, Clone, Default)]
pub struct ServerMetrics {
    pub requests_total: u64,
    pub errors_total: u64,
}
"""
```

---

#### `type = "ReplaceFileContent"`: Replace the entire content of a file.

This job overwrites the entire content of a target file with the provided new content. Use with caution.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`new_content`**: `String` - The new content for the file, typically as a multi-line string.

**Example:**

```toml
[[edits]]
type = "ReplaceFileContent"
target_file = "src/server/config_update.rs" # Targets a file like config_update.rs.tmp
new_content = """
// This file has been completely replaced.
use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct NewConfigSchema {
    pub new_field: String,
}
"""
```

---

#### `type = "ReplaceFileContentFromFile"`: Replace file content from another source file.

This job replaces the content of a target file with the content of another specified source file.

*   **`target_file`**: `PathBuf` - Path to the Rust file to modify.
*   **`source_file`**: `PathBuf` - Path to the file whose content will be used as the replacement.

**Example:**

```toml
[[edits]]
type = "ReplaceFileContentFromFile"
target_file = "src/server/error.rs" # Targets a file like error.rs.tmp
source_file = "templates/new_error_handler.rs.template"
```
