# Derive Macro Refactoring Pattern

## Problem Statement

Developing Rust procedural macros, particularly derive macros, within a large and complex workspace can lead to several challenges, primarily related to:

1.  **Macro Resolution:** The compiler often struggles to find and correctly apply derive macros, resulting in errors like `cannot find derive macro Diagnostic in this scope` or `unresolved import`. This can be exacerbated by intricate module structures and inter-crate dependencies.
2.  **Cyclic Dependencies:** A common pitfall occurs when a procedural macro crate (e.g., `my_macro`) needs to know about the types it's deriving for (e.g., structs in `my_lib`), and `my_lib` needs to depend on `my_macro` to use the derive attributes. This creates a circular dependency, preventing compilation.
3.  **Dependency Version Mismatches:** Tools used within procedural macros (e.g., `syn`, `quote`, `proc-macro2`) can have specific version requirements. If the procedural macro crate or its dependencies use conflicting versions of these tools, it leads to "mismatched types" errors, where types from different versions of the same crate are considered incompatible.

## Solution: The Three-Crate Refactoring Pattern

To address these issues and establish a robust, maintainable structure for derive macros, we adopted a three-crate refactoring pattern:

1.  **The Library Crate (e.g., `my_lib_errors`):**
    *   **Purpose:** Contains the core data structures (e.g., `Diagnostic` and `Subdiagnostic` structs) that the derive macros will operate on. It also defines the traits (`Diagnostic`, `Subdiagnostic`) that the derive macros will implement for these structures.
    *   **Dependencies:** Depends on core Rust crates (e.g., `rustc_span`) and, importantly, depends on the *procedural macro crate* (e.g., `my_lib_macros`) to apply the `#[derive(...)]` attributes to its structs.

2.  **The Procedural Macro Crate (e.g., `my_lib_macros`):**
    *   **Purpose:** Implements the actual derive logic for the traits defined in the library crate. It contains functions annotated with `#[proc_macro_derive]`.
    *   **Dependencies:** Strictly depends on `syn`, `quote`, `proc-macro2`, and `synstructure` (or similar parsing/code-generation crates). **Crucially, it does NOT directly depend on the library crate (`my_lib_errors`) at the Rust code level for type information.** It receives type information via `syn::DeriveInput` during macro expansion. This avoids cyclic dependencies.
    *   **Implementation Detail:** Ensures its `syn` dependency version matches that of `synstructure` (or other macro helper crates) to prevent "mismatched types" errors.

3.  **The Test Driver Crate (e.g., `my_lib_test_driver`):**
    *   **Purpose:** A binary crate used solely for integration testing of the library and macro crates. It verifies that the derive macros can be successfully applied to the library's structs and that the generated code compiles and runs as expected.
    *   **Dependencies:** Depends on both the library crate (`my_lib_errors`) and the procedural macro crate (`my_lib_macros`).
    *   **Usage:** Explicitly uses fully qualified paths for the derive macros (e.g., `#[derive(my_lib_macros::Diagnostic)]`) and imports the traits from the library crate (e.g., `use my_lib_errors::Diagnostic;`) to ensure correct resolution and testing.

## Steps Taken in Refactoring `rustc_expand_base_lib`

1.  **Created New Crates:**
    *   `rustc_expand_base_lib_errors` (library): for `TraceMacroBase` and `TraceMacroNote` structs, and their `Diagnostic`/`Subdiagnostic` traits.
    *   `rustc_expand_base_lib_macros` (proc-macro): for `#[derive(Diagnostic)]` and `#[derive(Subdiagnostic)]` implementations.
    *   `rustc_expand_base_lib_test_driver` (binary): for testing the above two.

2.  **Updated Root `Cargo.toml`:** Added all three new crates to the `[workspace.members]` section to ensure Cargo recognizes them.

3.  **Configured `Cargo.toml` for each new crate:**
    *   **`rustc_expand_base_lib_errors`**: Declared its dependencies on `rustc_span` and `rustc_errors` (for core Rust diagnostic types), and `rustc_expand_base_lib_macros` (for the derive attributes).
    *   **`rustc_expand_base_lib_macros`**: Declared as `proc-macro = true`, with dependencies on `syn = "1.0"`, `quote = "1.0"`, `proc-macro2 = "1.0"`, and `synstructure = "0.12"`. The `syn` version was specifically aligned with `synstructure` to resolve type mismatches. **Crucially, this crate does NOT depend on `rustc_expand_base_lib_errors` to avoid cycles.**
    *   **`rustc_expand_base_lib_test_driver`**: Declared its dependencies on `rustc_expand_base_lib_errors`, `rustc_expand_base_lib_macros`, and `rustc_span`.

4.  **Moved Code:**
    *   `TraceMacroBase` and `TraceMacroNote` structs (without `#[derive(...)]` attributes initially) were moved from `rustc_expand_base_lib/src/errors` to `rustc_expand_base_lib_errors/src/trace_macro_base.rs` and `rustc_expand_base_lib_errors/src/trace_macro_note.rs` respectively.
    *   `rustc_expand_base_lib_errors/src/lib.rs` was updated to re-export these modules and define placeholder `Diagnostic` and `Subdiagnostic` traits along with a `DummyDiagnosticHandler` for testing.

5.  **Implemented Placeholder Derive Macros:** `rustc_expand_base_lib_macros/src/diagnostic.rs` and `subdiagnostic.rs` were updated with basic `synstructure` based implementations that generate `impl` blocks for the placeholder traits, ensuring the macros correctly resolve and apply.

6.  **Adjusted `rustc_expand_base_lib`:** Removed the original `StructTraceMacroBase.rs` and `StructTraceMacroNote.rs` files, and cleared `rustc_expand_base_lib/src/errors/mod.rs` to reflect the new location of these structs. The `ext_ctxt_def.rs` file was reverted to directly instantiate `errors::TraceMacroBase` and `errors::TraceMacroNote`.

7.  **Configured Test Driver:**
    *   `rustc_expand_base_lib_test_driver/src/main.rs` was created to instantiate the structs and apply the derive macros.
    *   The `From<DummySpan> for Span` implementation was simplified.
    *   `extern crate rustc_expand_base_lib_macros;` was added to ensure the macro crate is loaded.
    *   The derive macros were applied using fully qualified paths (e.g., `#[derive(rustc_expand_base_lib_macros::Diagnostic)]`) and the traits were explicitly imported (`use rustc_expand_base_lib_errors::{Diagnostic, Subdiagnostic};`) to resolve all paths correctly.

## Current Status

The build process successfully compiles and runs the `rustc_expand_base_lib_test_driver` executable, confirming that the structural setup for the derive macros is correct. The `#[derive(...)]` attributes are correctly resolved and applied, generating basic `impl` blocks for the `Diagnostic` and `Subdiagnostic` traits. The next phase involves replacing the basic macro implementations with robust code generation logic that parses custom attributes and generates functional diagnostic code.
