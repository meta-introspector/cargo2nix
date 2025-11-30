// crates/trait-fixer-lang-items-trait/src/lib.rs

// No rustc imports here.
pub trait LangItems<'tcx> {
    type DefId;
    type Symbol;

    // The implementor of this trait is the context (e.g., TyCtxt or MockTyCtxt)
    fn get_clone_trait_def_id(&'tcx self) -> Option<Self::DefId>;
    fn get_debug_trait_def_id(&'tcx self, sym_debug: Self::Symbol) -> Option<Self::DefId>;
}