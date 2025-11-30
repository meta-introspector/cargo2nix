# Trait Fixer Tool (`trait-fixer`)

## Overview
The `trait-fixer` is an experimental Rust tool designed to automatically identify and resolve trait-related issues within Rust codebases. It aims to act as a "clippytask"-like utility, potentially integrating into the project's build and continuous integration pipelines to enforce trait correctness and facilitate migrations.

## Technology Stack
The tool is built using Rust and leverages several internal `rustc_` crates, such as `rustc_driver` and `rustc_ast`, to gain deep insight and control over the Rust compiler's internal representations. This approach allows for sophisticated analysis and programmatic modification of code.

## Current Development Status
The `trait-fixer` is currently under active development. The primary focus is on establishing a stable build environment, which involves correctly configuring `rustc_` dependencies and addressing challenges related to unstable Rust features. This requires a nightly Rust toolchain and specific compiler flags (`RUSTFLAGS`) to enable necessary unstable functionalities and ensure successful compilation.

## Planned Usage
Once stable, the `trait-fixer` is envisioned to be used as:
*   **Code Linting/Fixing:** Automatically suggesting or applying fixes for common trait-related problems.
*   **Migration Assistant:** Helping adapt codebases to new trait system features or changes.
*   **Integration with Build Systems:** Operating as part of a pre-commit hook or CI/CD pipeline to maintain code quality.

## Build Considerations
Due to its reliance on internal compiler components and unstable features, building `trait-fixer` requires:
*   A **nightly Rust toolchain**.
*   The `RUSTC_BOOTSTRAP=1` environment variable.
*   A comprehensive set of `RUSTFLAGS` to enable unstable features (e.g., `debug_closure_helpers`, `new_zeroed_alloc`), specific linting rules, and compilation options tailored for internal `rustc_` usage. Path overrides are configured in `.cargo/config.toml` to point to the local `rust` submodule.