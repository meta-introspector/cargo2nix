// crates/trait-fixer-query-context-real/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::{self as rustc_hir_actual, Item}; // Alias rustc_hir to avoid collision
use rustc_hir::intravisit::Visitor; // Explicitly import Visitor for walk_tops

use trait_fixer_query_context_trait::QueryContext; // Import the trait

// Implementation for TyCtxt
impl<'tcx> QueryContext<'tcx> for TyCtxt<'tcx> {
    fn walk_hir_tops(&self, mut f: impl FnMut(&'tcx Item<'tcx>)) {
        self.hir().walk_tops(|item| f(item));
    }
}