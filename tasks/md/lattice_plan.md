# Lattice Transformation Plan

## High-Level Goal
The primary objective is to transform the current "lattice of crates" into a more flexible and efficient "lattice of features," ultimately enabling representation as a "string of bit flags." This transformation aims to improve configuration management, dependency resolution, and feature toggling within the project's Rust ecosystem.

## Hecke Operator Concept
The "Hecke operator" is a conceptual mechanism employed to guide each crate towards its "pure mathematical eigenform." This involves a systematic refactoring process that emphasizes minimalism, abstraction, and trait-based interfaces. The operator facilitates the conversion of concrete implementations into abstract, interchangeable components.

## Eigenform Traits
Each crate's public interface will be represented by a "vernacular trait" (a native trait bundle). These vernacular traits will serve as the abstract contract for the crate's functionality. To support various use cases and testing scenarios, each trait will eventually have multiple versions:
*   **Mock Version**: For testing and isolated development.
*   **Serde Version**: For serialization and deserialization capabilities.
*   **External Syscall Version**: For interacting with the underlying operating system or external processes.
*   **Cargo Lib Version**: The standard library implementation.
*   **Shared Object Version**: For dynamic linking and plugin architectures.

## Transformation Strategy: Bottom-Up Approach
The transformation will proceed in a bottom-up manner, starting with foundational crates that have minimal or no dependencies on other workspace crates. This ensures that the base layers are stable and trait-ified before higher-level components are addressed.

## Current Focus: `git-wrapper-lib`
The `git-wrapper-lib` crate has been identified as the first target for this transformation due to its foundational role in interacting with Git operations.

### Steps for `git-wrapper-lib` Transformation:

1.  **Define `GitWrapperLibTrait`**: Create a comprehensive vernacular trait that encapsulates all core Git operations provided by the `git-wrapper-lib` crate. This trait will combine access to lower-level traits suchs as `GitExecutor`, `GitRepositoryOperations`, `GhExecutor`, `Execv`, and `GitAdapter`.

2.  **Create `RealGitWrapperLib`**: Develop a concrete implementation of `GitWrapperLibTrait` that utilizes the existing real-world implementations (e.g., `SystemGitExecutor`, `RealGitRepositoryOperations`, `SystemGhExecutor`, `RealExecv`, `ShellGitAdapter`).

3.  **Create `MockGitWrapperLib`**: Develop a mock implementation of `GitWrapperLibTrait` that uses mock versions of its constituent traits (e.g., `DummyGitExecutor`, `MockGitRepositoryOperations`, `MockGhExecutor`, `MockExecv`, `MockGitAdapter`).

4.  **Create Mock Implementations for Sub-Traits**: Ensure that mock implementations exist for all lower-level traits (`Execv`, `GitRepositoryOperations`, `GhExecutor`, `GitExecutor`, `GitAdapter`) that `GitWrapperLibTrait` provides access to.

5.  **Refactor Internal Code to Use `GitWrapperLibTrait`**: Modify the internal components of `git-wrapper-lib` (e.g., `SubmoduleManager`) to depend on `Arc<dyn GitWrapperLibTrait>` rather than directly on concrete implementations or individual trait objects. This promotes dependency inversion and testability.

6.  **Make External Dependencies Optional via Features**: Convert all external crate dependencies (e.g., `anyhow`, `walkdir`, `regex`, `toml_edit`, `pathdiff`, `serde`, `serde_json`, `git2`, `sha1`, `hex`) into optional features within `git-wrapper-lib/Cargo.toml`.

7.  **Conditionally Compile Code Based on Features**: Implement `#[cfg(feature = "...")]` attributes throughout the `git-wrapper-lib` codebase to ensure that code sections relying on optional dependencies are only compiled when their respective features are enabled. This includes conditional imports, type definitions, and function implementations, with appropriate fallbacks (e.g., `std::result::Result` for `anyhow::Result`).

## Future Steps
Once `git-wrapper-lib` has been successfully transformed, the same systematic approach will be applied to other foundational crates within the workspace, gradually moving up the dependency graph. This iterative process will ensure that the entire crate ecosystem evolves towards the desired "lattice of features" and "string of bit flags" representation.
