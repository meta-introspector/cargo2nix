// crates/trait-fixer-attribute-reader-mock/src/lib.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym, Symbol}; // All from mock crate

// Newtype wrapper to implement external trait for external type
pub struct MockAttributeReaderTyCtxt<'tcx>(pub TyCtxt<'tcx>);

// Implementation for the newtype wrapper
impl<'tcx> AttributeReader<'tcx> for MockAttributeReaderTyCtxt<'tcx> {
    type DefId = DefId;
    type Symbol = Symbol;

    fn has_derive_attr(&'tcx self, def_id: Self::DefId, trait_name: &str) -> bool {
        self.0.get_attrs(def_id, Self::sym_derive())
            .into_iter()
            .flat_map(|attr| attr.meta_item_list().into_iter())
            .any(|item| item.has_name(Self::sym_intern(trait_name)))
    }

    fn sym_derive() -> Self::Symbol {
        sym::derive
    }

    fn sym_intern(s: &str) -> Self::Symbol {
        Symbol::intern(s)
    }
}
