// isolated_mock_test_env/crates/trait-fixer-hir-info-real/src/lib.rs

use rustc_hir::Item;
use rustc_hir::OwnerId;
use rustc_hir::ItemKind;
use rustc_span::Span;
use trait_fixer_hir_info_trait::HirInfo;

impl<'tcx> HirInfo<'tcx> for Item<'tcx> {
    type Item = Item<'tcx>;
    type OwnerId = OwnerId;
    type ItemKind = ItemKind<'tcx>;
    type Span = Span;

    fn get_owner_id(&'tcx self) -> Self::OwnerId {
        self.owner_id
    }

    fn get_item_kind(&'tcx self) -> &'tcx Self::ItemKind {
        &self.kind
    }

    fn get_item_span(&'tcx self) -> Self::Span {
        self.span
    }
}
