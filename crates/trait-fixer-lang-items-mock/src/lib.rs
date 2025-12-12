// crates/trait-fixer-lang-items-mock/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_span::def_id::CRATE_DEF_INDEX;
use trait_fixer_lang_items_trait::LangItems;
use trait_fixer_rustc_mock::{sym, TyCtxt};

pub struct MockTyCtxt<'tcx>(pub TyCtxt<'tcx>);

// Implementation for MockTyCtxt
impl<'tcx> LangItems<'tcx> for MockTyCtxt<'tcx> {
    fn get_clone_trait_def_id(&self) -> Option<DefId> {
        Some(DefId::local(CRATE_DEF_INDEX)) // Always return a dummy DefId for mock
    }

    fn get_debug_trait_def_id(&self) -> Option<DefId> {
        Some(DefId::local(CRATE_DEF_INDEX)) // Always return a dummy DefId for mock
    }
}
