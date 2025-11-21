### Problem: Cyclic Package Dependency during `add_submodules` build

The `cargo build -p cargo-repo-sync --bin add_submodules` command fails with a "cyclic package dependency" error, specifically:
`package parking_lot v0.12.5 (...) depends on itself. Cycle: parking_lot -> tracing-subscriber -> loom -> crossbeam-utils -> crossbeam-epoch -> crossbeam-deque -> rayon-core -> rayon -> hashbrown -> indexmap -> gimli -> addr2line -> backtrace -> parking_lot_core -> parking_lot`

This cycle arises because `Cargo`'s resolver detects a circular dependency among path dependencies. Even though submodules are added and `[patch.crates-io]` entries exist in the root `Cargo.toml`, the `Cargo.toml` files *within* the submodules (and their sub-crates) are still referencing each other in a way that creates a loop.

The `patch_cargo_toml` function, in its current form, is not effectively breaking this cycle because:
1.  It primarily focuses on top-level submodules and might not correctly identify or modify dependencies that are sub-crates of other vendored submodules (e.g., `parking_lot_core` is a sub-crate of `parking_lot`).
2.  The `vendored_crates` list passed to `patch_cargo_toml` currently only includes the names of top-level submodules, not their internal sub-crates.

### Proposed Solution: Refine `patch_cargo_toml` and Dependency Discovery

To resolve this, the `patch_cargo_toml` function and the dependency discovery mechanism need to be more robust:

1.  **Comprehensive `vendored_crates` List:**
    *   The `vendored_crates` list must be expanded to include *all* crate names that are part of the vendored set, including sub-crates within multi-crate submodules (e.g., `parking_lot_core` should be in this list, not just `parking_lot`).
    *   This requires a more sophisticated discovery process that not only finds top-level `Cargo.toml` files but also parses them to identify all internal crates (members of a workspace within a submodule, or individual crates within a submodule directory structure).

2.  **Aggressive `Cargo.toml` Patching:**
    *   The `patch_cargo_toml` function needs to ensure that *any* dependency found in a submodule's `Cargo.toml` that corresponds to a crate in the comprehensive `vendored_crates` list is converted to use `workspace = true`.
    *   This involves removing any explicit `version`, `git`, `branch`, or `path` keys from such dependencies and inserting `workspace = true`. This will force Cargo to resolve these dependencies through the root workspace's `[workspace.dependencies]` or `[patch.crates-io]` sections, effectively breaking internal cycles.

3.  **Re-evaluate `[workspace.dependencies]` and `[patch.crates-io]` Interaction:**
    *   Ensure that the entries in the root `Cargo.toml`'s `[workspace.dependencies]` and `[patch.crates-io]` sections are consistent and do not inadvertently create new conflicts or cycles. The primary goal is for all vendored dependencies to be resolvable through the workspace mechanism.

By implementing these changes, the system should be able to correctly resolve the dependencies and break the cyclic dependency error. The next step is to enhance the `RepoDiscoverer` to find all sub-crates and then update `patch_cargo_toml` to use this more comprehensive list.