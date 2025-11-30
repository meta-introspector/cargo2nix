// crates/trait-fixer-attribute-reader-mock/src/lib.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym}; // Corrected: use TyCtxt directly

// Implementation for MockTyCtxt (now aliased as TyCtxt)
impl<'tcx> AttributeReader<'tcx> for TyCtxt<'tcx> {
    fn has_derive_attr(&self, tcx: TyCtxt<'tcx>, def_id: DefId, trait_name: &str) -> bool {
        tcx.get_attrs(def_id, sym::derive)
            .into_iter()
            .flat_map(|attr| attr.meta_item_list().into_iter())
            .any(|item| item.has_name(Symbol::intern(trait_name)))
    }
}