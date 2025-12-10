// crates/trait-fixer-hir-info-mock/src/lib.rs

use std::marker::PhantomData;
use trait_fixer_hir_info_trait::HirInfo;

// Import the real rustc types that the HirInfo trait expects
use rustc_hir::ItemKind;
use rustc_hir_id::OwnerId;
use rustc_span::Span;

// Import the mock Item type to be wrapped
use trait_fixer_rustc_mock::Item; // Keep this for the MockItem struct

// Newtype wrapper to satisfy the orphan rule.
pub struct MockItem<'tcx>(pub Item<'tcx>, PhantomData<&'tcx ()>);

// Implementation for Mock Item directly.
impl<'tcx> HirInfo<'tcx> for MockItem<'tcx> {
    fn get_owner_id(&self) -> rustc_hir_id::OwnerId {
        rustc_hir_id::OwnerId::DUMMY
    }

    fn get_item_kind<'a>(&'a self) -> &'a rustc_hir::hir::ItemKind<'tcx> {
        &rustc_hir::hir::ItemKind::Struct(PhantomData)
    }

    fn get_item_span(&self) -> rustc_span::span_encoding::Span {
        rustc_span::span_encoding::Span::DUMMY
    }
}
