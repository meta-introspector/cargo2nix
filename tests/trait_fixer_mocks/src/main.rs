// tests/trait_fixer_mocks/src/main.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt as MockTyCtxt, DefId as MockDefId, sym as mock_sym, Symbol as MockSymbol};
use trait_fixer_attribute_reader_mock::MockAttributeReaderTyCtxt; // Import the newtype

// For CompilerHost test
use trait_fixer_compiler_host_trait::CompilerHost;
use trait_fixer_compiler_host_mock::MockCompilerHost;
// We need a mock Callbacks type that does not depend on rustc_driver
// Let's create a simple one.
struct MockCompilerCallbacks; // Simple empty struct

impl MockCompilerCallbacks {
    pub fn new() -> Self {
        MockCompilerCallbacks
    }
}

fn main() {
    println!("--- Running mock attribute reader test ---");

    let mock_tcx_instance = MockTyCtxt(std::marker::PhantomData);
    let mock_tcx_wrapper = MockAttributeReaderTyCtxt(mock_tcx_instance); // Wrap the mock TyCtxt
    let mock_def_id = MockDefId;

    // Call has_derive_attr on the wrapper
    let has_derive = mock_tcx_wrapper.has_derive_attr(mock_def_id, "Debug");

    assert_eq!(has_derive, true, "Mock AttributeReader should always return true for has_derive_attr");

    println!("Mock attribute reader test passed!\n");

    println!("--- Running mock compiler host test ---");

    let compiler_host_mock = MockCompilerHost;
    let mut mock_callbacks = MockCompilerCallbacks::new(); // Use our mock callbacks
    let args = vec!["arg1".to_string(), "arg2".to_string()];

    // Call run_compiler_callbacks
    compiler_host_mock.run_compiler_callbacks(args, &mut mock_callbacks);

    println!("Mock compiler host test finished (output above)");
    println!("Mock compiler host test passed!\n");
}
