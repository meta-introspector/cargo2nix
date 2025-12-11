// crates/trait-fixer-hir-info-mock/src/lib.rs

use std::marker::PhantomData;
use trait_fixer_hir_info_trait::HirInfo;

// Import the real rustc types that the HirInfo trait expects
// Use aliases to avoid direct naming conflicts if trait_fixer_rustc_mock also defines them.
use rustc_hir::def_id::{DefId, LocalDefId, OwnerId, CRATE_DEF_INDEX};
use rustc_hir::hir::ItemKind;
use rustc_span::{Span, BytePos, DUMMY_SPANA, DUMMY_CRATE_ID};

// Import the mock Item type to be wrapped
use trait_fixer_rustc_mock::Item; // Keep this for the MockItem struct

// Newtype wrapper to satisfy the orphan rule.
pub struct MockItem<'tcx>(pub Item<'tcx>, PhantomData<&'tcx ()>);

// Implementation for Mock Item directly.
impl<'tcx> HirInfo<'tcx> for MockItem<'tcx> {
    fn get_owner_id(&self) -> OwnerId {
        // OwnerId::DUMMY no longer exists. Construct a dummy OwnerId.
        // A valid dummy DefId needs a dummy CrateNum and a dummy DefIndex.
        // rustc_span::DUMMY_CRATE_ID and rustc_hir::def_id::CRATE_DEF_INDEX are suitable.
        OwnerId { def_id: DefId { krate: DUMMY_CRATE_ID, index: CRATE_DEF_INDEX } }
    }

    fn get_item_kind<'a>(&'a self) -> &'a ItemKind<'tcx> {
        // Assuming ItemKind::Struct is a valid variant for mocking.
        // This might need adjustment if ItemKind API changed.
        &ItemKind::Struct(PhantomData)
    }

    fn get_item_span(&self) -> Span {
        // Span::DUMMY no longer exists. Use DUMMY_SPANA.
        DUMMY_SPANA
    }
}
