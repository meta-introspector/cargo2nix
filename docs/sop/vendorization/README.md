# Standard Operating Procedure: Vendorization of Rust Crates

This document outlines the process for vendorizing Rust crates within the `cargo2nix` project. The core principle is that all external dependencies should be replaced with local path dependencies pointing to submodules within the `vendor/rust/cargo2nix/submodules` directory. This ensures a controlled and reproducible build environment.

## General Principles

1.  **Local Paths Only:** All `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` entries in `Cargo.toml` files across the workspace should point to local paths within the `submodules` directory, or use `workspace = true` if the dependency is defined in the root `[workspace.dependencies]` section as a path dependency.
2.  **Consistency:** Ensure that dependency names (e.g., `serde_core` vs `serde-core`) are consistent across feature definitions, `[dependencies]` sections, and `[workspace.dependencies]` entries.
3.  **Optional Dependencies:** If a feature conditionally enables a dependency using `dep:crate_name` or `crate_name?/feature`, the corresponding dependency in the `[dependencies]` section *must* be marked with `optional = true`.
4.  **Workspace Definition:** The root `Cargo.toml`'s `[workspace.dependencies]` section should define common dependencies as path dependencies to their respective submodules.

## Troubleshooting Common Issues

### 1. Dependency with Different Source Paths (e.g., `rand08`)

**Problem:** `Dependency 'rand08' has different source paths depending on the build target. Each dependency must have a single canonical source path irrespective of build target.`

**Cause:** This typically occurs when a dependency (e.g., `rand`) is defined in multiple ways:
    *   As a `crates.io` dependency in `[workspace.dependencies]`.
    *   As a local `path` dependency in a crate's `[dependencies]` section.
    *   When multiple versions of the same crate (e.g., `rand08` and `rand09`) point to the same submodule path.

**Resolution Steps:**

*   **Remove `[patch.crates-io]` entries:** If a `[patch.crates-io]` entry is attempting to override a dependency that is also defined as a path dependency, remove the `[patch.crates-io]` entry.
*   **Consolidate `rand` versions:** If multiple versions of a crate (e.g., `rand08`, `rand09`) are pointing to the *same* submodule, remove the redundant entries from the crate's `Cargo.toml` and its features.
*   **Remove conflicting `[workspace.dependencies]` entries:** If a crate explicitly defines a dependency as a `path` to a submodule, ensure that the same dependency is *not* defined as a `crates.io` dependency in the root `[workspace.dependencies]` section.

### 2. Dependency Not Found in Workspace (e.g., `proc-macro2`)

**Problem:** `error inheriting 'proc-macro2' from workspace root manifest's 'workspace.dependencies.proc-macro2' Caused by: 'dependency.proc-macro2' was not found in 'workspace.dependencies'`

**Cause:** A crate within the workspace is trying to inherit a dependency from `[workspace.dependencies]`, but that dependency is missing from the root `Cargo.toml`.

**Resolution Steps:**

*   **Add to `[workspace.dependencies]`:** Locate the submodule for the missing dependency (e.g., `proc-macro2`) within `submodules/`. Add an entry to the root `Cargo.toml`'s `[workspace.dependencies]` section, pointing to this local path:
    ```toml
    proc-macro2 = { path = "./submodules/proc-macro2" }
    ```

### 3. Feature Includes Dependency Not Listed (e.g., `serde-core`)

**Problem:** `feature 'alloc' includes 'serde-core?/alloc', but 'serde-core' is not a dependency`

**Cause:** A feature definition refers to a dependency (e.g., `serde-core`) that is either:
    *   Not listed in the `[dependencies]` section at all.
    *   Listed with a different name (e.g., `serde_core` vs `serde-core`).

**Resolution Steps:**

*   **Correct Dependency Name:** Ensure the dependency name in the feature definition exactly matches the name in the `[dependencies]` section. If the dependency is `serde_core` in `[dependencies]`, it should be `serde_core?/alloc` in the feature.
*   **Add Missing Dependency:** If the dependency is entirely missing, add it to the `[dependencies]` section.

### 4. Feature Includes Non-Optional Dependency (e.g., `serde-core?/alloc`)

**Problem:** `feature 'alloc' includes 'serde-core?/alloc' with a '?', but 'serde-core' is not an optional dependency`

**Cause:** A feature uses the `?` syntax to conditionally enable a dependency's feature, but the dependency itself is not marked as `optional = true` in the `[dependencies]` section.

**Resolution Steps:**

*   **Mark as Optional:** Add `optional = true` to the dependency definition in the `[dependencies]` section:
    ```toml
    serde_core = { path = "./submodules/serde/serde_core" , default-features = false, optional = true }
    ```

## Example Workflow (from recent troubleshooting)

1.  **Initial Error:** `dependency (rand) specified without providing a local path, Git repository, version, or workspace dependency to use`
    *   **Action:** Removed `[patch.crates-io]` entry for `rand` from root `Cargo.toml`.
2.  **Next Error:** `Dependency 'rand08' has different source paths depending on the build target.`
    *   **Action:** Removed `rand` feature from `time` dependency in root `Cargo.toml`.
    *   **Action:** Removed `rand09` feature and dependency from `time/Cargo.toml`.
    *   **Action:** Removed `rand08` from `[workspace.dependencies]` in root `Cargo.toml` (as `time` was using a path dependency).
3.  **Next Error:** `error inheriting 'rand08' from workspace root manifest's 'workspace.dependencies.rand08' Caused by: 'dependency.rand08' was not found in 'workspace.dependencies'`
    *   **Action:** Removed `rand08` and `rand09` from `dev-dependencies` in `time/Cargo.toml`.
4.  **Next Error:** `feature 'alloc' includes 'serde-core?/alloc', but 'serde-core' is not a dependency`
    *   **Action:** Corrected `serde-core?/alloc` to `serde_core?/alloc` in `time/Cargo.toml`'s `alloc` feature.
5.  **Next Error:** `feature 'alloc' includes 'serde_core?/alloc' with a '?', but 'serde_core' is not an optional dependency`
    *   **Action:** Marked `serde_core` as `optional = true` in `time/Cargo.toml`'s `[dependencies]`.
6.  **Next Error:** `feature 'formatting' includes 'time-macros?/formatting' with a '?', but 'time-macros' is not an optional dependency`
    *   **Action:** Marked `time-macros` as `optional = true` in `time/Cargo.toml`'s `[dependencies]`.
7.  **Next Error:** `feature 'rand08' includes 'dep:rand08', but 'rand08' is not an optional dependency`
    *   **Action:** Marked `rand08` as `optional = true` in `time/Cargo.toml`'s `[dependencies]`.
8.  **Next Error:** `feature 'serde' includes 'dep:serde-core', but 'serde-core' is not listed as a dependency`
    *   **Action:** Corrected `dep:serde-core` to `dep:serde_core` in `time/Cargo.toml`'s `serde` feature.
9.  **Next Error:** `failed to read '/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/serde-core/Cargo.toml' Caused by: No such file or directory`
    *   **Action:** Corrected `serde_core` path in `time/Cargo.toml` to `serde_core = { path = "./submodules/serde/serde_core" , default-features = false, optional = true }`.
10. **Next Error:** `feature 'alloc' includes 'serde-core?/alloc', but 'serde-core' is not a dependency`
    *   **Action:** Corrected `serde-core?/alloc` to `serde_core?/alloc` in `time/Cargo.toml`'s `alloc` feature.
11. **Next Error:** `feature 'serde' includes 'dep:serde-core', but 'serde-core' is not listed as a dependency`
    *   **Action:** Corrected `dep:serde-core` to `dep:serde_core` in `time/Cargo.toml`'s `serde` feature.
12. **Next Error:** `error inheriting 'proc-macro2' from workspace root manifest's 'workspace.dependencies.proc-macro2' Caused by: 'dependency.proc-macro2' was not found in 'workspace.dependencies'`
    *   **Action:** Added `proc-macro2 = { path = "./submodules/proc-macro2" }` to root `Cargo.toml`'s `[workspace.dependencies]`.
