# rust-mcp-server Onboarding Guide

This guide provides instructions on how to set up the development environment, build, and run the `rust-mcp-server` in its various modes.

## 1. Introduction

The `rust-mcp-server` is a core component for the Monster Composability Proof (MCP) system. It can operate as a Language Server Protocol (LSP) server, perform direct Rust file analysis, and dynamically load and execute plugins. It uses RocksDB for persistent storage of plugin metadata and other relevant data.

## 2. Prerequisites

*   **Nix:** Ensure Nix is installed on your system. This project relies heavily on Nix flakes for reproducible development environments.

## 3. Environment Setup

To enter the development environment with all necessary tools and dependencies (including the Rust toolchain), use the following command from the project root:

```bash
nix develop
```

This command will drop you into a shell where `cargo` and other development tools are available.

## 4. Building `rust-mcp-server`

Once inside the Nix development shell, navigate to the `rust-mcp-server` crate and build it using Cargo:

```bash
cd crates/rust-mcp-server
cargo build
```

This will compile the `rust-mcp-server` executable and place it in `target/debug/rust-mcp-server`.

## 5. Running Modes

The `rust-mcp-server` supports several operational modes:

### 5.1. Direct File Analysis

This mode allows you to analyze a Rust source file directly from the command line, bypassing the LSP. It will parse the file and extract information, such as function names.

To run `rust-mcp-server` in this mode, specifying its own `main.rs` as an example:

```bash
cargo run --bin rust-mcp-server -- --file crates/rust-mcp-server/src/main.rs
```

The output will be a JSON object containing the collected function names.

### 5.2. Dynamic Plugin Execution

The server can load and execute dynamic plugins that adhere to the `mcp-plugin-traits` interface. First, ensure the example plugin is built:

```bash
cd crates/mcp-plugin-example
cargo build --lib
cd ../../ # Go back to project root
```

Then, run `rust-mcp-server` specifying the path to the compiled plugin:

```bash
cargo run --bin rust-mcp-server -- --plugin-path target/debug/libmcp_plugin_example.so
```

The server will load the plugin, execute its example logic, and store/retrieve its metadata from RocksDB.

### 5.3. LSP Server

In this mode, `rust-mcp-server` acts as a Language Server, communicating with an LSP client (e.g., an IDE like VS Code, Neovim) over `stdin`/`stdout`. To start the server in LSP mode (without specific CLI arguments):

```bash
cargo run --bin rust-mcp-server
```

This command will typically be executed by your LSP client.

## 6. RocksDB Data

The `rust-mcp-server` uses RocksDB for persistent storage. You will find a directory named `mcp_db/` in the project root after running the server. This directory contains the database files used to store plugin metadata and morphological indices.

---
**Note:** The dynamic plugin loading mechanism uses `unsafe` Rust. Ensure any custom plugins are thoroughly tested and reviewed for safety.
