// crates/trait-fixer-query-context-trait/src/lib.rs

// No rustc imports here.
pub trait QueryContext<'tcx> {
    type Item: 'tcx;

    // The implementor is the context itself.
    fn walk_hir_tops(&'tcx self, f: impl FnMut(&'tcx Self::Item));
}