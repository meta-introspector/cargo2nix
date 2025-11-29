use rustc_hir::{Item, ItemKind, Generics, Impl};
use rustc_middle::ty::{Ty, TyCtxt, TypeckResults};
use rustc_span::Span;
use rustc_ast::ast::{Attribute, DeriveInput};
use std::collections::HashSet;

pub struct TraitFixer<'tcx> {
    tcx: TyCtxt<'tcx>,
    config: crate::rules::Config,
    fixes: Vec<Fix>,
}

#[derive(Debug)]
pub enum Fix {
    AddDerive { span: Span, trait_name: String },
    AddCloneImpl { def_id: rustc_hir::def_id::DefId },
    RemoveImpl { item_id: rustc_hir::ItemId },
}

impl<'tcx> TraitFixer<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            config: crate::rules::Config::load(),
            fixes: Vec::new(),
        }
    }

    fn all_fields_impl_trait(&self, ty: Ty<'tcx>, trait_name: &str) -> bool {
        match ty.kind() {
            rustc_middle::ty::Adt(adt, substs) => {
                for field in adt.all_fields() {
                    let field_ty = field.ty(self.tcx, substs);
                    // This part needs the actual trait DefId, which is complex to get.
                    // For now, this is a placeholder. A real implementation would involve
                    // querying the TyCtxt for the trait's DefId.
                    // For simplicity, we'll assume a basic check or skip for now.
                    // if !self.tcx.type_implements_trait(
                    //     self.tcx.get_diagnostic_item(&trait_name.into()).unwrap(),
                    //     field_ty,
                    //     substs,
                    // ) {
                    //     return false;
                    // }
                    // Placeholder: Assume it implements for now for demonstration
                }
                true
            }
            _ => false,
        }
    }

    pub fn check_item(&mut self, item: &'tcx Item<'tcx>) {
        for rule in &self.config.rule {
            match &rule.kind {
                crate::rules::RuleKind::AddDerive => {
                    if let ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..) = item.kind {
                        if rule.apply_to.contains(&format!("{:?}", item.kind).to_lowercase()) {
                            if rule.condition == "always" {
                                let trait_name = rule.trait_name[0].clone();
                                if !has_derive(self.tcx, item.owner_id.to_def_id(), &trait_name) {
                                    self.fixes.push(Fix::AddDerive {
                                        span: item.span,
                                        trait_name,
                                    });
                                }
                            }
                        }
                    }
                }
                crate::rules::RuleKind::AddImpl if rule.trait_name[0] == "Clone" => {
                    if let ItemKind::Struct(_, _) = item.kind {
                        if rule.condition == "all_fields_clone" {
                            let typeck = self.tcx.typeck(item.owner_id.to_def_id());
                            let adt = self.tcx.type_of(item.owner_id.to_def_id());
                            if self.all_fields_impl_trait(adt, "Clone") {
                                // Check if Clone is already implemented (placeholder)
                                // if !self.tcx.has_impl(item.owner_id.to_def_id(), "Clone") {
                                    self.fixes.push(Fix::AddCloneImpl {
                                        def_id: item.owner_id.to_def_id(),
                                    });
                                // }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn has_derive(tcx: TyCtxt, def_id: rustc_hir::def_id::DefId, trait_name: &str) -> bool {
    tcx.get_attrs(def_id, rustc_span::symbol::sym::derive)
        .iter()
        .flat_map(|attr| attr.meta_item_list().into_iter().flatten())
        .any(|item| item.has_name(rustc_span::symbol::Symbol::intern(trait_name)))
}
