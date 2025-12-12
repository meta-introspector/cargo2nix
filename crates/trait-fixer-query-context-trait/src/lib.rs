// crates/trait-fixer-query-context-trait/src/lib.rs

pub trait QueryContext<'tcx, T, I>
where
    T: Sized + 'tcx, // Generic for TyCtxt
    I: Sized + 'tcx, // Generic for Item
{
    fn walk_hir_tops(&self, f: impl FnMut(&'tcx I));
}
