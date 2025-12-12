// crates/trait-fixer-core-real/src/lib.rs

use rustc_hir::{self as hir, ItemKind}; // Needed for ItemKind
use rustc_middle::ty::{TyCtxt, GenericArgs};
use rustc_span::Span; // Import for rustc_span::Span
use trait_fixer_core_trait::{CoreFixer, Fix};
use trait_fixer_rules_trait::{ConfigTrait, RuleKind}; // RuleKind and ConfigTrait are from rules_trait
use trait_fixer_attribute_reader_trait::AttributeReader; // To use has_derive_attr from TyCtxt
use trait_fixer_lang_items_trait::LangItems; // To use get_clone_trait_def_id from TyCtxt
use trait_fixer_trait_checker_trait::TraitChecker; // To use type_implements_trait from TyCtxt
use trait_fixer_hir_info_trait::HirInfo; // To use get_item_kind from RealHirInfoItem
use trait_fixer_hir_info_real::RealHirInfoItem; // To wrap Item
use trait_fixer_attribute_reader_real::RustcTyCtxt as RustcAttributeReaderTyCtxt;
use trait_fixer_lang_items_real::RustcTyCtxt as RustcLangItemsTyCtxt;
use trait_fixer_trait_checker_real::RustcTyCtxt as RustcTraitCheckerTyCtxt;
use trait_fixer_query_context_real::TyCtxt as RustcQueryContextTyCtxt; // This one is implemented for TyCtxt directly

pub struct TraitFixer<'tcx, C>
where
    C: ConfigTrait,
{
    pub tcx: TyCtxt<'tcx>,
    pub config: C,
    pub fixes: Vec<Fix<rustc_span::Span, hir::def_id::DefId, hir::ItemId>>,
}

impl<'tcx, C> TraitFixer<'tcx, C>
where
    C: ConfigTrait,
{
    // The check_item logic will be implemented here
}

impl<'tcx, C> CoreFixer<'tcx, C, TyCtxt<'tcx>, hir::Item<'tcx>, hir::def_id::DefId, hir::ItemId, rustc_span::Span> for TraitFixer<'tcx, C>
where
    C: ConfigTrait,
{
    fn new(
        tcx: TyCtxt<'tcx>,
        config: C,
    ) -> Self {
        Self {
            tcx,
            config,
            fixes: Vec::new(),
        }
    }

    fn add_fix(&mut self, fix: Fix<rustc_span::Span, hir::def_id::DefId, hir::ItemId>) {
        self.fixes.push(fix);
    }

    fn get_fixes(&self) -> &Vec<Fix<rustc_span::Span, hir::def_id::DefId, hir::ItemId>> {
        &self.fixes
    }

    fn process_hir(&mut self) {
        RustcQueryContextTyCtxt(self.tcx).walk_hir_tops(|item| {
            self.check_item(item);
        });
    }

    fn check_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        let real_hir_info = RealHirInfoItem(*item); // Wrap the item

        for rule in self.config.get_rules() {
            match &rule.kind {
                RuleKind::AddDerive => {
                    if let ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..) = real_hir_info.get_item_kind() {
                        if rule.apply_to.contains(&format!("{:?}", real_hir_info.get_item_kind()).to_lowercase()) {
                            if rule.condition == "always" {
                                let trait_name = rule.trait_name[0].clone();
                                if !RustcAttributeReaderTyCtxt(self.tcx).has_derive_attr(real_hir_info.get_owner_id().to_def_id(), &trait_name) {
                                    self.fixes.push(Fix::AddDerive {
                                        span: real_hir_info.get_item_span(),
                                        trait_name,
                                    });
                                }
                            }
                        }
                    }
                }
                RuleKind::AddImpl if rule.trait_name[0] == "Clone" => {
                    if let ItemKind::Struct(..) = real_hir_info.get_item_kind() {
                        if rule.condition == "all_fields_clone" {
                            let _typeck = self.tcx.typeck(real_hir_info.get_owner_id());
                            let adt_ty = self.tcx.type_of(real_hir_info.get_owner_id()).instantiate(self.tcx, GenericArgs::identity_for_item(self.tcx, real_hir_info.get_owner_id().to_def_id()));

                            let trait_def_id = RustcLangItemsTyCtxt(self.tcx).get_clone_trait_def_id();

                            if let Some(trait_def_id) = trait_def_id {
                                if RustcTraitCheckerTyCtxt(self.tcx).type_implements_trait(self.tcx, adt_ty, real_hir_info.get_owner_id().to_def_id(), trait_def_id) {
                                    self.fixes.push(Fix::AddCloneImpl {
                                        def_id: real_hir_info.get_owner_id().to_def_id(),
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
