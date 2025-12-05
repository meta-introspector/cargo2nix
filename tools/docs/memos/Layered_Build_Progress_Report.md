# Layered Build Progress Report

This document summarizes the progress made in establishing a reproducible, micro-step-based layered build process for Rust crates, specifically focusing on the challenges encountered and solutions implemented for compiling a single crate from layer 10.

## Key Learnings and Actions Taken:

1.  **Initial `cargo-llm-bootstrap` Argument Parsing Issue:**
    *   **Problem:** The `cargo-llm-bootstrap` tool initially did not recognize `--limit` and `--dry-run` arguments, leading to execution failures.
    *   **Solution:** Modified `cargo-llm-bootstrap/src/main.rs` to add `limit: Option<u32>` to the `Args` struct and integrated its usage to restrict the number of crates processed. The `dry_run` argument was already present.
    *   **Verification:** Successfully performed a dry run for a single crate from layer 10.

2.  **Nix Build Environment and `Cargo.lock` Resolution:**
    *   **Problem:** When attempting to rebuild `cargo-llm-bootstrap` using `nix develop`, the Nix build environment failed to find `Cargo.lock`, even after local generation. This was due to `cargo-llm-bootstrap` being part of a larger workspace, causing `cargo generate-lockfile` to place `Cargo.lock` at the workspace root (`cargo2nix/tools/Cargo.lock`) instead of the crate's directory. The `flake.nix` for `cargo-llm-bootstrap` expected `Cargo.lock` to be local (`./Cargo.lock`).
    *   **Solution:** Copied the workspace `Cargo.lock` to the `cargo-llm-bootstrap` directory and committed it to Git.
    *   **Decision:** User requested to avoid Nix for now, so subsequent builds of `cargo-llm-bootstrap` were performed using `cargo build` directly.

3.  **Compilation Failure due to Unresolved Dependencies:**
    *   **Problem:** Initial attempts to compile a single crate (`icu_collections`) from layer 10 resulted in `rustc` errors (`E0432`, `E0433`) indicating unresolved imports and unlinked crates (e.g., `zerovec`, `potential_utf`, `yoke`). This was because `cargo-llm-bootstrap`'s `compiled_artifacts_map` was empty, as dependencies from lower layers had not yet been compiled.
    *   **Problem:** The sorting logic for `crates_to_process_initial` was incorrect, processing higher layer numbers first (more dependent) instead of lower layer numbers (least dependent).
    *   **Solution (Sorting):** Corrected the sorting order in `cargo-llm-bootstrap/src/main.rs` from descending to ascending by layer number, ensuring that least dependent crates are prioritized. This resulted in `icu_properties_data` being selected as the first crate from layer 10.
    *   **Solution (Temporary Dependency Mocking):** Implemented temporary logic in `cargo-llm-bootstrap/src/main.rs` to:
        *   Parse the `Cargo.toml` of the current crate being processed.
        *   For each direct dependency, use the `crate_name_to_root_map` to find its root path.
        *   Construct a mocked `.rlib` path (e.g., `dep_root_path/target/debug/lib<dep_name>.rlib`).
        *   Insert these mocked paths into `compiled_artifacts_map` before invoking `rustc`. This effectively "mocked" the presence of compiled dependencies.
    *   **Verification:** Successfully compiled `icu_properties_data` from layer 10. The `rustc` command was correctly invoked with all necessary `--extern` flags pointing to the mocked `.rlib` paths, and the `.rlib` output path was correctly identified.

## Current Status and Next Steps:

*   **Achieved:** Successful compilation of a single crate (`icu_properties_data`) from layer 10 using direct `rustc` invocation with mocked dependencies.
*   **Remaining Work:**
    1.  **Full Layered Compilation:** Remove the `--limit 1` and the temporary dependency mocking logic. Implement the full layered build process where `cargo-llm-bootstrap` iterates through all layers, compiling crates in topological order. As each crate compiles, its *actual* `.rlib` path will be added to `compiled_artifacts_map`, making it genuinely available for subsequent dependent crates.
    2.  **Robust State Management:** Re-enable and fully implement the caching and resumability features as described in the updated `plan.md`. This includes saving detailed build records, content hashes, and compiler outputs after each micro-step.
    3.  **Address Warnings:** Clean up the 11 warnings in `cargo-llm-bootstrap` (unused imports, deprecated types, unused variables).
