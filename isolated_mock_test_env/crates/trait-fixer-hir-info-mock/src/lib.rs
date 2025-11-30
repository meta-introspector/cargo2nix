// isolated_mock_test_env/crates/trait-fixer-hir-info-mock/src/lib.rs

use trait_fixer_hir_info_trait::HirInfo;
use trait_fixer_rustc_mock::{Item, OwnerId, ItemKind, Span}; // All from mock crate
use std::marker::PhantomData; // Needed for ItemKind and Item in mock

impl<'tcx> HirInfo<'tcx> for Item<'tcx> {
    type Item = Item<'tcx>;
    type OwnerId = OwnerId;
    type ItemKind = ItemKind<'tcx>;
    type Span = Span;

    fn get_owner_id(&'tcx self) -> Self::OwnerId {
        OwnerId
    }

    fn get_item_kind(&'tcx self) -> &'tcx Self::ItemKind {
        &ItemKind::Struct(PhantomData)
    }

    fn get_item_span(&'tcx self) -> Self::Span {
        Span
    }
}