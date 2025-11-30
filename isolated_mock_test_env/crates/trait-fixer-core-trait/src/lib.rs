// crates/trait-fixer-core-trait/src/lib.rs

// No rustc imports here.
// All types that were rustc-specific should be generic parameters or associated types.

// Fix enum should also be generic over its types
#[derive(Debug)]
pub enum Fix<S, D, I> { // S for Span, D for DefId, I for ItemId
    AddDerive { span: S, trait_name: String },
    AddCloneImpl { def_id: D },
    RemoveImpl { item_id: I },
}

pub trait CoreFixer<'tcx, Tcx, S, D, I> // Tcx for TyCtxt, S for Span, D for DefId, I for ItemId
where
    Tcx: 'tcx,
    S: 'tcx,
    D: 'tcx,
    I: 'tcx,
{
    fn new(tcx: Tcx) -> Self;
    fn add_fix(&mut self, fix: Fix<S, D, I>);
    fn get_fixes(&self) -> &Vec<Fix<S, D, I>>;
    fn process_hir(&mut self);
}
