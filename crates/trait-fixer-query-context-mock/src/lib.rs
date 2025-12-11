// crates/trait-fixer-query-context-mock/src/lib.rs

use trait_fixer_query_context_trait::QueryContext;
use trait_fixer_rustc_mock::{Item, TyCtxt};

// Implementation for MockTyCtxt
impl<'tcx> QueryContext<'tcx> for TyCtxt<'tcx> {
    fn walk_hir_tops(&self, _f: impl FnMut(&'tcx Item<'tcx>)) {
        println!("Mock MockTyCtxt::walk_hir_tops called");
    }
}
