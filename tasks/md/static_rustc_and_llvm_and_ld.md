# Idea: Static Rustc, LLVM, and LD with Trait-Based Syscall Replacement

## Description

This document outlines a long-term architectural vision for the `cargo2nix` project and its associated tooling. The core idea is to achieve a highly controlled, reproducible, and auditable build environment by integrating `rustc` (and potentially LLVM and `ld`) as a library, rather than invoking them as external processes. Furthermore, all system calls (syscalls) would be abstracted behind traits, allowing for static linking and complete control over I/O and environmental interactions.

## Motivation

The primary motivations behind this approach are:

1.  **Ultimate Reproducibility**: By statically linking `rustc` and controlling all syscalls, the build process becomes entirely deterministic. External factors like system libraries, environment variables, or even the specific version of `ld` on the host system would no longer influence the build output. This is crucial for projects aiming for bit-for-bit reproducible builds, especially within a Nix-based ecosystem.

2.  **Enhanced Security and Auditability**: Abstracting syscalls behind traits provides a clear interface for all interactions with the operating system. This allows for:
    *   **Fine-grained Control**: Restricting or modifying specific I/O operations.
    *   **Auditing**: Logging every system interaction for security and debugging purposes.
    *   **Sandboxing**: Implementing custom sandboxing mechanisms at a very low level, without relying on external tools.

3.  **Performance Optimization**: Direct library calls to `rustc` and `ld` can potentially reduce process overhead compared to spawning external commands. Furthermore, controlling syscalls allows for optimizations like caching I/O operations or implementing custom virtual file systems.

4.  **Integration with Nix**: This approach aligns perfectly with the Nix philosophy of reproducible builds. By making the Rust compilation process self-contained and auditable, it becomes easier to define precise Nix derivations that capture the entire build graph.

5.  **Trait-Based Flexibility**: The existing trait-based architecture (e.g., `Execv`, `FileSystemWriter`) already lays the groundwork for abstracting system interactions. Extending this to cover all syscalls would provide a consistent and extensible framework.

## Key Components and Approach

1.  **`rustc` as a Library**:
    *   Investigate the feasibility of integrating `rustc` as a Rust library. This would involve understanding `rustc`'s internal APIs and how to invoke its compilation stages programmatically.
    *   This might require significant upstream work or a custom fork of `rustc` if its APIs are not sufficiently stable or exposed.

2.  **LLVM and `ld` Integration**:
    *   Similarly, explore integrating LLVM (the backend for `rustc`) and the linker (`ld`) as libraries. This would provide even deeper control over the compilation and linking process.
    *   This is a more ambitious step and might depend on the success of integrating `rustc` as a library.

3.  **Syscall Abstraction Traits**:
    *   Define a comprehensive set of Rust traits that represent common system calls (e.g., `ReadTrait`, `WriteTrait`, `StatTrait`, `ExecTrait`).
    *   These traits would abstract operations like file I/O, process execution, network communication, and time queries.

4.  **Static Linking**:
    *   The goal is to statically link all necessary components (including `rustc`, LLVM, `ld`, and custom syscall implementations) into a single, self-contained binary. This eliminates runtime dependencies on system libraries.

5.  **"Real" and "Mock" Implementations**:
    *   Provide "real" implementations of the syscall traits that interact with the actual operating system (e.g., using `std::fs`, `std::process`).
    *   Provide "mock" or "virtual" implementations for testing, dry-run scenarios, or custom sandboxing environments. This is where the "virtual canonical file system" concept would come into play.

6.  **Virtual Canonical File System (VCFS)**:
    *   Develop a VCFS that can be mapped into Nix. This VCFS would be the sole source of truth for all file system interactions during the build, ensuring that `rustc` and other tools operate on a precisely defined and auditable environment.

## Challenges

*   **Complexity of `rustc` Internals**: Integrating `rustc` as a library is a highly complex task due to its intricate internal architecture and rapid development.
*   **LLVM/LD Integration**: Similar challenges apply to LLVM and `ld`.
*   **Comprehensive Syscall Abstraction**: Defining and implementing traits for all relevant syscalls is a massive undertaking.
*   **Performance Overhead**: While the goal is performance optimization, the initial implementation of syscall abstraction might introduce overhead that needs to be carefully managed.
*   **Upstream Compatibility**: Maintaining compatibility with upstream `rustc` and LLVM changes would be a continuous effort.

## Conclusion

This vision represents a significant leap towards achieving ultimate reproducibility, security, and control over the Rust build process within the Nix ecosystem. While challenging, the benefits in terms of build integrity, auditability, and deterministic behavior are substantial. The current trait-based refactoring of `cargo-repo-sync` is a foundational step towards realizing this ambitious goal.
