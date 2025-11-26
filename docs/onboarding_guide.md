# Onboarding Guide for `cargo2nix` within the Meta-Introspector Project

Welcome to the `cargo2nix` submodule within the broader Meta-Introspector project! This guide is designed to help new contributors understand how `cargo2nix` integrates into our Nix-centric development workflow, focusing on Nixification, flake management, and external dependency policies.

## 1. Project Context: Nixification and CRQ-016

The Meta-Introspector project is undergoing a significant "Submodule Nixification and Flake Refactoring" initiative (CRQ-016). This aims to standardize our development environment, build processes, and dependency management using Nix, ensuring reproducibility and consistency across all submodules, including `cargo2nix`.

`cargo2nix` plays a critical role in this by providing robust Nix dependency management for Rust projects. Its evolution is aligned with our goal of building a quasi-meta computationally self-aware system, eventually replacing shell-based orchestration with Rust-based solutions for deeper, formally verifiable integration.

## 2. Setting Up Your Development Environment

Our project leverages Nix flakes to create reproducible development environments.

### Using `nix develop`

To enter the `cargo2nix` development shell, navigate to the `cargo2nix` directory and run:

```bash
nix develop
```

This command uses the `flake.nix` file in the current directory to set up all necessary tools and dependencies (e.g., Rust toolchain, `cargo`, `openssl`, `zlib`, `sccache`, `llvm`, `libclang`, `clang`, `pkg-config`, `minizinc`, `gecode`, `rust-analyzer`).

### Understanding `flake.nix`

The `flake.nix` file defines the development environment and how `cargo2nix` integrates with the Nix ecosystem. Key aspects include:

*   **Inputs:** All external dependencies, such as `nixpkgs`, `rust-overlay`, `flake-utils`, and `cargo2nix` itself, are sourced from `github:meta-introspector` repositories. This adheres to our project's policy for integrating external dependencies, ensuring a controlled and consistent supply chain. For example, `cargo2nix` is fetched from `github:meta-introspector/cargo2nix/release-0.12`.
*   **Outputs:** The flake defines `devShells` and `packages`, making the development environment and `cargo2nix`'s build outputs readily available.

## 3. Managing Rust Dependencies with `cargo-repo-sync`

`cargo-repo-sync` is a crucial tool for streamlining Git operations and managing Rust dependencies within our Nix ecosystem.

### Key Features and Project Alignment:

*   **Super Fast Resolution System (CRQ-016 Related):** `cargo-repo-sync` utilizes a `rollup.lock` file to cache metadata of `Cargo.toml` and `Cargo.lock` files. This enables conditional `Cargo.nix` generation, significantly speeding up builds. This system is a core component of CRQ-016, enabling efficient dependency management across submodules.
*   **Unified Dependency Management (CRQ-016 & `github:meta-introspector` Alignment):** `cargo-repo-sync` aims to centralize Rust dependency management across submodules, generating a single, unified `Cargo.nix` and `flake.nix`. This vision directly supports CRQ-016 objectives for Submodule Nixification and Flake Refactoring, and leverages the `github:meta-introspector` policy for integrating external dependencies in a controlled and consistent manner.
*   **Automated Git Operations:** It automates the lifecycle of vendored Git repositories (submodules), including adding, committing, branching, and pushing, ensuring our controlled integration policy.

### Basic Usage:

```bash
# Enter the development shell
nix develop

# Run cargo-repo-sync (e.g., for dry run or plan generation)
cargo repo-sync --dry-run
cargo repo-sync plan generate
```

## 4. Building with Nix

Once `Cargo.nix` files are generated, you can build your Rust projects using Nix:

```bash
# Ensure flake.nix and Cargo.nix are in version control
git add flake.nix Cargo.nix

# Build the default package defined in your flake
nix build
```

## 5. External Dependency Integration Policy (`github:meta-introspector`)

A fundamental policy of the Meta-Introspector project is that **all external dependencies are integrated via `github:meta-introspector` URLs**. This means:

*   **No Direct External References:** You will not directly reference `github.com/original-author/repo` in our `flake.nix` files or other Nix expressions.
*   **Internal Mirroring:** External projects are mirrored or forked into the `github:meta-introspector` organization, allowing us to maintain control, apply specific patches, and ensure long-term availability and reproducibility.
*   **Submodule Usage:** Submodules are primarily reserved for scenarios involving *editing, pushing, and tagging* of those external repositories, not for general integration.

This policy is crucial for maintaining the integrity, security, and reproducibility of our entire system.

## 6. Contribution Guidelines

For general contribution guidelines, including our Change Request (CRQ) and Standard Operating Procedure (SOP) processes, please refer to the main project's documentation.
