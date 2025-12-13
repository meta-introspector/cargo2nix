# Current Build Debugging Plan - REBOOTED

## I. Current Status and Resolved Issues (Recap)

The project build has undergone significant debugging and refactoring. The following issues have been investigated and largely resolved, as previously documented:

*   **Compilation Errors Resolution:** Various C/C++ header issues, Rust macro incompatibilities, and type errors related to `rustc_target` and `librocksdb-sys` have been addressed. This included extensive modifications to `mod.rs` files across several modules, correcting imports, and enabling necessary Rust features.
*   **`rustc_llvm` Compilation Errors:** Incompatibilities with LLVM 19.1.7 were resolved by cherry-picking a specific commit into `submodules/rust`.
*   **`rustc_fluent_macro` Type Inference Error (`E0282`):** A lifetime issue with `Ident::new` was fixed by explicitly managing `String` ownership.
*   **`rustc_log` Unresolved Import (`tracing_core`):** An incorrect `pub use` statement was removed.
*   **`rustc_index` Unstable Feature Errors (`E0658` & `E0635`):** Unstable features were enabled and deprecated ones removed.
*   **`librocksdb-sys` `Unable to find libclang` Error:** `LIBCLANG_PATH` in `flake.nix` was corrected.
*   **Nix `rust-bin.nightly` Attribute Missing Error:** The nightly build date in `flake.nix` was updated.
*   **`cargo` `unclosed delimiter` errors:** Syntax errors in `compilation_orchestration.rs` were corrected.
*   **Nix Flake Escaping and Naming:** Flake generation logic was refactored into a `flake-repro-lib` crate for improved structure and escaping.

## II. Ongoing Refactoring and Next Steps

The project is undergoing significant architectural refactoring to enhance reproducibility, modularity, and control over the build process.

*   **Rustc Argument Capture and TOML Serialization (Completed Splitting of `rustc_target/src/spec/mod.rs`)**
    *   The previous goal was to split the large `rustc_target/src/spec/mod.rs` file into smaller, logically grouped files.
    *   **Status:** This splitting has been successfully completed. The `mod.rs` file now primarily serves as a re-export hub for the individual modules, ensuring better maintainability and readability. All previously identified components now reside in their dedicated files within `submodules/rust/compiler/rustc_target/src/spec/`.

*   **Rustc Expand Refactoring (`InvocationCollectorNode` Type Unification)**
    *   **Goal:** Resolve type mismatch errors related to the `InvocationCollectorNode` trait in `submodules/rust/compiler/rustc_expand`. The trait's `OutputTy` associated type was being used in conflicting ways (for single optional nodes and for multiple nodes).
    *   **Progress:**
        *   The `InvocationCollectorNode` trait definition in `src/ast_fragments_split/ast_fragments_helpers.rs` has been refactored to introduce two distinct associated types: `VisitOutputTy` (for single optional node results) and `FlatMapOutputTy` (for zero, one, or many node results).
        *   The `fragment_to_output` method has been split into `fragment_to_visit_output` and `fragment_to_flat_map_output` within the `InvocationCollectorNode` trait.
        *   The `InvocationCollector::visit_node` function in `src/ast_fragments_split/ast_fragments_helpers.rs` has been updated to use `Node::VisitOutputTy` and `Node::fragment_to_visit_output`.
        *   The `signature of InvocationCollector::flat_map_node` in `src/ast_fragments_split/ast_fragments_helpers.rs` has been updated to use `Node::FlatMapOutputTy`.
        *   The `walk` method for `ast::Stmt` in `src/ast_fragments_split/ast_fragments_node_impls.rs` has been corrected to handle the `StmtKind::Let` variant (API change) and to manually visit `MacCallStmt` components (due to `visit_mac_call_stmt` not being found/available).
        *   The `post_flat_map_node_collect_bang` function in `src/ast_fragments_split/ast_fragments_node_impls.rs` has been fixed to correctly handle `Option<SmallVec<Stmt, 1>>` when calling `pop()` and `push()`.
        *   All instances of `Node::fragment_to_output` within the `flat_map_node` function in `src/ast_fragments_split/ast_fragments_helpers.rs` have been replaced with `Node::fragment_to_flat_map_output`.
    *   **Next Action:**
        1.  **Update all `impl InvocationCollectorNode for ...` blocks:** Go through every implementation of `InvocationCollectorNode` in `src/ast_fragments_split/ast_fragments_node_impls.rs` and update:
            *   `type OutputTy = ...` to `type VisitOutputTy = ...` and `type FlatMapOutputTy = ...`.
            *   `fn fragment_to_output` to `fn fragment_to_visit_output` and `fn fragment_to_flat_map_output`.
            *   Update all other trait methods to use the new `VisitOutputTy` or `FlatMapOutputTy` as appropriate.
        2.  **Address `Option<T>` dereferencing/field access errors:** Fix explicit unwrapping or handling of `Option<T>` where errors like `type Option<T> cannot be dereferenced` or `no field 'kind' on type Option<T>` occur.
        3.  **Implement `MutVisitor` for `AstFragment`:** Provide a concrete implementation for the `MutVisitor` trait for the `AstFragment` enum.
        4.  **Implement `std::fmt::Display` for `AstFragment`:** Provide a `Display` trait implementation for `AstFragment`.

## III. Remaining Issues (Warnings)

The following issues are currently present as warnings and do not block the build, but should be addressed in future cleanup:

*   **`im-rs/rc` `unexpected cfg condition name` Warnings:** Warnings about `has_specialisation` and `threadsafe` `cfg` conditions not being properly declared to `rustc`.
*   **`rustc_llvm` `llvm_component` `unexpected cfg condition name` Warnings:** Warnings indicating that `llvm_component` `cfg` flags, while set by `build.rs`, are not formally declared to `rustc`.
*   **General `unused imports`, `unused variables`, and `dead_code` Warnings:** Various warnings across several crates indicating potential code quality issues.

## IV. Future Work

1.  **Systematic Warning Resolution:** After a clean build, address the remaining warnings by either:
    *   Adding appropriate `check-cfg` entries to `Cargo.toml` files or `build.rs` scripts for unexpected `cfg` conditions.
    *   Removing unused `use` statements or variables.
    *   Refactoring code flagged as `dead_code` if it's indeed unused, or marking it appropriately if it's retained.
2.  **Implement `cargo build` flags in Nix for reproducibility and capture:** Configure Nix derivations to use `cargo build --quiet --reproducible=bash` and `--capture=all` flags. This will involve identifying the relevant Nix expressions that invoke `cargo build` and modifying them to include these flags.
3.  **Full Build Verification:** Execute `make build` to ensure all current fixes have taken effect and that the project now compiles without any blocking errors.
4.  **Review `ast_parser_impl` dependencies:** Re-verify that `prelude-generator` and `split-expanded-lib` are correctly handled. (This was a lingering task that needs a final check).
5.  **Final `tracing-tree` conflict check:** Ensure no latent `tracing-tree` version conflicts remain.

This plan aims to achieve a fully compiling project with Nix integration, then to systematically clean up all warnings to ensure code quality and maintainability.
