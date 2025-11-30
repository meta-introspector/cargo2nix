// crates/trait-fixer-attribute-reader-real/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use rustc_span::symbol::Symbol;
use rustc_span::symbol::sym; // For sym::derive
use trait_fixer_attribute_reader_trait::AttributeReader; // Import the trait

// Implementation for TyCtxt
impl<'tcx> AttributeReader<'tcx> for TyCtxt<'tcx> {
    fn has_derive_attr(&self, def_id: DefId, trait_name: &str) -> bool {
        self.get_attrs(def_id, sym::derive) // Use self directly
            .flat_map(|attr| attr.meta_item_list().into_iter().flatten())
            .any(|item| item.has_name(Symbol::intern(trait_name)))
    }
}
