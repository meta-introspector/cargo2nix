// crates/trait-fixer-trait-checker-mock/src/lib.rs

use trait_fixer_rustc_mock::{DefId, MockTy, MockTyCtxt};
use trait_fixer_trait_checker_trait::TraitChecker; // Using MockTy for adt_ty

// Implementation for MockTyCtxt
impl<'tcx> TraitChecker<'tcx> for MockTyCtxt<'tcx> {
    fn get_trait_def_id(&self, _trait_name: &str) -> Option<DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }

    fn type_implements_trait(
        &self,
        _tcx: MockTyCtxt<'tcx>,
        _adt_ty: MockTy<'tcx>,
        _item_def_id: DefId,
        _trait_def_id: DefId,
    ) -> bool {
        // Always return true for mock, indicating it implements the trait
        true
    }
}
