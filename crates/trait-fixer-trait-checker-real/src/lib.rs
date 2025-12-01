// crates/trait-fixer-trait-checker-real/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_infer::infer::{InferCtxt, TyCtxtInferExt};
use rustc_middle::ty::subst;
use rustc_middle::ty::Binder;
use rustc_middle::ty::{ParamEnv, Predicate, Ty, TyCtxt, TypingMode};
use rustc_span::symbol::Symbol;
use rustc_span::DUMMY_SP;
use rustc_trait_selection::traits::{
    ObligationCause, ObligationCauseCode, PredicateObligation, TraitEngine,
}; // For subst::Substs::empty()

use trait_fixer_trait_checker_trait::TraitChecker; // Import the trait

// Implementation for TyCtxt
impl<'tcx> TraitChecker<'tcx> for TyCtxt<'tcx> {
    fn get_trait_def_id(&self, trait_name: &str) -> Option<DefId> {
        match trait_name {
            "Clone" => self.lang_items().clone_trait(),
            "Debug" => self.get_diagnostic_item(Symbol::intern("Debug")),
            _ => {
                let trait_sym = Symbol::intern(trait_name);
                self.get_diagnostic_item(trait_sym)
            }
        }
    }

    fn type_implements_trait(
        &self,
        tcx: TyCtxt<'tcx>,
        adt_ty: Ty<'tcx>,
        item_def_id: DefId,
        trait_def_id: DefId,
    ) -> bool {
        let param_env = tcx.param_env(item_def_id);

        let obligation = PredicateObligation::new(
            tcx,
            ObligationCause::new(
                DUMMY_SP,
                item_def_id.as_local().unwrap(),
                ObligationCauseCode::Misc,
            ),
            param_env,
            Binder::dummy(tcx.mk_trait_ref(
                trait_def_id,
                tcx.mk_args_trait(adt_ty, subst::Substs::empty()),
            )),
        );

        let infcx = tcx.infer_ctxt().build(TypingMode::default());
        infcx.probe(|_infcx| {
            _infcx
                .at(&obligation.cause, obligation.param_env)
                .predicate_may_hold(&obligation.predicate)
                .is_ok()
        })
    }
}
