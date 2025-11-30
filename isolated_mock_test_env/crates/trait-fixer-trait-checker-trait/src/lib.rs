// crates/trait-fixer-trait-checker-trait/src/lib.rs

// No rustc imports here.
pub trait TraitChecker<'tcx> {
    type Ty: 'tcx;
    type DefId;
    type Symbol;

    // The implementor of this trait is the context (e.g., TyCtxt or MockTyCtxt)
    fn get_trait_def_id(&'tcx self, trait_name: &str, sym_intern: impl FnOnce(&str) -> Self::Symbol) -> Option<Self::DefId>;
    fn type_implements_trait(
        &'tcx self, // self is the TyCtxt here
        adt_ty: Self::Ty,
        item_def_id: Self::DefId,
        trait_def_id: Self::DefId,
    ) -> bool;
}