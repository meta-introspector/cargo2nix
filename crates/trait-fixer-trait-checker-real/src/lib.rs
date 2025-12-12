// crates/trait-fixer-trait-checker-real/src/lib.rs

use rustc_hir::def_id::DefId;
use rustc_span::def_id::DefIndex;
use rustc_infer::infer::{InferCtxt, TyCtxtInferExt};
use rustc_middle::ty::{Binder, ParamEnv, Predicate, Ty, TyCtxt, TypingMode};
use rustc_middle::ty::ClauseKind; // Import ClauseKind
use rustc_middle::ty::PredicateKind; // Import PredicateKind
use rustc_span::symbol::Symbol;
use rustc_span::DUMMY_SP;
use rustc_trait_selection::traits::{
    Obligation, ObligationCause, ObligationCauseCode, PredicateObligation, TraitEngine,
}; // For Substs::empty()
use rustc_span::def_id::{CRATE_DEF_INDEX, LOCAL_CRATE};
use std::default::Default;

use trait_fixer_trait_checker_trait::TraitChecker; // Import the trait

pub struct RustcTyCtxt<'tcx>(pub TyCtxt<'tcx>);

// Implementation for RustcTyCtxt
impl<'tcx> TraitChecker<'tcx> for RustcTyCtxt<'tcx> {
    fn get_trait_def_id(&self, trait_name: &str) -> Option<DefId> {
        match trait_name {
            "Clone" => self.0.lang_items().clone_trait(),
            "Debug" => self.0.get_diagnostic_item(Symbol::intern("Debug")),
            _ => {
                let trait_sym = Symbol::intern(trait_name);
                self.0.get_diagnostic_item(trait_sym)
            }
        }
    }

    fn type_implements_trait(
        &self,
        ty: Ty<'tcx>,
        trait_def_id: DefId,
    ) -> bool {
        let tcx = self.0;
        let infcx = tcx.infer_ctxt().build(TypingMode::Analysis { defining_opaque_types_and_generators: Default::default() });
        let param_env = ParamEnv { caller_bounds: tcx.mk_clauses(&[]), reveal_all: true };
        let predicates = [tcx.mk_predicate(Binder::dummy(PredicateKind::Clause(ClauseKind::Trait(PredicateObligation {
            cause: ObligationCause::new(DUMMY_SP, DefId::local(DefIndex::from_usize(0)).expect_local(), ObligationCauseCode::Pattern),
            param_env,
            predicate: Binder::dummy(tcx.mk_trait_predicate(trait_def_id, tcx.mk_args(&[ty.into()]))),
        })];

        let mut fulfill_cx = TraitEngine::new(tcx);
        for predicate in predicates {
            fulfill_cx.register_predicate_obligation(infcx, predicate);
        }

        let errors = fulfill_cx.select_all_and_apply_where_possible(&infcx, TypingMode::Canonical);
        errors.is_empty()
    }
}

