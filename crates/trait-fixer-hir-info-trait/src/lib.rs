// crates/trait-fixer-hir-info-trait/src/lib.rs

use std::fmt::Debug;
use std::hash::Hash;

// Define the HirInfo trait with associated types for OwnerId, ItemKind, and Span.
// This allows different implementations (mock or real) to provide their own types
// for these items, as long as they satisfy the specified bounds.
pub trait HirInfo<'tcx> {
    type OwnerId: Debug + Clone + Copy + PartialEq + Eq + Hash;
    type ItemKind: Debug + Clone + Copy + PartialEq + Eq + Hash + 'tcx; // 'tcx lifetime needed for ItemKind
    type Span: Debug + Clone + Copy + PartialEq + Eq + Hash;

    fn get_owner_id(&self) -> Self::OwnerId;
    fn get_item_kind<'a>(&'a self) -> &'a Self::ItemKind;
    fn get_item_span(&self) -> Self::Span;
}
