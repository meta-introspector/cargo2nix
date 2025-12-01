# Braindump: Refactoring `trait-fixer` for Mockability

## Current Situation

We are attempting to refactor the `trait-fixer` tool to make it mockable, specifically to enable faster testing and development by replacing real `rustc_*` dependencies with mock implementations. This has proven significantly more complex than anticipated due to the intricacies of Rust's ecosystem, Cargo's dependency resolution, and the deep integration requirements of `rustc`'s internal crates.

## Journey (What didn't work and why)

1.  **Initial Conditional Compilation (`#[cfg(feature = "use_mock_rustc")]`)**:
    *   **Approach**: Used `#[cfg]` attributes and Cargo features (`use_mock_rustc`, `use_real_rustc`) within the `trait-fixer-*` vernacular crates (`crates/trait-fixer-compiler-host`, etc.) to switch between real `rustc_*` imports and mock imports (`trait_fixer_rustc_mock`).
    *   **Failure Point**:
        *   **`CFG_` environment variables**: Even with features enabled to use mocks, the build process for `rustc_*` components (like `rustc_hir`'s `version.rs`) still required highly specific and difficult-to-mimic `CFG_` environment variables (e.g., `CFG_RELEASE`). Our `scripts/set_mock_env.sh` was insufficient.
        *   **Transitive Real Dependencies**: It proved challenging to completely prevent the compilation of *real* `rustc_*` crates, even when they were marked `optional` and gated behind features. Cargo's resolution logic within a large workspace containing the `rustc` submodules often pulled them in.

2.  **Separate Mock Workspace with `[patch.crates-io]`**:
    *   **Approach**: Created a dedicated `mock_workspace/` with its own `Cargo.toml`. This workspace contained *copies* (not symlinks) of our `trait-fixer-*` crates and `trait-fixer-rustc-mock`. The `mock_workspace/Cargo.toml` used `[patch.crates-io]` to redirect all `rustc_*` dependencies to our `trait-fixer-rustc-mock` crate.
    *   **Intermediate Failures & Learnings**:
        *   **Symlink issues**: Initially used symlinks, which caused `read_file` tool errors and path resolution problems as they pointed outside the `mock_workspace` context. Switched to copies.
        *   **`workspace = true` in copied crates**: The copied `trait-fixer-*` crates still had `rustc_driver = { workspace = true, optional = true }`. This forced Cargo to look for `rustc_driver` in the `mock_workspace`'s (empty) `[workspace.dependencies]` section, leading to errors.
        *   **`[workspace.dependencies]` population**: Realized that any dependency using `workspace = true` *must* be declared in the root `[workspace.dependencies]`. We progressively added `trait-fixer-rustc-mock` and other `trait-fixer-*` crates, plus dummy `rustc_*` entries to this section.
        *   **`path = "..."` for peer `trait-fixer-*` dependencies**: When `trait-fixer-core` depended on `trait-fixer-rules` with `path = "trait-fixer-rules"`, it looked for `mock_workspace/trait-fixer-core/trait-fixer-rules/Cargo.toml`, which was incorrect. This indicated that for workspace members, `path = "..."` (relative to the individual crate) is not always interpreted as desired when peers are involved, and `workspace = true` is more appropriate.
    *   **Ultimate Failure Point**:
        *   **`[patch.crates-io]` expects identical names**: The core issue with `[patch.crates-io]` was that when we tried to patch `rustc_driver` (or `rustc_middle`, etc.) to point to `trait-fixer-rustc-mock`, Cargo expected the *package name* of the patched crate to be `rustc_driver` (or `rustc_middle`, etc.). Our `trait-fixer-rustc-mock` crate only has one package name: `trait-fixer-rustc-mock`. A single mock crate cannot pretend to be multiple distinct `crates.io` packages simultaneously via `[patch]`.

## New Architectural Plan: Trait-Only Crates + Separate Impl Crates

The previous approaches failed because they tried to either conditionally compile *within* the vernacular crates or use Cargo's patching mechanism in a way it wasn't designed for multi-crate aliases. The key insight is to follow the pattern "trait in one crate, implementations in others."

**Revised Structure for Each Vernacular Trait (`trait-fixer-xxx`):**

1.  **Trait Definition Crate (`crates/trait-fixer-xxx-trait`)**
    *   **Purpose**: To define only the Rust trait (e.g., `CompilerHost`) and its associated types/methods.
    *   **Dependencies**: Minimal, no `rustc_*` or other `trait-fixer-*` dependencies.
    *   **Example**: `crates/trait-fixer-compiler-host-trait/Cargo.toml`, `src/lib.rs` (defining `CompilerHost`).

2.  **Real Implementation Crate (`crates/trait-fixer-xxx-real`)**
    *   **Purpose**: To provide a concrete implementation of the trait using actual `rustc_*` components.
    *   **Dependencies**: Depends on `crates/trait-fixer-xxx-trait` and the necessary `rustc_*` crates.
    *   **Example**: `crates/trait-fixer-compiler-host-real/Cargo.toml`, `src/lib.rs` (implementing `CompilerHost` for `ActualCompilerHost`, using `rustc_driver`).

3.  **Mock Implementation Crate (`crates/trait-fixer-xxx-mock`)**
    *   **Purpose**: To provide a concrete implementation of the trait using mock types from `trait-fixer-rustc-mock`.
    *   **Dependencies**: Depends on `crates/trait-fixer-xxx-trait` and `crates/trait-fixer-rustc-mock`.
    *   **Example**: `crates/trait-fixer-compiler-host-mock/Cargo.toml`, `src/lib.rs` (implementing `CompilerHost` for `MockCompilerHost`, using types from `trait_fixer_rustc_mock`).

4.  **Central Mock Types Crate (`crates/trait-fixer-rustc-mock`)**
    *   **Purpose**: To define all the placeholder/mock types (e.g., `TyCtxt`, `Item`, `Span`, `DefId`, `Symbol`, etc.) that mimic the public API surface of the `rustc_*` crates.
    *   **Dependencies**: No `rustc_*` dependencies. Minimal external dependencies.
    *   **Content**: Only type definitions, `impl` blocks for basic functionality that traits might rely on (e.g., `TyCtxt::hir()`), and `PhantomData` for lifetime parameters.

5.  **Main `trait-fixer` Tool (`tools/trait-fixer`)**
    *   **Purpose**: To consume the vernacular traits and perform the analysis/fixing.
    *   **Dependencies**: Will depend on the specific `-real` or `-mock` implementation crates based on a chosen feature (e.g., `use_real_impls`, `use_mock_impls`). This will likely involve a pattern like:
        ```toml
        [dependencies]
        trait-fixer-compiler-host-trait = { workspace = true }
        trait-fixer-query-context-trait = { workspace = true }
        # ... other trait crates

        [features]
        default = ["use_real_impls"]
        use_real_impls = [
            "trait-fixer-compiler-host-real",
            "trait-fixer-query-context-real",
            # ...
        ]
        use_mock_impls = [
            "trait-fixer-compiler-host-mock",
            "trait-fixer-query-context-mock",
            # ...
        ]
        ```
        And then `trait-fixer`'s `src/main.rs` would use a `Box<dyn Trait>` pattern or similar to abstract over the specific implementation.

## Next Steps

This requires a complete overhaul of the crate structure for `trait-fixer`.

1.  **For each `trait-fixer-xxx` crate:**
    *   Create a new `crates/trait-fixer-xxx-trait` crate and move the trait definition there.
    *   Rename the original `crates/trait-fixer-xxx` to `crates/trait-fixer-xxx-real`. Update its `Cargo.toml` and `src/lib.rs` to implement the new trait using real `rustc` dependencies.
    *   Create a new `crates/trait-fixer-xxx-mock` crate. Update its `Cargo.toml` and `src/lib.rs` to implement the new trait using `trait-fixer-rustc-mock` types.
2.  **Update `crates/trait-fixer-rustc-mock/src/lib.rs`**: Ensure it only contains the mock type definitions, and does not try to implement any traits directly.
3.  **Update `tools/trait-fixer/Cargo.toml` and `src/main.rs`**: Adjust dependencies and usage to select between real and mock implementations.
4.  **Update the main workspace `Cargo.toml`**: Add all the new `trait-fixer-xxx-trait`, `-real`, and `-mock` crates as members.

This is a large task and will require careful execution. I will ensure clear communication of progress.
