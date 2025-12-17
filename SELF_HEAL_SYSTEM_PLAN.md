# Self-Healing Rust Build System: 4-Repo Architecture

This document outlines the architecture for a self-healing Rust build system. The system is split into four focused repositories for modularity, reusability, and clear ownership: a generic core library, a patch application tool, a dataset for machine learning, and a monorepo indexer.

## 1. Core Library: `rust-self-heal-core`

This repository houses reusable macros, types, and binaries for the core pipeline, including error parsing, "ticket" generation, Nix flake synthesis, and patch specifications. It is structured as a Cargo workspace.

- **Crates**:
  | Crate              | Type         | Purpose                                                              |
  |--------------------|--------------|----------------------------------------------------------------------|
  | `self_heal_macros` | `proc-macro` | Provides `patch_build!`, ticket data codegen, and error JSON parsing.  |
  | `self_heal`        | `lib`        | Contains core types (`TicketData`, `ErrorGroup`, `PatchSpec`) and a CLI. |
  | `self_heal_nix`    | `lib/binary` | Generates `flake.nix` files from `Cargo.toml` and error diagnostics.   |

- **Exports**: Macros are re-exported via `pub use self_heal_macros::*;` in `lib.rs`.
- **Binary**: The CLI (`self-heal`) provides commands like `init-tickets` and `gen-flake`.
- **Publishing**: This library will be published to crates.io for downstream consumption.

## 2. Patch Tool: `patch-build-rs` (Existing)

This existing repository serves as the "patch applicator" and self-healing orchestrator, depending on `rust-self-heal-core`.

- **Enhancements**:
  - Integrates `self_heal::TicketData` for reading `cargo_check_output.json`.
  - Adds a `patch_build!` macro to invoke the core error parsing logic.
  - The CLI (`patch-build`) will have a `heal` command (`patch-build heal --crate mycrate`) that reads ticket data, applies patches, and re-runs the build using the generated Nix flake.
- **Structure**:
  ```
  patch-build-rs/
  ├── Cargo.toml (depends on `self-heal`)
  ├── src/bin/patch-build.rs
  └── examples/ticket-data/
  ```

## 3. Dataset Repo: `rust-build-errors-hf`

This is a Hugging Face repository for hosting curated datasets of real-world Cargo JSON errors, corresponding patches, and Nix flakes. This dataset is suitable for training machine learning models (e.g., error -> patch models).

- **Structure**:
  ```
  rust-build-errors-hf/
  ├── README.md              (HF dataset card)
  ├── dataset/
  │   ├── errors.jsonl       (Line-delimited Cargo diagnostics)
  │   ├── patches.toml       (Patch specs from healed builds)
  │   └── flakes/            (Per-crate flake.nix + lock files)
  └── scripts/generate.py    (Script to export data from the self-heal pipeline)
  ```
- **Workflow**: A CI process will run the `self-heal` pipeline on open-source Rust workspaces and upload new tickets to this repository.

## 4. Repo Indexer: `rust-monorepo-indexer`

A new repository to discover, collect, and index massive Rust workspaces, especially those with many git submodules (e.g., `rust-lang/rust`).

| Component          | Purpose                                        | Output                                             |
|--------------------|------------------------------------------------|----------------------------------------------------|
| `indexer` binary   | Crawls git repositories and resolves submodules. | `repos.json` (workspace tree, crate graph).        |
| `workspace-analyzer` | Parses nested `Cargo.toml` files and path deps.| Per-repo statistics (crate count, error-prone deps). |
| `index.db`         | SQLite database for querying workspaces.         | Searchable index of repositories.                  |

- **Workflow**:
  1. The `indexer` clones repositories and their submodules.
  2. It parses `cargo metadata` for each workspace to build a dependency graph.
  3. The collected data is used to generate an index of repositories.
  4. This index can be used to feed the `rust-build-errors-hf` dataset by identifying failing builds to heal.

## Overall Integration Workflow

1.  A user runs `cargo check --message-format=json > cargo_check_output.json` in their workspace.
2.  `self-heal init` (from `rust-self-heal-core`) is run to generate per-crate `ticket-data/` directories, `build.rs` shims, and `flake.nix` files.
3.  `patch-build heal` (from `patch-build-rs`) is invoked to start the self-healing loop: it parses errors, suggests/applies patches, and rebuilds the crate using Nix.
4.  Once a build is healed, the new data (error, patch, flake) is exported and can be added to the `rust-build-errors-hf` dataset.
