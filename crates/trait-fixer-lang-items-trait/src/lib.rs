// crates/trait-fixer-lang-items-trait/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;

pub trait LangItems<'tcx> {
    fn get_clone_trait_def_id(&self) -> Option<DefId>;
    fn get_debug_trait_def_id(&self) -> Option<DefId>;
}
