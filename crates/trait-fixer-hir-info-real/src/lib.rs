// crates/trait-fixer-hir-info-real/src/lib.rs

use rustc_hir::Item;
use rustc_hir::ItemKind;
use rustc_hir::OwnerId;
use rustc_span::Span;
// 

use trait_fixer_hir_info_trait::HirInfo; // Import the trait

// Newtype struct to wrap rustc_hir::Item
pub struct RealHirInfoItem<'tcx>(pub Item<'tcx>);

// Implementation for RealHirInfoItem
impl<'tcx> HirInfo<'tcx> for RealHirInfoItem<'tcx> {
    fn get_owner_id(&self) -> OwnerId {
        self.0.owner_id
    }

    fn get_item_kind<'a>(&'a self) -> &'a ItemKind<'tcx> {
        &self.0.kind
    }

    fn get_item_span(&self) -> Span {
        self.0.span
    }
}

// Helper function to convert Item to RealHirInfoItem if needed elsewhere
// pub fn to_real_hir_info_item(item: Item) -> RealHirInfoItem {
//     RealHirInfoItem(item)
// }
