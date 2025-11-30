// isolated_mock_test_env/crates/trait-fixer-lang-items-real/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use rustc_span::symbol::sym;
use rustc_span::symbol::Symbol; // For Symbol::intern
use trait_fixer_lang_items_trait::LangItems;

impl<'tcx> LangItems<'tcx> for TyCtxt<'tcx> {
    type DefId = DefId;
    type Symbol = Symbol;

    fn get_clone_trait_def_id(&'tcx self) -> Option<Self::DefId> {
        self.lang_items().clone_trait()
    }

    fn get_debug_trait_def_id(&'tcx self, sym_debug: Self::Symbol) -> Option<Self::DefId> {
        self.get_diagnostic_item(sym_debug)
    }
}
