// crates/trait-fixer-core-real/src/lib.rs

use rustc_hir::Item;
use rustc_hir::ItemKind; // Needed for ItemKind
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::subst; // Needed for subst::Substs::empty()
use trait_fixer_core_trait::{CoreFixer, Fix};
use trait_fixer_rules_trait::{ConfigTrait, RuleKind}; // Import RuleKind
use trait_fixer_attribute_reader_trait::AttributeReader; // To use has_derive_attr from TyCtxt
use trait_fixer_lang_items_trait::LangItems; // To use get_clone_trait_def_id from TyCtxt
use trait_fixer_trait_checker_trait::TraitChecker; // To use type_implements_trait from TyCtxt
use trait_fixer_hir_info_trait::HirInfo; // To use get_item_kind from RealHirInfoItem
use trait_fixer_hir_info_real::RealHirInfoItem; // To wrap Item

pub struct TraitFixer<'tcx, C>
where
    C: ConfigTrait,
{
    pub tcx: TyCtxt<'tcx>,
    pub config: C,
    pub fixes: Vec<Fix>,
}

impl<'tcx, C> TraitFixer<'tcx, C>
where
    C: ConfigTrait,
{
    // The check_item logic will be implemented here
}

impl<'tcx, C> CoreFixer<'tcx, C> for TraitFixer<'tcx, C>
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

    fn add_fix(&mut self, fix: Fix) {
        self.fixes.push(fix);
    }

    fn get_fixes(&self) -> &Vec<Fix> {
        &self.fixes
    }

    fn process_hir(&mut self) {
        self.tcx.hir().walk_tops(|item| {
            self.check_item(item);
        });
    }

    fn check_item(&mut self, item: &'tcx Item<'tcx>) {
        let real_hir_info = RealHirInfoItem(*item); // Wrap the item

        for rule in self.config.get_rules() {
            match &rule.kind {
                RuleKind::AddDerive => {
                    if let ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..) = real_hir_info.get_item_kind() {
                        if rule.apply_to.contains(&format!("{:?}", real_hir_info.get_item_kind()).to_lowercase()) {
                            if rule.condition == "always" {
                                let trait_name = rule.trait_name[0].clone();
                                if !self.tcx.has_derive_attr(real_hir_info.get_owner_id().to_def_id(), &trait_name) {
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
                            let adt_ty = self.tcx.type_of(real_hir_info.get_owner_id()).instantiate(self.tcx, subst::Substs::empty());

                            let trait_def_id = self.tcx.get_clone_trait_def_id();

                            if let Some(trait_def_id) = trait_def_id {
                                if self.tcx.type_implements_trait(self.tcx, adt_ty, real_hir_info.get_owner_id().to_def_id(), trait_def_id) {
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
