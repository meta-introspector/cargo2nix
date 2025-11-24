# Compilation Plan: Layer-by-Layer Build with Direct Rustc and Manual Dependency Management

## Goal:
Compile all Rust crates layer by layer, managing dependencies and artifacts manually, without relying on Cargo's built-in dependency resolution for vendored code. Each crate compilation is treated as an atomic 'micro-step' with comprehensive state capture, ensuring full reproducibility, auditability, and resumability. The process prioritizes compiling less dependent crates first to make their artifacts available for dependents.

## Current Steps & Progress:

1.  **Crate Discovery:**
    *   Pre-scan `rust_src_path` for `Cargo.toml` files.
    *   Build a `crate_name_to_root_map` (HashMap<String, PathBuf>) mapping crate names to their root directories.
    *   Gracefully handle malformed `Cargo.toml` files by printing warnings and skipping them.

2.  **Layered Processing:**
    *   Load `layered_graph.json` to determine crate layers.
    *   Filter `crates_to_process_initial` by a specified layer (`--level` argument). Each crate's compilation within a layer is a distinct 'micro-step'.
    *   A topological sort is performed within each layer to ensure correct processing order, guaranteeing that dependencies are built before their dependents.

3.  **Direct `rustc` Compilation:**
    *   For each crate, determine its main source file (`src/lib.rs` or `src/main.rs`).
    *   Run `rustc` directly on the main source file.
    *   **Dependency Resolution:** Construct the precise `rustc` command, including all `--extern <dep_name>=<path_to_dep_rlib>` flags. The `<path_to_dep_rlib>` will point to the *exact hashed `.rlib` files* of its dependencies from previously completed micro-steps, retrieved from the `compiled_artifacts_map`.

4.  **Atomic Build Records & Resumability:**
    *   `compiled_artifacts_map` (HashMap<String, PathBuf>) stores the paths to the compiled `.rlib` files for each successfully built crate.
    *   After each successful micro-step (crate compilation), an immutable "build record" is generated and persisted. This record includes:
        *   **Monster Gödel Index:** A semantic hash (Gödel number) representing the artifact's position within the Monster Group encoding scheme, derived from the sequential index of the file.
        *   Cryptographic hashes (e.g., SHA256) of all input files (source, `Cargo.toml`, build scripts).
        *   The exact `rustc` command executed, including all flags and environment variables.
        *   Cryptographic hashes of all output artifacts (`.rlib`, `.rmeta`, logs).
        *   Full `stdout` and `stderr` from the `rustc` invocation.
        *   References to the build records of its direct dependencies.
    *   **Caching & Resumption:** Before attempting to build a crate, the system checks for an existing build record whose input hashes and Monster Gödel Index match the current state. If found, the micro-step is skipped, and its artifacts are reused, ensuring efficient and resumable builds.
    *   **Comprehensive Logging:** Every action, decision, hash calculation, and `rustc` output is logged to a structured, machine-readable format for auditability.

5.  **Error Handling:**
    *   Gracefully handle malformed state files (`main_state.json`, `index_*.json`) by printing warnings and returning a default state.
    *   All `rustc` `stdout` and `stderr` are captured and stored as part of the build record, providing detailed debugging information on compilation failure.

6.  **Robust State Management:**
    *   State saving and caching mechanisms are fully enabled and critical. After each successful micro-step, the system's state (including the `compiled_artifacts_map` and build records) is updated and persisted, allowing for seamless resumption and ensuring that progress is never lost.

## Next Immediate Steps:

*   1. Enhance `cargo-llm-bootstrap` to implement comprehensive state capture, atomic build record generation, and robust logging for each micro-step, including the Monster Gödel Index derived from the file's sequential index.
*   2. Fully implement topological sorting within layers for `crates_to_process_initial`.
*   3. Refine `crate_name` extraction from `crate_root_path` to be more robust.
*   4. Address warnings about unused variables and imports.
*   5. Further refine `calculate_semantic_exponents` to incorporate more sophisticated analysis of `Cargo.toml` content and dependencies for more accurate exponent derivation (beyond just the sequential index).