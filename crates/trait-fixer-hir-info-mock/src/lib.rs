// crates/trait-fixer-hir-info-mock/src/lib.rs

use trait_fixer_hir_info_trait::HirInfo;
use trait_fixer_rustc_mock::{Item, OwnerId, ItemKind, Span};
use std::marker::PhantomData; // Needed for ItemKind and Item in mock

// Implementation for Mock Item directly.
impl<'tcx> HirInfo<'tcx> for Item<'tcx> {
    fn get_owner_id(&self, _item: &'tcx Item<'tcx>) -> OwnerId {
        OwnerId
    }

    fn get_item_kind(&self, _item: &'tcx Item<'tcx>) -> &'tcx ItemKind<'tcx> {
        &ItemKind::Struct(PhantomData)
    }

    fn get_item_span(&self, _item: &'tcx Item<'tcx>) -> Span {
        Span
    }
}
