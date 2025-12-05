use rustc_hir::{Item, ItemKind};
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_span::Span;
use rustc_hir::def_id::DefId;

use trait_fixer_core::{Fix, TraitFixer};
use trait_fixer_rules as rules;
use trait_fixer_hir_info::HirInfo;
use trait_fixer_lang_items::LangItems;
use trait_fixer_attribute_reader::AttributeReader;
use trait_fixer_trait_checker::TraitChecker;


// The TraitFixer struct is now in trait_fixer_core, so we don't define it here.
// The Fix enum is also in trait_fixer_core.

impl<'tcx> TraitFixer<'tcx> {
    // all_fields_impl_trait will be moved to TraitChecker trait implementation
    // has_derive will be moved to AttributeReader trait implementation

    pub fn check_item(&mut self, item: &'tcx Item<'tcx>) {
        for rule in &self.config.rule {
            match &rule.kind {
                rules::RuleKind::AddDerive => {
                    if let ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..) = item.get_item_kind(&item) {
                        if rule.apply_to.contains(&format!("{:?}", item.get_item_kind(&item)).to_lowercase()) {
                            if rule.condition == "always" {
                                let trait_name = rule.trait_name[0].clone();
                                if !self.tcx.has_derive_attr(self.tcx, item.get_owner_id(&item).to_def_id(), &trait_name) {
                                    self.fixes.push(Fix::AddDerive {
                                        span: item.get_item_span(&item),
                                        trait_name,
                                    });
                                }
                            }
                        }
                    }
                }
                rules::RuleKind::AddImpl if rule.trait_name[0] == "Clone" => {
                    if let ItemKind::Struct(..) = item.get_item_kind(&item) {
                        if rule.condition == "all_fields_clone" {
                            // typeck is needed to get the type of the item
                            let _typeck = self.tcx.typeck(item.get_owner_id(&item));
                            let adt_ty = self.tcx.type_of(item.get_owner_id(&item)).instantiate(self.tcx, ty::subst::Substs::empty()); // Assuming ty::subst::Substs::empty() is correct

                            let trait_def_id = self.tcx.get_clone_trait_def_id();

                            if let Some(trait_def_id) = trait_def_id {
                                if self.tcx.type_implements_trait(self.tcx, adt_ty, item.get_owner_id(&item).to_def_id(), trait_def_id) {
                                    // Check if Clone is already implemented (placeholder)
                                    // if !self.tcx.has_impl(item.owner_id.to_def_id(), "Clone") {
                                        self.fixes.push(Fix::AddCloneImpl {
                                            def_id: item.get_owner_id(&item).to_def_id(),
                                        });
                                    // }
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