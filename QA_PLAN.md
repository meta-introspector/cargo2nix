# QA Plan for Unstaged Changes

This QA plan outlines the steps to verify the integrity and functionality of the unstaged changes, covering dependency updates, tooling adjustments, and new feature development.

## 1. Dependency Resolution and Build Integrity

*   **Objective:** Ensure all dependencies are correctly resolved and the project builds successfully with the updated `Cargo.lock`, `Cargo.toml`, and `flake.lock`.
*   **Steps:**
    1.  Run `nix develop` to ensure the Nix environment can be set up without errors.
    2.  Execute `cargo check --workspace` to verify that all crates in the workspace compile without type errors.
    3.  Execute `cargo build --workspace` to ensure all crates in the workspace build successfully.
    4.  Verify that `cargo build` and `cargo check` respect the changes in `Cargo.toml` (e.g., the new `monster_multivector` and `monster_traits` crates are recognized).

## 2. Submodule Integrity

*   **Objective:** Verify that the numerous submodule updates (indicated by "modified content") haven't introduced regressions or inconsistencies.
*   **Steps:**
    1.  After `nix develop`, run `git submodule status` to confirm that all submodules are in a consistent state (i.e., no unexpected `-dirty` indicators after a fresh `nix develop` or `git submodule update`).
    2.  If the changes were intended to be propagated (e.g., through a `git submodule update --remote`), run that command and then rebuild the project to ensure compatibility.

## 3. `doit.sh` Functionality

*   **Objective:** Ensure the updated `doit.sh` script correctly invokes the `gemini.js` CLI.
*   **Steps:**
    1.  Execute `bash ./doit.sh` (or `nix develop . -c bash ./doit.sh` if it's meant to be run within the Nix environment).
    2.  Verify that the `gemini.js` CLI is executed and produces expected output (e.g., a help message or a simple interaction).
    3.  Confirm that the script does not produce any unexpected errors related to pathing or missing dependencies.

## 4. Internal Tooling Functionality

*   **Objective:** Verify that refactored internal tools (`cargo-edit-lib`, `cargo-llm-bootstrap`, `cargo-submodule-tool-lib`, `feature-permutation-builder`, `git-wrapper-lib`, `syn-adapter-lib`) continue to function as expected.
*   **Steps:**
    1.  Run existing unit and integration tests for these tools if available (e.g., `cargo test -p cargo-edit-lib`).
    2.  Execute known commands or workflows that rely on these tools (e.g., any `cargo submodule` commands, or `feature-permutation-builder` tasks) and observe their behavior for correctness.
    3.  Specifically verify the `Execv` trait re-export in `git-wrapper-lib` by ensuring any code that uses it compiles and runs correctly.

## 5. New Feature Development (monster_multivector, monster_traits, tasks/toml)

*   **Objective:** Perform basic verification of the new `monster_multivector` and `monster_traits` crates and the `tasks/toml` definitions.
*   **Steps:**
    1.  Ensure `crates/monster_multivector` and `crates/monster_traits` are part of the workspace build (covered by Step 1.3).
    2.  If any example usage or tests are defined within these new crates, execute them (e.g., `cargo test -p monster_multivector`).
    3.  Review the contents of the `tasks/toml/*.toml` files to ensure they are well-formed and logically consistent with their intended purpose.

## 6. Overall System Stability

*   **Objective:** Conduct a general sanity check to ensure no unexpected regressions have been introduced across the wider project.
*   **Steps:**
    1.  Run the main project tests (`cargo test --workspace`).
    2.  If there are any known critical workflows or demonstration commands for the project, execute them to ensure they still work.

By following these steps, we can ensure a comprehensive quality assurance process for the unstaged changes.