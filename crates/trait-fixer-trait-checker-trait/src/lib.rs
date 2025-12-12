// crates/trait-fixer-trait-checker-trait/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

pub trait TraitChecker<'tcx> {
    fn get_trait_def_id(&self, trait_name: &str) -> Option<DefId>;
    fn type_implements_trait(
        &self,
        tcx: TyCtxt<'tcx>,
        adt_ty: Ty<'tcx>,
        item_def_id: DefId,
        trait_def_id: DefId,
    ) -> bool;
}
