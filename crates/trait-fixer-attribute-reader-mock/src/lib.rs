// crates/trait-fixer-attribute-reader-mock/src/lib.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym, Symbol}; // Added Symbol

// Newtype wrapper to implement external trait for external type
pub struct MockAttributeReaderTyCtxt<'tcx>(pub TyCtxt<'tcx>);

// Implementation for the newtype wrapper
impl<'tcx> AttributeReader<'tcx> for MockAttributeReaderTyCtxt<'tcx> {
    fn has_derive_attr(&self, def_id: DefId, trait_name: &str) -> bool {
        self.0.get_attrs(def_id, sym::derive) // Use self.0 to access the inner TyCtxt
            .into_iter()
            .flat_map(|attr| attr.meta_item_list().into_iter())
            .any(|item| item.has_name(Symbol::intern(trait_name)))
    }
}