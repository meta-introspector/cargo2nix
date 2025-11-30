// isolated_mock_test_env/crates/trait-fixer-trait-checker-real/src/lib.rs

use rustc_middle::ty::{Ty, TyCtxt, ParamEnv, Predicate, TypingMode};
use rustc_hir::def_id::DefId;
use rustc_span::DUMMY_SP;
use rustc_span::symbol::Symbol;
use rustc_infer::infer::{TyCtxtInferExt, InferCtxt};
use rustc_trait_selection::traits::{
    ObligationCause, ObligationCauseCode, PredicateObligation, TraitEngine,
};
use rustc_middle::ty::Binder;
use rustc_middle::ty::subst; // For subst::Substs::empty()
use trait_fixer_trait_checker_trait::TraitChecker;

impl<'tcx> TraitChecker<'tcx> for TyCtxt<'tcx> {
    type Ty = Ty<'tcx>;
    type DefId = DefId;
    type Symbol = Symbol;

    fn get_trait_def_id(&'tcx self, trait_name: &str, sym_intern: impl FnOnce(&str) -> Self::Symbol) -> Option<Self::DefId> {
        match trait_name {
            "Clone" => self.lang_items().clone_trait(),
            "Debug" => self.get_diagnostic_item(sym_intern("Debug")), // Use the provided sym_intern
            _ => {
                let trait_sym = sym_intern(trait_name); // Use the provided sym_intern
                self.get_diagnostic_item(trait_sym)
            }
        }
    }

    fn type_implements_trait(
        &'tcx self,
        adt_ty: Self::Ty,
        item_def_id: Self::DefId,
        trait_def_id: Self::DefId,
    ) -> bool {
        let param_env = self.param_env(item_def_id);

        let obligation = PredicateObligation::new(
            self,
            ObligationCause::new(
                DUMMY_SP,
                item_def_id.as_local().unwrap(),
                ObligationCauseCode::Misc,
            ),
            param_env,
            Binder::dummy(
                self.mk_trait_ref(trait_def_id, self.mk_args_trait(adt_ty, subst::Substs::empty()))
            ),
        );

        let infcx = self.infer_ctxt().build(TypingMode::default());
        infcx.probe(|_infcx| {
            _infcx.at(&obligation.cause, obligation.param_env).predicate_may_hold(&obligation.predicate).is_ok()
        })
    }
}
