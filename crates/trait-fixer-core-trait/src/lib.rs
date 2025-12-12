// crates/trait-fixer-core-trait/src/lib.rs

use trait_fixer_rules_trait::ConfigTrait;

#[derive(Debug)]
pub enum Fix<S, D, ID> {
    AddDerive { span: S, trait_name: String },
    AddCloneImpl { def_id: D },
    RemoveImpl { item_id: ID },
}

// CoreFixer now only needs ConfigTrait explicitly, others are via TyCtxt
pub trait CoreFixer<'tcx, C, T, I, D, ID, S>
where
    C: ConfigTrait,
    T: Sized + 'tcx, // Generic for TyCtxt
    I: Sized + 'tcx, // Generic for Item
    D: Sized + 'tcx, // Generic for DefId
    ID: Sized + 'tcx, // Generic for ItemId
    S: Sized + 'tcx + Copy + Debug, // Generic for Span, adding Copy and Debug for consistency
{
    fn new(
        tcx: T,
        config: C,
    ) -> Self;
    fn add_fix(&mut self, fix: Fix<S, D, ID>);
    fn get_fixes(&self) -> &Vec<Fix<S, D, ID>>;
    fn process_hir(&mut self);
    fn check_item(&mut self, item: &'tcx I); // Use generic Item type 'I'
}

use std::fmt::Debug; // Required for Debug trait bound
