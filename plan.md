# Current Build Debugging Plan

## I. Current Status and Resolved Issues

The project build was failing with several errors. The following issues have been investigated and resolved:

*   **Compilation Errors Resolution (Current Task):**
    *   **Problem:** C/C++ header not found errors (e.g., `stdbool.h`, `stdlib.h`), Rust macro incompatibility errors, and unresolved type errors after module refactoring in `rustc_target` and `librocksdb-sys`.
    *   **Resolution:**
        *   Fixed `mod os;` and `mod env;` declarations in `submodules/rust/compiler/rustc_target/src/spec/targets.rs` by removing them, as these modules are managed by `mod.rs`.
        *   Resolved duplicate `LinkerFlavor` and `LinkerFlavorCli` imports in `submodules/rust/compiler/rustc_target/src/spec/target_options.rs`.
        *   Corrected `rustc_abi::Align` import in `submodules/rust/compiler/rustc_target/src/spec/mod.rs` to use global path `::rustc_abi::Align`.
        *   Enabled `DebuginfoKind` re-export by adding `pub mod debuginfo_kind;` and `pub use debuginfo_kind::*;` to `submodules/rust/compiler/rustc_target/src/spec/mod.rs`.
        *   Implemented `ToJson` trait for `Align` in `submodules/rust/compiler/rustc_target/src/spec/json.rs`, switching from `Json::U64` to `Json::Number` and adding `use serde_json::Number;`.
        *   Added `use std::str::FromStr;` to `submodules/rust/compiler/rustc_target/src/spec/debuginfo_kind.rs` for macro context.
        *   Added `use crate::spec::crt_objects::CrtObjects;` and `use crate::spec::SymbolVisibility;` to `submodules/rust/compiler/rustc_target/src/spec/target_options.rs`.
        *   Resolved `librocksdb-sys` header discovery issues by:
            *   Setting Nix store paths as `const` values instead of relying on environment variables within `build.rs`.
            *   Explicitly setting `LIBCLANG_FLAGS` environment variable for `bindgen_rocksdb` to correctly specify `sysroot` and include paths.
            *   Configuring `build_rocksdb` to use explicit `CPATH` environment variable and `sysroot` flag for `cc-rs` to ensure `g++` finds `stdlib.h`.
        *   Fixed `unused_fields` privacy error in `submodules/rust/compiler/rustc_target/src/spec/json.rs` by using `TargetWarnings::empty()` constructor.

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
*   **Nix Flake Escaping and Naming:**
    *   **Problem:** Generated Nix reproduction flakes contained improperly escaped `rustc` commands in their `shellHook`, leading to syntax errors. They also lacked a structured naming convention and output directory.
    *   **Resolution:** Refactored flake generation logic into a new `flake-repro-lib` crate. The flakes are now generated into a `repro/` directory with SHA256 hashed filenames. This includes correctly escaping shell commands and dynamically determining Nix system architecture.

## II. Ongoing Refactoring and Next Steps

The project is undergoing significant architectural refactoring to enhance reproducibility, modularity, and control over the build process.

*   **Rustc Argument Capture and TOML Serialization:**
    *   **Goal:** To capture raw `rustc` invocation arguments directly from `cargo` before any shell escaping, and serialize them into a TOML format. This eliminates the need for shell-based escaping in Nix flakes and paves the way for a custom Rust "Nix runner" to execute `rustc`.
    *   **Progress:**
        *   Introduced `RustcInvocation` struct in `submodules/cargo/src/cargo/util/rustc.rs` for structured `rustc` invocation data.
        *   Modified `submodules/cargo/src/cargo/core/compiler/invocation_args.rs::prepare_rustc_process` to capture `ProcessBuilder` data into a `RustcInvocation` instance and return both the `ProcessBuilder` and `RustcInvocation`.
        *   Added `serde` and `toml` dependencies to `submodules/cargo/Cargo.toml` to support serialization.
        *   Created `crates/rustc-arg-builder-lib` with a `RustcArgGenerator` trait and `DefaultRustcArgGenerator` implementation to encapsulate `rustc` command generation logic.
        *   Modified `crates/flake-repro-lib` to use `rustc-arg-builder-lib` and handle the new `RustcInvocation` object.
        *   Split `mod.rs` into `linker_flavor.rs` for `LinkerFlavor` and related components.
        *   Split `mod.rs` into `link_self_contained.rs` for `LinkSelfContainedDefault` and `LinkSelfContainedComponents`.
        *   Split `mod.rs` into `linker_features.rs` for `LinkerFeatures`.
        *   Split `mod.rs` into `panic_strategy.rs` for `PanicStrategy`.
        *   Split `mod.rs` into `on_broken_pipe.rs` for `OnBrokenPipe`.
        *   Split `mod.rs` into `relro_level.rs` for `RelroLevel`.
        *   Split `mod.rs` into `symbol_visibility.rs` for `SymbolVisibility`.
        *   Split `mod.rs` into `small_data_threshold_support.rs` for `SmallDataThresholdSupport`.
        *   Split `mod.rs` into `merge_functions.rs` for `MergeFunctions`.
        *   Split `mod.rs` into `reloc_model.rs` for `RelocModel`.
        *   Split `mod.rs` into `code_model.rs` for `CodeModel`.
        *   Split `mod.rs` into `float_abi.rs` for `FloatAbi`.
        *   Split `mod.rs` into `rustc_abi.rs` for `RustcAbi`.
        *   Split `mod.rs` into `tls_model.rs` for `TlsModel`.
        *   Split `mod.rs` into `link_output_kind.rs` for `LinkOutputKind`.
        *   Split `mod.rs` into `debuginfo_kind.rs` for `DebuginfoKind`.
        *   Split `mod.rs` into `split_debuginfo.rs` for `SplitDebuginfo`.
        *   Split `mod.rs` into `stack_probe_type.rs` for `StackProbeType`.
        *   Split `mod.rs` into `sanitizer_set.rs` for `SanitizerSet`.
        *   Split `mod.rs` into `frame_pointer.rs` for `FramePointer`.
        *   Split `mod.rs` into `stack_protector.rs` for `StackProtector`.
        *   Split `mod.rs` into `binary_format.rs` for `BinaryFormat`.
        *   Split `mod.rs` into `target_warnings.rs` for `TargetWarnings`.
        *   Split `mod.rs` into `arch.rs` for `Arch`.
        *   Split `mod.rs` into `os.rs` for `Os`.
    *   **Next Action:** Continue splitting `mod.rs` into additional files as outlined in the internal TODO list.

*   **Refactor `rustc_target/src/spec/mod.rs`:**
    *   **Goal:** Split the large `mod.rs` file into smaller, logically grouped files to improve maintainability and readability.
    *   **Progress:**
        *   Created `linker_flavor.rs` and moved `Cc`, `Lld`, `LinkerFlavor`, `LinkerFlavorCli`, `LldFlavor`, their `impl` blocks, and associated macros/implementations into it.
        *   Updated `mod.rs` to import and re-export the contents of `linker_flavor.rs`.
        *   Simplified import paths in `json.rs` and `target_options.rs` (e.g., `crate::spec::module::Type` to `crate::spec::Type`).
        *   Verified `arch.rs` for correct `desc_symbol` implementation (use `rustc_span::Symbol::intern("unknown")`).
    *   **Next Action:** Continue splitting `mod.rs` into additional files as outlined in the internal TODO list.

## III. Remaining Issues (Warnings)

The following issues are currently present as warnings and do not block the build, but should be addressed in future cleanup:

*   **`im-rs/rc` `unexpected cfg condition name` Warnings:** Warnings about `has_specialisation` and `threadsafe` `cfg` conditions not being properly declared to `rustc`.
*   **`rustc_llvm` `llvm_component` `unexpected cfg condition name` Warnings:** Warnings indicating that `llvm_component` `cfg` flags, while set by `build.rs`, are not formally declared to `rustc`.
*   **General `unused imports`, `unused variables`, and `dead_code` Warnings:** Various warnings across several crates indicating potential code quality issues.

## IV. Future Work

1.  **Systematic Warning Resolution:** After a clean build, address the remaining warnings by either:
    *   Adding appropriate `check-cfg` entries to `Cargo.toml` files or `build.rs` scripts for unexpected `cfg` conditions.
    *   Removing unused `use` statements or variables.
    *   Refactoring code flagged as `dead_code` if it's indeed unused, or marking it appropriately if it's intentionally retained.
2.  **Implement `cargo build` flags in Nix for reproducibility and capture:** Configure Nix derivations to use `cargo build --quiet --reproducible=bash` and `--capture=all` flags. This will involve identifying the relevant Nix expressions that invoke `cargo build` and modifying them to include these flags.
3.  **Full Build Verification:** Execute `make build` to ensure all current fixes have taken effect and that the project now compiles without any blocking errors.
4.  **Review `ast_parser_impl` dependencies:** Re-verify that `prelude-generator` and `split-expanded-lib` are correctly handled. (This was a lingering task that needs a final check).
5.  **Final `tracing-tree` conflict check:** Ensure no latent `tracing-tree` version conflicts remain.

This plan aims to achieve a fully compiling project with Nix integration, then to systematically clean up all warnings to ensure code quality and maintainability.