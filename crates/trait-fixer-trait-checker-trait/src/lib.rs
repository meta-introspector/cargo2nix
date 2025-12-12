// crates/trait-fixer-trait-checker-trait/src/lib.rs

pub trait TraitChecker<'tcx, T, D, Y>
where
    T: Sized + 'tcx, // Generic for TyCtxt
    D: Sized + 'tcx, // Generic for DefId
    Y: Sized + 'tcx, // Generic for Ty
{
    fn get_trait_def_id(&self, trait_name: &str) -> Option<D>;
    fn type_implements_trait(
        &self,
        tcx: T,
        adt_ty: Y,
        item_def_id: D,
        trait_def_id: D,
    ) -> bool;
}
