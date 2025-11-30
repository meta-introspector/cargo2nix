// tests/trait_fixer_mocks/src/main.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt as MockTyCtxt, DefId as MockDefId, sym as mock_sym, Symbol as MockSymbol};
use trait_fixer_attribute_reader_mock::MockAttributeReaderTyCtxt; // Import the newtype

fn main() {
    println!("Running mock attribute reader test...");

    let mock_tcx_instance = MockTyCtxt(std::marker::PhantomData);
    let mock_tcx_wrapper = MockAttributeReaderTyCtxt(mock_tcx_instance); // Wrap the mock TyCtxt
    let mock_def_id = MockDefId;

    // Call has_derive_attr on the wrapper, passing its own DefId and using its Symbol interning
    let has_derive = mock_tcx_wrapper.has_derive_attr(mock_def_id, "Debug");

    assert_eq!(has_derive, true, "Mock AttributeReader should always return true for has_derive_attr");

    println!("Mock attribute reader test passed!");
}