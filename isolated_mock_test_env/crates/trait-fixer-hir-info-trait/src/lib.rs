// crates/trait-fixer-hir-info-trait/src/lib.rs

pub trait HirInfo<'tcx> {
    type Item: 'tcx;
    type OwnerId;
    type ItemKind: 'tcx;
    type Span;

    fn get_owner_id(&'tcx self) -> Self::OwnerId;
    fn get_item_kind(&'tcx self) -> &'tcx Self::ItemKind;
    fn get_item_span(&'tcx self) -> Self::Span;
}