// crates/trait-fixer-core-trait/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_hir::ItemId;
use rustc_span::Span;
use rustc_middle::ty::TyCtxt;

#[derive(Debug)]
pub enum Fix {
    AddDerive { span: Span, trait_name: String },
    AddCloneImpl { def_id: DefId },
    RemoveImpl { item_id: ItemId },
}

pub trait CoreFixer<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self;
    fn add_fix(&mut self, fix: Fix);
    fn get_fixes(&self) -> &Vec<Fix>;
    fn process_hir(&mut self); // Add this method
}