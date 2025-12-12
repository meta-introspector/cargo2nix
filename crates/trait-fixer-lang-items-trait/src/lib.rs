// crates/trait-fixer-lang-items-trait/src/lib.rs

pub trait LangItems<'tcx, T, D>
where
    T: Sized + 'tcx, // Generic for TyCtxt
    D: Sized + 'tcx, // Generic for DefId
{
    fn get_clone_trait_def_id(&self) -> Option<D>;
    fn get_debug_trait_def_id(&self) -> Option<D>;
}
