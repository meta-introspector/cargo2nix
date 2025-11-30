// isolated_mock_test_env/crates/trait-fixer-lang-items-mock/src/lib.rs

use trait_fixer_lang_items_trait::LangItems;
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym, Symbol}; // All from mock crate

// Newtype wrapper to implement external trait for external type
pub struct MockLangItemsTyCtxt<'tcx>(pub TyCtxt<'tcx>);

impl<'tcx> LangItems<'tcx> for MockLangItemsTyCtxt<'tcx> {
    type DefId = DefId;
    type Symbol = Symbol;

    fn get_clone_trait_def_id(&'tcx self) -> Option<Self::DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }

    fn get_debug_trait_def_id(&'tcx self, _sym_debug: Self::Symbol) -> Option<Self::DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }
}
