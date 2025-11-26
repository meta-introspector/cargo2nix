# Tool: `run_shell_command`

## Category: Workflow & Task Management

## Purpose
This tool executes a given shell command as `bash -c <command>`. It is the primary mechanism for interacting with the operating system, running project-specific tools, and performing any task that can be accomplished via the command line.

## Use Cases
*   Building projects (e.g., `cargo build`, `make`).
*   Running tests (e.g., `cargo test`, `pytest`).
*   Installing or managing dependencies (e.g., `nix-shell`, `npm install`).
*   Executing custom scripts (e.g., `./scripts/setup.sh`).
*   Interacting with version control systems (e.g., `git status`, `git add`).
*   Managing background processes.

## Key Parameters

*   **`command`** (string, **required**):
    *   The exact bash command to execute as `bash -c <command>`. Commands can start background processes using `&`.
*   **`description`** (string, optional):
    *   A brief description of the command's purpose for the user. Be specific and concise. Up to 3 sentences for clarity, no line breaks.
*   **`dir_path`** (string, optional):
    *   The path of the directory to run the command in. If not provided, the project root directory is used. Must be a directory within the workspace and must already exist.

## Important Safety Rule

*   **Explain Critical Commands**: Before executing commands that modify the file system, codebase, or system state, you *must* provide a brief explanation of the command's purpose and potential impact. Prioritize user understanding and safety. You should not ask permission to use the tool; the user will be presented with a confirmation dialogue upon use.

## Examples

**1. Build the current Rust project:**
```
run_shell_command(
    command="cargo build",
    description="Build the main project binaries."
)
```

**2. Run a specific test suite from a subdirectory:**
```
run_shell_command(
    command="cargo test --workspace --test my_integration_tests",
    description="Execute integration tests for the workspace.",
    dir_path="crates/my_crate"
)
```

**3. List Git status and recent commits (non-modifying):**
```
run_shell_command(
    command="git status && git log -n 3",
    description="Check the current Git status and view the last 3 commit messages."
)
```