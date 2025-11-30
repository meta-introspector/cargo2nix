// tests/trait_fixer_mocks/src/main.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym, Symbol};
use trait_fixer_attribute_reader_mock::MockAttributeReaderTyCtxt; // Import the newtype

fn main() {
    println!("Running mock attribute reader test...");

    let raw_mock_tcx = TyCtxt(std::marker::PhantomData);
    let mock_tcx_wrapper = MockAttributeReaderTyCtxt(raw_mock_tcx); // Wrap the mock TyCtxt
    let def_id = DefId;

    let has_derive = mock_tcx_wrapper.has_derive_attr(raw_mock_tcx, def_id, "Debug"); // Pass the raw mock_tcx

    assert_eq!(has_derive, true, "Mock AttributeReader should always return true for has_derive_attr");

    println!("Mock attribute reader test passed!");
}
