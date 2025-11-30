// crates/trait-fixer-attribute-reader-trait/src/lib.rs

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;

pub trait AttributeReader<'tcx> {
    fn has_derive_attr(&self, tcx: TyCtxt<'tcx>, def_id: DefId, trait_name: &str) -> bool;
}
