// crates/trait-fixer-attribute-reader-real/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use rustc_span::symbol::Symbol;
use rustc_span::symbol::sym;
use trait_fixer_attribute_reader_trait::AttributeReader;

impl<'tcx> AttributeReader<'tcx> for TyCtxt<'tcx> {
    type DefId = DefId;
    type Symbol = Symbol;

    fn has_derive_attr(&'tcx self, def_id: Self::DefId, trait_name: &str) -> bool {
        self.get_attrs(def_id, Self::sym_derive())
            .flat_map(|attr| attr.meta_item_list().into_iter().flatten())
            .any(|item| item.has_name(Self::sym_intern(trait_name)))
    }

    fn sym_derive() -> Self::Symbol {
        sym::derive
    }

    fn sym_intern(s: &str) -> Self::Symbol {
        Symbol::intern(s)
    }
}