// isolated_mock_test_env/crates/trait-fixer-trait-checker-mock/src/lib.rs

use trait_fixer_trait_checker_trait::TraitChecker;
use trait_fixer_rustc_mock::{TyCtxt, DefId, Ty, Symbol}; // All from mock crate

impl<'tcx> TraitChecker<'tcx> for TyCtxt<'tcx> {
    type Ty = Ty<'tcx>;
    type DefId = DefId;
    type Symbol = Symbol;

    fn get_trait_def_id(&'tcx self, _trait_name: &str, _sym_intern: impl FnOnce(&str) -> Self::Symbol) -> Option<Self::DefId> {
        Some(DefId) // Always return a dummy DefId for mock
    }

    fn type_implements_trait(
        &'tcx self,
        _adt_ty: Self::Ty,
        _item_def_id: Self::DefId,
        _trait_def_id: Self::DefId,
    ) -> bool {
        // Always return true for mock, indicating it implements the trait
        true
    }
}