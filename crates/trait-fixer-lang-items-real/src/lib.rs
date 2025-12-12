// crates/trait-fixer-lang-items-real/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::symbol::sym;

use trait_fixer_lang_items_trait::LangItems; // Import the trait

pub struct RustcTyCtxt<'tcx>(pub TyCtxt<'tcx>);

// Implementation for RustcTyCtxt
impl<'tcx> LangItems<'tcx> for RustcTyCtxt<'tcx> {
    fn get_clone_trait_def_id(&self) -> Option<DefId> {
        self.0.lang_items().clone_trait()
    }

    fn get_debug_trait_def_id(&self) -> Option<DefId> {
        self.0.get_diagnostic_item(sym::Debug)
    }
}
