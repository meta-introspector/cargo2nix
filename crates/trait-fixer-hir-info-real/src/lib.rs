// crates/trait-fixer-hir-info-real/src/lib.rs

use rustc_hir::Item;
use rustc_hir::OwnerId;
use rustc_hir::ItemKind;
use rustc_span::Span;
// use std::marker::PhantomData; // This is not needed for the real rustc, but it helps when mocking. // Remove this line if not used

use trait_fixer_hir_info_trait::HirInfo; // Import the trait

// Implementation for Item directly.
impl<'tcx> HirInfo<'tcx> for Item<'tcx> {
    fn get_owner_id(&self, _item: &'tcx Item<'tcx>) -> OwnerId {
        self.owner_id
    }

    fn get_item_kind(&self, _item: &'tcx Item<'tcx>) -> &'tcx ItemKind<'_> {
        &self.kind
    }

    fn get_item_span(&self, _item: &'tcx Item<'tcx>) -> Span {
        self.span
    }
}