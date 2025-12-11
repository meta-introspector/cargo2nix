// crates/trait-fixer-hir-info-mock/src/lib.rs

use std::marker::PhantomData;
use trait_fixer_hir_info_trait::HirInfo;

// Import the real rustc types that the HirInfo trait expects
use trait_fixer_rustc_mock::{DUMMY_SP, ItemKind, OwnerId, Span};

// Import the mock Item type to be wrapped
use trait_fixer_rustc_mock::Item; // Keep this for the MockItem struct

// Newtype wrapper to satisfy the orphan rule.
pub struct MockItem<'tcx>(pub Item<'tcx>, PhantomData<&'tcx ()>);

// Implementation for Mock Item directly.
impl<'tcx> HirInfo<'tcx> for MockItem<'tcx> {
    type OwnerId = OwnerId;
    type ItemKind = ItemKind<'tcx>;
    type Span = Span;

    fn get_owner_id(&self) -> Self::OwnerId {
        OwnerId::DUMMY
    }

    fn get_item_kind<'a>(&'a self) -> &'a Self::ItemKind {
        &ItemKind::Struct(PhantomData)
    }

    fn get_item_span(&self) -> Self::Span {
        DUMMY_SP
    }
}
