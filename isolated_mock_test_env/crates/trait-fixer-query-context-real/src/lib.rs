// isolated_mock_test_env/crates/trait-fixer-query-context-real/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::Item;
use trait_fixer_query_context_trait::QueryContext;

impl<'tcx> QueryContext<'tcx> for TyCtxt<'tcx> {
    type Item = Item<'tcx>;

    fn walk_hir_tops(&'tcx self, mut f: impl FnMut(&'tcx Self::Item)) {
        self.hir().walk_tops(|item| f(item));
    }
}
