// crates/trait-fixer-attribute-reader-trait/src/lib.rs

// This crate defines the generic interface for reading attributes, independent of rustc internals.
pub trait AttributeReader<'tcx> {
    type DefId: 'tcx;
    type Symbol;

    // The implementor of this trait will be a context type (e.g., a wrapper around TyCtxt).
    // The 'self' here refers to that context type.
    fn has_derive_attr(&'tcx self, def_id: Self::DefId, trait_name: &str) -> bool;

    // We also need a way to get the "derive" symbol and intern other symbols.
    // This implies that the implementor of this trait also needs to provide symbol manipulation.
    fn sym_derive() -> Self::Symbol;
    fn sym_intern(s: &str) -> Self::Symbol;
}
