// isolated_mock_test_env/crates/trait-fixer-query-context-mock/src/lib.rs

use trait_fixer_query_context_trait::QueryContext;
use trait_fixer_rustc_mock::{TyCtxt, Item}; // All from mock crate

// Newtype wrapper to implement external trait for external type
pub struct MockQueryContextTyCtxt<'tcx>(pub TyCtxt<'tcx>);

impl<'tcx> QueryContext<'tcx> for MockQueryContextTyCtxt<'tcx> {
    type Item = Item<'tcx>;

    fn walk_hir_tops(&'tcx self, _f: impl FnMut(&'tcx Self::Item)) {
        println!("Mock MockTyCtxt::walk_hir_tops called");
    }
}
