// crates/trait-fixer-query-context-trait/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::Item;

pub trait QueryContext<'tcx> {
    fn walk_hir_tops(&self, f: impl FnMut(&'tcx Item<'tcx>));
}
