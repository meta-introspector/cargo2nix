# Build Process Learnings and Fixes

This document outlines the issues encountered and solutions implemented during the build process, particularly concerning the interaction between Rust's `cargo build` and the Nix development environment (`flake.nix`).

## Key Learnings and Fixes

1.  **`RUSTC_BOOTSTRAP` Environment Variable**:
    *   **Issue**: Initial `cargo build` attempts failed with an error indicating that `rustc_macros` could not be built without going through a bootstrap process. The error message explicitly suggested setting the `RUSTC_BOOTSTRAP` environment variable.
    *   **Resolution**: Setting `RUSTC_BOOTSTRAP=1` allowed the `rustc_macros` and other related compiler components to build successfully. This variable is now set in the `shellHook` of the relevant Nix development shells.

2.  **`text-size` Crate Version Conflict**:
    *   **Issue**: A `mismatched types` error occurred in `rust-analyzer` submodules (specifically `syntax-bridge` and `span`) because two different versions of the `text-size` crate were being used. One version was pulled from `crates.io` by a direct dependency, while another was being referenced as a local submodule within the workspace.
    *   **Resolution**: Modified `submodules/rowan/Cargo.toml` (which was transitively depending on `text-size`) to use `text-size.workspace = true`. This forced all relevant crates within the workspace to use the single, canonical `text-size` definition provided by the workspace's root `Cargo.toml` (which points to `submodules/text-size`).

3.  **Nix `flake.nix` `shellHook` Parameter Expansion Syntax**:
    *   **Issue**: Attempts to conditionally append to environment variables like `PKG_CONFIG_PATH` and `PATH` within the `flake.nix`'s `shellHook` using Bash parameter expansion syntax like `${VAR:+:}$VAR` directly led to Nix syntax errors (`unexpected '+'`, `unexpected ':'`). The Nix parser was trying to interpret the Bash syntax before the shell executed the string.
    *   **Resolution**: The correct and Nix-friendly way to handle conditional Bash parameter expansion inside `shellHook` strings is to escape the Bash expansion using `''${VAR:+:}$VAR`. This ensures that Nix treats the expansion literally as part of the string, which is then correctly interpreted by Bash when the shellHook executes.
        *   Example for `PKG_CONFIG_PATH`: `export PKG_CONFIG_PATH="${pkgs.openssl_1_1.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:}$PKG_CONFIG_PATH";`
        *   Example for `PATH`: `export PATH="${myRustc}/bin:${pkgs.cargo}/bin''${PATH:+:}$PATH";`

4.  **Makefile Integration with Nix Development Shells**:
    *   **Issue**: Initially, the `flake.nix`'s `shellHook` was configured to automatically run `make build`. This was undesirable due to side-effects upon simply entering `nix develop`. Also, the `Makefile`'s `build` target itself was redundantly calling `nix develop ... --command cargo build`, leading to nested Nix shell invocations.
    *   **Resolution**:
        *   Removed `make build` from the `shellHook` of the default `devShells.default` in `flake.nix` to prevent side-effects.
        *   Created a new `devShells.build` output in `flake.nix` that explicitly includes `make build;` in its `shellHook`. This allows users to opt into an environment that automatically builds by running `nix develop .#build`.
        *   Simplified the `build`, `nix-cargo-build`, and `nix-build` targets in the `Makefile` to directly call `cargo build` (e.g., `build: cargo build`). This avoids redundant `nix develop` calls within the Makefile targets.
        *   Removed redundant `nix-dev-build` and `run-nix-build` targets from the `Makefile`.

5.  **LLVM Configuration for `rustc_llvm`**:
    *   **Issue**: The `rustc_llvm` build script required specific environment variables (`LLVM_CONFIG`, `REAL_LIBRARY_PATH_VAR`, `REAL_LIBRARY_PATH`) to locate LLVM components. Without these, it would panic with errors like `LLVM_CONFIG was not set` or `REAL_LIBRARY_PATH_VAR`.
    *   **Resolution**: `pkgs.llvm` was added to the `packages` list in `flake.nix`. The `shellHook` for both `devShells.default` and `devShells.build` now correctly exports these variables, with `LLVM_CONFIG` pointing to `${pkgs.llvm}/bin/llvm-config`, and `REAL_LIBRARY_PATH_VAR` and `REAL_LIBRARY_PATH` correctly set for Linux environments.

## Current Status

The Nix flake parsing issues have been resolved, and the environment variables necessary for `rustc_llvm` and other components are correctly set within the `nix develop .#build` shell. The `make build` command is successfully initiated.

The build process (specifically `cargo build` invoked via `make build`) is still exiting with `Error 101`, indicating a failure within `cargo build`. The detailed reason for this failure needs further investigation, as `make` itself does not provide the underlying `cargo build` stderr output in the current configuration.
