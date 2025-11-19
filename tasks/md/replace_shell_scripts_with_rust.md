name = "Replace Shell Scripts with Rust"
description = "Replace existing Python and shell scripts (e.g., `ensure_non_vendored_workspaces.sh`, `generate_repolist.sh`, `tools/repo_manager`) with Rust implementations using traits and functors for improved maintainability, performance, and type safety."
status = "in-progress"
depends_on = []

## Detailed Translation Plan for Python and Shell Scripts

This section outlines the plan for translating remaining Python and shell scripts in the `tools/` directory into Rust, leveraging a trait-based architecture for modularity, testability, and maintainability.

### Python Scripts Translation Plan:

1.  **`analyze_layer0_usage.py`**:
    *   **Task**: Translate `analyze_layer0_usage.py` to Rust.
    *   **Plan**:
        *   Define a `Layer0Analyzer` trait and `RealLayer0Analyzer` struct in `tools/cargo-repo-sync/src/analysis/layer0_analyzer.rs`.
        *   Implement the logic for finding the most used Layer 0 module.
        *   Integrate into `run_analyze_command` in `tools/cargo-repo-sync/src/cli/run_commands.rs`.

2.  **`find_non_vendored.py`**:
    *   **Task**: Translate `find_non_vendored.py` to Rust.
    *   **Plan**:
        *   Define a `NonVendoredModuleFinder` trait and `RealNonVendoredModuleFinder` struct in `tools/cargo-repo-sync/src/analysis/non_vendored_module_finder.rs`.
        *   Implement the logic for finding and counting non-vendored modules.
        *   Integrate into `run_analyze_command` in `tools/cargo-repo-sync/src/cli/run_commands.rs`.

3.  **`generate_config_patches.py`**:
    *   **Task**: Translate `generate_config_patches.py` to Rust.
    *   **Plan**:
        *   Define a `CargoConfigPatcher` trait and `RealCargoConfigPatcher` struct in `tools/cargo-repo-sync/src/analysis/cargo_config_patcher.rs`.
        *   Implement the logic for generating `[patch.crates-io]` entries based on `Cargo.lock` and `Cargo.toml`.
        *   Integrate into `run_analyze_command` in `tools/cargo-repo-sync/src/cli/run_commands.rs`.

4.  **`generate_deps_from_names_txt.py`**:
    *   **Task**: Translate `generate_deps_from_names_txt.py` to Rust.
    *   **Plan**:
        *   Create a new module in `tools/cargo-repo-sync/src/` for this functionality.
        *   Implement logic to read a names file and generate dependency entries.

5.  **`generate_workspace_deps_from_submodules.py`**:
    *   **Task**: Translate `generate_workspace_deps_from_submodules.py` to Rust.
    *   **Plan**:
        *   Create a new module in `tools/cargo-repo-sync/src/` for this functionality.
        *   Implement logic to generate workspace dependencies from submodules.

6.  **`merge_depgraph_data.py`**:
    *   **Task**: Translate `merge_depgraph_data.py` to Rust.
    *   **Plan**:
        *   Define a `DepGraphDataMerger` trait and `RealDepGraphDataMerger` struct in `tools/cargo-repo-sync/src/analysis/dep_graph_data_merger.rs`.
        *   Implement the logic for merging layer data and usage counts.
        *   Integrate into `run_analyze_command` in `tools/cargo-repo-sync/src/cli/run_commands.rs`.

7.  **`process_tt_txt.py`**:
    *   **Task**: Translate `process_tt_txt.py` to Rust.
    *   **Plan**:
        *   Create a new module in `tools/cargo-repo-sync/src/` for this functionality.
        *   Implement logic to process `tt.txt` files.

8.  **`remove_submodule_workspaces.py`**:
    *   **Task**: Translate `remove_submodule_workspaces.py` to Rust.
    *   **Plan**:
        *   Define a `WorkspaceRemover` trait and `RealWorkspaceRemover` struct in `tools/cargo-repo-sync/src/analysis/workspace_remover.rs`.
        *   Implement the logic for removing workspace sections from `Cargo.toml` files in submodules.
        *   Integrate into `run_analyze_command` in `tools/cargo-repo-sync/src/cli/run_commands.rs`.

9.  **`update_cargo_toml.py`**:
    *   **Task**: Translate `update_cargo_toml.py` to Rust.
    *   **Plan**:
        *   Create a new module in `tools/cargo-repo-sync/src/` for this functionality.
        *   Implement logic to update `Cargo.toml` files.

### Shell Scripts Translation Plan:

For shell scripts, the translation often involves reimplementing the shell commands using Rust's `std::process::Command` or finding equivalent Rust crates.

1.  **`ensure_non_vendored_workspaces.sh`**:
    *   **Task**: Translate `ensure_non_vendored_workspaces.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely checking for non-vendored workspaces).
        *   Implement equivalent checks and logic in Rust.

2.  **`ensure_submodule_workspaces.sh`**:
    *   **Task**: Translate `ensure_submodule_workspaces.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely ensuring submodules are treated as workspaces).
        *   Implement equivalent checks and logic in Rust.

3.  **`execute_gh_plan.sh`**:
    *   **Task**: Translate `execute_gh_plan.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely executing a plan using the `gh` CLI tool).
        *   Implement equivalent `gh` CLI calls using Rust's `std::process::Command` or a `gh` Rust API if available.

4.  **`extract_packages.sh`**:
    *   **Task**: Translate `extract_packages.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely extracting package names or paths).
        *   Implement equivalent file parsing and string manipulation in Rust.

5.  **`find_cargo2nix_executables.sh`**:
    *   **Task**: Translate `find_cargo2nix_executables.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely finding executables related to `cargo2nix`).
        *   Implement equivalent file system traversal and executable detection in Rust.

6.  **`fix2.sh`**:
    *   **Task**: Translate `fix2.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely a general-purpose fix-it script).
        *   Implement equivalent logic in Rust.

7.  **`generate_repolist.sh`**:
    *   **Task**: Translate `generate_repolist.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely generating a list of repositories).
        *   Implement equivalent logic in Rust, possibly using `git2` crate for Git operations.

8.  **`generate_workspace_deps.sh`**:
    *   **Task**: Translate `generate_workspace_deps.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely generating workspace dependencies).
        *   Implement equivalent logic in Rust.

9.  **`process_repolist.sh`**:
    *   **Task**: Translate `process_repolist.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely processing a list of repositories).
        *   Implement equivalent logic in Rust.

10. **`process_repos.sh`**:
    *   **Task**: Translate `process_repos.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely processing multiple repositories).
        *   Implement equivalent logic in Rust.

11. **`run_dependency_generation.sh`**:
    *   **Task**: Translate `run_dependency_generation.sh` to Rust.
    *   **Plan**:
        *   Analyze the shell script's functionality (likely orchestrating dependency generation).
        *   Implement equivalent orchestration logic in Rust.
