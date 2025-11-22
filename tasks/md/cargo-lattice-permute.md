# Feature Permutation Builder Crate Plan

## Goal
To dynamically test various feature combinations of the `git-wrapper-lib` crate and measure their impact on build times and binary size, replacing the previous approach of creating multiple static test crates.

## Rationale
The previous strategy of creating individual test crates for each feature permutation proved unmanageable and led to workspace clutter. A dynamic "Feature Permutation Builder" crate will streamline this process, allowing for flexible and automated testing of feature interactions.

## Implementation Plan

### 1. Create `tools/feature-permutation-builder` directory and `Cargo.toml`
*   Create a new directory: `tools/feature-permutation-builder`
*   Create `Cargo.toml` within this directory with the following minimal content:
    ```toml
    [package]
    name = "feature-permutation-builder"
    version = "0.1.0"
    edition = "2021"
    publish = false

    [dependencies]
    git-wrapper-lib = { path = "../git-wrapper-lib" }
    # Add other dependencies as needed for the builder logic (e.g., `clap` for CLI args, `serde` for config)

    [build-dependencies]
    # Dependencies for build.rs, e.g., `serde_json`, `walkdir`
    ```

### 2. Add `feature-permutation-builder` to the root workspace `Cargo.toml`
*   Modify the `[workspace]` section in the root `Cargo.toml` to include the new crate:
    ```toml
    [workspace]
    members = [
        # ... existing members ...
        "tools/feature-permutation-builder",
    ]
    ```

### 3. Implement `build.rs` to generate test code based on feature flags
*   Create `tools/feature-permutation-builder/build.rs`.
*   This script will:
    *   Read a configuration (e.g., from an environment variable or a file) specifying the desired feature permutations for `git-wrapper-lib`.
    *   Generate Rust source files (e.g., `src/generated_tests.rs`) that instantiate `git-wrapper-lib` with different feature combinations.
    *   The generated code will include calls to `git-wrapper-lib` functions to ensure compilation and basic functionality.
    *   Use `println!("cargo:rerun-if-changed=build.rs");` and `println!("cargo:rerun-if-env-changed=FEATURE_PERMUTATIONS_CONFIG");` to ensure `build.rs` is re-run when relevant inputs change.

### 4. Implement `src/main.rs` to run the generated test
*   Create `tools/feature-permutation-builder/src/main.rs`.
*   This will be the main executable for the builder crate.
*   It will:
    *   Include the generated test code: `include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));`
    *   Provide a CLI interface (e.g., using `clap`) to:
        *   Trigger builds for specific feature permutations.
        *   Measure build times (e.g., using `std::time::Instant`).
        *   Potentially measure binary sizes.
        *   Execute the generated tests.
        *   Collect and report results.

### 5. Remove the previously created static test crates
*   Delete the following directories and their contents:
    *   `workspaces/git-wrapper-lib-test-no-features`
    *   `workspaces/git-wrapper-lib-test-default-features`
    *   `workspaces/git-wrapper-lib-test-all-features`
    *   `workspaces/git-wrapper-lib-test-git2-only`
    *   `workspaces/git-wrapper-lib-test-anyhow-only`
    *   `workspaces/git-wrapper-lib-test-serde-only`
    *   `workspaces/git-wrapper-lib-test-git2-anyhow-serde`
*   Remove their corresponding entries from the `[workspace.members]` section in the root `Cargo.toml`.

## Execution and Reporting

### 1. Execute build and time measurements
*   The `feature-permutation-builder` crate will be invoked with different configurations to build `git-wrapper-lib` with various feature sets.
*   Build times will be captured using `std::time::Instant` or external `time` command.
*   Binary sizes will be obtained from the build artifacts.

### 2. Document results in `tasks/md/feature_lattice_build_results.md`
*   Create a new Markdown file: `tasks/md/feature_lattice_build_results.md`.
*   This file will contain a structured report of:
    *   Each feature permutation tested.
    *   Corresponding build times.
    *   Corresponding binary sizes.
    *   Any notable observations or conclusions.

This plan will enable efficient and comprehensive testing of `git-wrapper-lib`'s feature eigenforms.