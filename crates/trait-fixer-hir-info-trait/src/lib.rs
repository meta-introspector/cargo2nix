// crates/trait-fixer-hir-info-trait/src/lib.rs

use rustc_hir::Item;
use rustc_hir::ItemKind;
use rustc_hir::OwnerId;
use rustc_span::Span;

pub trait HirInfo<'tcx> {
    fn get_owner_id(&self) -> OwnerId;
    fn get_item_kind<'a>(&'a self) -> &'a ItemKind<'tcx>;
    fn get_item_span(&self) -> Span;
}
