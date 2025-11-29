# CRQ-040: Rusttycoon Factory Regeneration

## Objective
Refactor the `rusttycoon` crate to implement an ABI-compatible, dynamically loaded "Factory" system for its "FactoryBlocks." This involves defining a common Application Binary Interface (ABI) for blocks, enabling `rusttycoon` to dynamically load and manage these blocks as shared libraries, thereby achieving maximal modularity, configurability, and adherence to the "ABI syscall trait" constraint.

## Current Status and Motivation (from tmux_output.log)
Initial compilation errors (E0432) in `crates/rusttycoon/src/factory_blocks/media_workflow_blocks.rs` and `crates/rusttycoon/src/factory.rs` indicated issues with module imports and re-exports, stemming from an attempt to consolidate `factory_blocks` modules. The user's directive to "comment out all failing code in tycoon its too large" and "regenerate the factory with the factory" shifted the focus from immediate compilation fixes to a fundamental architectural change. The new approach prioritizes an ABI-based system for `FactoryBlocks` to align `rusttycoon` with the overarching goal of "self-referential meta-programming approach aligned with the rusttycoon ABI syscall trait constraint."

## Plan: Establish ABI and Dynamic Loading

### Phase 1: Establish ABI and Dynamic Loading
1.  **Define `factory_abi` crate**: Create `crates/factory_abi` with a C-compatible `FactoryBlock` interface using `#[no_mangle]` and `extern "C"` functions. This interface will define methods like `name()`, `cost()`, and `execute()`.
    *   **Progress**: `crates/factory_abi` directory, `Cargo.toml`, and `src/lib.rs` created. `src/lib.rs` defines `FactoryContext`, `FactoryBlockGetName`, `FactoryBlockGetCost`, `FactoryBlockExecute` types, and `AbiFactoryBlock` struct, along with `to_c_string` and `from_c_string` helpers.
2.  **Modify `rusttycoon` to use `factory_abi`**:
    *   Add `factory_abi` as a dependency in `crates/rusttycoon/Cargo.toml`.
        *   **Progress**: `crates/rusttycoon/Cargo.toml` updated to include `factory_abi = { path = "../factory_abi" }`.
    *   Remove all `use crate::factory_blocks::...` imports from `crates/rusttycoon/src/factory.rs`. (This will be done after `libloading` is integrated).
    *   Modify `get_available_tools()` to dynamically load shared libraries (e.g., using `libloading`) via FFI calls.
        *   **Progress**: `libloading = "0.7"` added as a dependency to `crates/rusttycoon/Cargo.toml`. Next step is to modify `crates/rusttycoon/src/factory.rs` to use `libloading`.
3.  **Convert one existing `FactoryBlock` to a shared library**: Move a simple `FactoryBlock` (e.g., `ConveyerBeltBlock`) to its own `crates/conveyer_belt_block` crate, implement the `factory_abi` interface, and compile it as a `cdylib`. Ensure `rusttycoon` can load and execute it.

### Phase 2: Implement "FactoryBlock Generator"
4.  **Design "Block Definition" format**: Create a simple TOML/YAML configuration for defining new blocks.
5.  **Implement `FactoryBlockFactoryBlock` in an external crate**: This crate will depend on `factory_abi`. Its `execute` method will generate Rust code for new blocks based on the definition, compile it to a shared library, and load it into the Factory.

## Benefits
*   **Dynamic Modularity**: `FactoryBlocks` can be developed, compiled, and deployed independently of `rusttycoon`, enabling hot-swapping and extending functionality at runtime.
*   **Reduced Compile Times**: `rusttycoon`'s compile time will be decoupled from the number and complexity of `FactoryBlocks`.
*   **ABI Compliance**: Strict adherence to the ABI constraint, allowing for broader interoperability and future-proofing.
*   **Self-Generation**: Lays the groundwork for `FactoryBlocks` to generate and load other `FactoryBlocks`, fulfilling the "regenerate the factory with the factory" directive.

## Assigned To
Gemini

## Status
In Progress

## Dependencies
*   `tasks/md/cargo-eigenform.md` (High-level strategy)
*   `tasks/md/cargo-hecke.md` (High-level strategy)
*   `tasks/md/purification_of_cargo_build.md` (General refactoring context)

## Related Files
*   `crates/factory_abi/`
*   `crates/rusttycoon/Cargo.toml`
*   `crates/rusttycoon/src/factory.rs`
*   `tmux_output.log` (for detailed trace of initial errors and plan formulation)
