# `cargo-feature-adapter` Tool Plan

## Goal
The `cargo-feature-adapter` tool aims to automate the process of transforming an existing Rust crate (typically a submodule or a foreign dependency) into an "adaptive crate." An adaptive crate is one where its external dependencies are feature-gated, allowing for conditional compilation and fine-grained control over its build characteristics based on enabled features. This facilitates the "lattice of features" concept for efficient configuration management and dependency resolution.

## Core Functionality

1.  **Input**:
    *   Path to the original crate (e.g., a submodule directory).
    *   Path to the output directory where the new adaptive crate will be generated.
    *   (Optional) A configuration file specifying advanced adaptation rules (e.g., specific dependencies to ignore, custom feature names).

2.  **Output**:
    *   A new Rust crate directory at the specified output path.
    *   The new crate's `Cargo.toml` will have its dependencies marked as `optional = true` and new `with-<dependency-name>` features generated.
    *   The new crate's source code (`src/`) will be transformed to include `#[cfg(feature = "with-<dependency-name>")]` attributes around code that uses the now-optional dependencies.

## Detailed Implementation Plan

### Phase 1: Design and Setup `cargo-feature-adapter` Crate (Completed)

*   **Crate Creation**: Created the `tools/cargo-feature-adapter` directory.
*   **`Cargo.toml` Setup**: Configured `tools/cargo-feature-adapter/Cargo.toml` with necessary dependencies:
    *   `syn`, `quote`, `proc-macro2` (for Rust AST manipulation).
    *   `toml_edit` (for `Cargo.toml` parsing and modification).
    *   `walkdir` (for traversing source directories).
    *   `fs_extra` (for copying files).
    *   `anyhow` (for error handling).
    *   `clap` (for command-line interface).
    *   `sha1`, `hex` (for source hashing/caching).
    *   `serde`, `serde_json` (for configuration and reporting).
*   **Workspace Integration**: Added `cargo-feature-adapter` to the root `Cargo.toml`'s `[workspace.members]` section.

### Phase 2: Implement `Cargo.toml` Analysis and Transformation (In Progress)

This phase focuses on adapting the `Cargo.toml` file of the target crate.

1.  **`cargo_toml_adapter.rs` Module**:
    *   **Function**: `adapt_cargo_toml(input_path: &Path, output_path: &Path) -> Result<()>`
    *   **Read Original `Cargo.toml`**: Reads and parses the `Cargo.toml` from `input_path` using `toml_edit`.
    *   **Identify Dependencies**: Iterates through `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` sections.
    *   **Generate New Features**: For each identified dependency (e.g., `foo`), it generates a corresponding feature named `with-foo` (or `with_foo` if `foo` contains hyphens). This feature will enable the `dep:foo` dependency.
    *   **Modify Dependencies**: In the new `Cargo.toml`, all original dependencies will be marked as `optional = true`.
    *   **Add New Features Section**: A `[features]` section will be created (if it doesn't exist) and populated with the generated `with-<dependency-name>` features.
    *   **Set Default Features**: A `default` feature will be defined that enables a sensible subset of the generated `with-` features (e.g., all of them, or a predefined set).
    *   **Rename Package**: The package name in the new `Cargo.toml` will be modified (e.g., `original-crate` becomes `original-crate-adaptive`).
    *   **Write New `Cargo.toml`**: The modified `Cargo.toml` will be written to `output_path/Cargo.toml`.

2.  **`main.rs` Integration**:
    *   The `main.rs` of `cargo-feature-adapter` will call `cargo_toml_adapter::adapt_cargo_toml` to perform this step.

### Phase 3: Implement Source Code Transformation

This is the most complex phase, involving programmatic rewriting of Rust source code.

1.  **Copy Source Files**: Recursively copy the `src` directory from `input_crate_path` to `output_crate_path`.
2.  **Process Each Rust Source File**:
    *   **Read Content**: Read the content of each `.rs` file.
    *   **Parse AST**: Use `syn::parse_file` to parse the content into a `syn::File` (Rust AST).
    *   **AST Traversal and Transformation**: Implement a `syn::visit_mut::VisitMut` visitor to traverse and modify the AST. Key transformations will include:
        *   **`use` statements**: Identify `use` statements that import items from now-optional dependencies. Wrap these `use` statements with `#[cfg(feature = "with-<dependency-name>")]`.
        *   **Item-level Conditional Compilation**: For top-level items (functions, structs, enums, modules, etc.) that *directly* depend on an optional feature, wrap them with `#[cfg(feature = "with-<dependency-name>")]`.
        *   **Expression-level Conditional Compilation**: For expressions or statements within functions that use items from optional dependencies, wrap them with `#[cfg(feature = "with-<dependency-name>")]`. This might require more advanced AST analysis to determine the smallest possible code block to wrap.
        *   **Fallback Implementations**: For code paths that become unreachable when a feature is disabled, generate appropriate fallback code (e.g., `unimplemented!()`, dummy types, or alternative logic). This will be highly context-dependent and might require a configurable approach.
    *   **Rewrite and Output**: Use `quote::quote!` to convert the modified `syn::File` back into Rust code and write it to the corresponding file in the `output_crate_path/src` directory.

### Phase 4: Integration and Testing

1.  **CLI Interface**: Enhance `main.rs` to provide a user-friendly CLI using `clap` for specifying input/output paths and configuration.
2.  **Testing**:
    *   Create unit tests for `cargo_toml_adapter` and the source code transformation logic.
    *   Use `cargo-feature-adapter` to adapt a simple test crate.
    *   Use the `feature-permutation-builder` to test the newly adapted `git-wrapper-lib` (or other target crates) to verify that the feature-gating works as expected and that build times/sizes are affected correctly.

## Challenges and Considerations

*   **Complexity of AST Transformation**: Identifying all usages of an external crate within arbitrary Rust code is non-trivial. This will require careful design of the AST visitor and potentially heuristics.
*   **Fallback Logic**: Generating meaningful fallback implementations when features are disabled will be a significant challenge and might require user-defined rules.
*   **Macro Expansion**: `syn` operates on the raw AST, not the expanded AST. This means macros from optional dependencies might not be correctly identified or transformed without additional steps (e.g., a separate macro expansion phase).
*   **Error Handling**: Robust error handling throughout the process is crucial.

This ambitious plan will enable the creation of highly configurable and optimized Rust crates, aligning with the project's long-term goals.
