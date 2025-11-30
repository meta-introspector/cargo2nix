// crates/trait-fixer-lang-items-mock/src/lib.rs

use trait_fixer_lang_items_trait::LangItems;
use trait_fixer_rustc_mock::{MockTyCtxt, DefId, sym};

// Implementation for MockTyCtxt
impl<'tcx> LangItems<'tcx> for MockTyCtxt<'tcx> {
    fn get_clone_trait_def_id(&self) -> Option<DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }

    fn get_debug_trait_def_id(&self) -> Option<DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }
}
