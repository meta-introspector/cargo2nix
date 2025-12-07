// crates/trait-fixer-core-trait/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_hir::ItemId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

// Import the helper traits (but they will be implemented by TyCtxt)
use trait_fixer_rules_trait::ConfigTrait;


#[derive(Debug)]
pub enum Fix {
    AddDerive { span: Span, trait_name: String },
    AddCloneImpl { def_id: DefId },
    RemoveImpl { item_id: ItemId },
}

// CoreFixer now only needs ConfigTrait explicitly, others are via TyCtxt
pub trait CoreFixer<'tcx, C>
where
    C: ConfigTrait,
{
    fn new(
        tcx: TyCtxt<'tcx>,
        config: C,
    ) -> Self;
    fn add_fix(&mut self, fix: Fix);
    fn get_fixes(&self) -> &Vec<Fix>;
    fn process_hir(&mut self);
    fn check_item(&mut self, item: &'tcx rustc_hir::Item<'tcx>); // Add check_item to trait
}
