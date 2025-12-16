# Pure Type Manifesto

This document outlines an architectural principle for highly decoupled and robust Rust code, particularly within complex systems like compilers. The core idea is to treat every logical operation or transformation as a method of its own trait, promoting explicit dependencies and clear contracts between components.

## Principles:

1.  **Each Function is a Trait Method:**
    *   Every distinct, testable unit of functionality that operates on `self` (or would logically be a method) *must* be defined as a method within its own dedicated trait.
    *   For example, instead of a free function `do_something(my_struct: &mut MyStruct)`, define a trait `DoSomething` with a method `do_something(&mut self)`.

2.  **Trait-Based Interaction Only:**
    *   Functions (now trait methods) should interact with other functionalities exclusively through traits. Direct calls to concrete types or free functions from other modules should be minimized.
    *   If `TraitA::method_a` needs to call `TraitB::method_b`, then the trait `TraitA` (or its `impl` block) must have a way to access an object that implements `TraitB`. This often means `TraitA` will have `where Self: TraitB` bounds, or the method will take a `&impl TraitB` parameter.

3.  **One Trait, One `impl` per Functionality:**
    *   Ideally, each trait representing a single function or small group of related functions should have one primary implementation for a core type. This keeps the trait's purpose focused.
    *   Avoid monolithic traits that group many unrelated methods.

4.  **Wrap Other Types:**
    *   To enforce clear boundaries and prevent accidental direct access to internal details of external types (especially those from other crates or complex compiler structures), wrap these types in newtype structs.
    *   For example, if a function needs `rustc_ast::Expr`, define `pub struct WrappedExpr(pub rustc_ast::Expr);` and implement conversion traits (`From`, `Into`) as needed. Operations on `WrappedExpr` should expose only what's necessary, often by requiring traits for specific functionalities.

5.  **Minimize Direct Struct Access:**
    *   Access to fields of a struct should primarily be through getter/setter methods, especially for public fields of types that are part of an API. This allows for future refactoring without breaking external users.
    *   Minimize `pub` fields, especially for types crossing module/crate boundaries.

6.  **Automation via Clippy Macros (Future Goal):**
    *   The manual application of these rules is arduous. The long-term vision is to develop `clippy` macros or similar tooling that can analyze code, suggest refactorings to adhere to these principles, and potentially automate parts of the conversion.

## Benefits:

*   **Extreme Decoupling:** Components are independent, reducing ripple effects of changes.
*   **Enhanced Testability:** Mocking and testing individual functionalities becomes trivial by providing alternative trait implementations.
*   **Clear Contracts:** Traits explicitly define the capabilities required and provided by components.
*   **Improved Maintainability:** Code is easier to understand, navigate, and refactor.
*   **Robustness:** Reduces implicit dependencies and makes unexpected interactions less likely.

This manifesto is a guiding principle to evolve the architecture towards a more functional, composable, and testable design.
