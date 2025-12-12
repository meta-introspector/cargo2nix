// crates/trait-fixer-trait-checker-mock/src/lib.rs

use trait_fixer_rustc_mock::{DefId, MockTy, TyCtxt};
use trait_fixer_trait_checker_trait::TraitChecker;

pub struct MockTyCtxtWrapper<'tcx>(pub TyCtxt<'tcx>);

// Implementation for MockTyCtxtWrapper
impl<'tcx> TraitChecker<'tcx, TyCtxt<'tcx>, DefId, MockTy<'tcx>> for MockTyCtxtWrapper<'tcx> {
    fn get_trait_def_id(&self, _trait_name: &str) -> Option<DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }

    fn type_implements_trait(
        &self,
        _tcx: TyCtxt<'tcx>, // Changed to _tcx because it's unused, and uses mock TyCtxt
        _adt_ty: MockTy<'tcx>,
        _item_def_id: DefId,
        _trait_def_id: DefId,
    ) -> bool {
        // Always return true for mock, indicating it implements the trait
        true
    }
}
