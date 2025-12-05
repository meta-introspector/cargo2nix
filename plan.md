# Current Build Debugging Plan

## I. Current Status and Resolved Issues

The project build was failing with several errors. The following issues have been investigated and resolved:

*   **`rustc_llvm` Compilation Errors:**
    *   **Problem:** Incompatibility between `rustc_llvm`'s C++ wrappers and LLVM 19.1.7, manifesting as errors like `no member named 'SanitizeRealtime'` and `fatal error: 'llvm/Transforms/Instrumentation/RealtimeSanitizer.h' file not found`.
    *   **Resolution:** Cherry-picked commit `27b7b3f0314` into `submodules/rust`. This commit significantly modified `submodules/rust/compiler/rustc_llvm/build.rs` to hardcode LLVM component definitions, effectively bypassing the dynamic `llvm-config` calls that caused compatibility issues.
*   **`rustc_fluent_macro` Type Inference Error (`E0282`):**
    *   **Problem:** A type inference error in `submodules/rust/compiler/rustc_fluent_macro/src/fluent.rs` where `Ident::new` received a temporary `&String` from a `format!` macro, leading to a lifetime issue.
    *   **Resolution:** Modified the problematic line to explicitly create a `String` variable (`formatted_name`) and then pass a stable reference (`&formatted_name`) to `Ident::new`, resolving the lifetime and type inference issue.
*   **`rustc_log` Unresolved Import (`tracing_core`):**
    *   **Problem:** `submodules/rust/compiler/rustc_log/src/lib.rs` attempted to `pub use tracing_core`, but `tracing_core` was not a direct dependency declared in `rustc_log/Cargo.toml`.
    *   **Resolution:** Removed `tracing_core` from the `pub use` statement in `submodules/rust/compiler/rustc_log/src/lib.rs`.
*   **`rustc_index` Unstable Feature Errors (`E0658` & `E0635`):**
    *   **Problem:** Initial errors about `const_pin` and `new_zeroed` being unstable features, followed by `unknown feature new_zeroed` and `new_zeroed_alloc` being stable warnings after updating the Rust toolchain.
    *   **Resolution:** Enabled `#![feature(const_pin)]` and `#![feature(new_zeroed_alloc)]` in `submodules/rust/compiler/rustc_index/src/lib.rs`. Subsequently, removed `#![feature(new_zeroed)]` as it was an unknown/deprecated feature for the updated nightly Rust compiler.
*   **`librocksdb-sys` `Unable to find libclang` Error:**
    *   **Problem:** The `librocksdb-sys` build script, which uses `bindgen`, could not find `libclang.so` because `LIBCLANG_PATH` in `flake.nix` was incorrectly pointing to `pkgs.clang/lib` instead of the actual `libclang` package.
    *   **Resolution:** Corrected `LIBCLANG_PATH` in `devShells.default` and `devShells.build` sections of `flake.nix` to `"${pkgs.llvmPackages_19.libclang}"`.
*   **Nix `rust-bin.nightly` Attribute Missing Error:**
    *   **Problem:** Attempting to use a non-existent `rust-bin.nightly` date (`2025-12-05`) in `flake.nix`.
    *   **Resolution:** Updated `myRustc` in `flake.nix` to use the available nightly build `2025-10-05`.
*   **`cargo` `unclosed delimiter` errors in `compilation_orchestration.rs`:**
    *   **Problem:** Compilation errors in `submodules/cargo/src/cargo/core/compiler/compilation_orchestration.rs` related to unclosed delimiters within a `with_context` closure. This prevented `cargo` from building.
    *   **Resolution:** Fixed the `format!` macro call, closed the `match` statement, and explicitly returned the `result` variable from the `Work::new` closure, and added the final closing brace for the `rustc_work` function. These changes ensure the correct syntactic structure and allow `cargo` to compile.

## II. Remaining Issues (Warnings)

The following issues are currently present as warnings and do not block the build, but should be addressed in future cleanup:

*   **`im-rs/rc` `unexpected cfg condition name` Warnings:** Warnings about `has_specialisation` and `threadsafe` `cfg` conditions not being properly declared to `rustc`.
*   **`rustc_llvm` `llvm_component` `unexpected cfg condition name` Warnings:** Warnings indicating that `llvm_component` `cfg` flags, while set by `build.rs`, are not formally declared to `rustc`.
*   **General `unused imports`, `unused variables`, and `dead_code` Warnings:** Various warnings across several crates indicating potential code quality issues.

## III. Next Steps

1.  **Implement `cargo build` flags in Nix for reproducibility and capture:** Configure Nix derivations to use `cargo build --quiet --reproducible=bash` and `--capture=all` flags. This will involve identifying the relevant Nix expressions that invoke `cargo build` and modifying them to include these flags.
2.  **Full Build Verification:** Execute `make build` to ensure all current fixes have taken effect and that the project now compiles without any blocking errors.
3.  **Systematic Warning Resolution:** After a clean build, address the remaining warnings by either:
    *   Adding appropriate `check-cfg` entries to `Cargo.toml` files or `build.rs` scripts for unexpected `cfg` conditions.
    *   Removing unused `use` statements or variables.
    *   Refactoring code flagged as `dead_code` if it's indeed unused, or marking it appropriately if it's intentionally retained.
4.  **Review `ast_parser_impl` dependencies:** Re-verify that `prelude-generator` and `split-expanded-lib` are correctly handled. (This was a lingering task that needs a final check).
5.  **Final `tracing-tree` conflict check:** Ensure no latent `tracing-tree` version conflicts remain.

This plan aims to achieve a fully compiling project with Nix integration, then to systematically clean up all warnings to ensure code quality and maintainability.