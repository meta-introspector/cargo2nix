// crates/trait-fixer-attribute-reader-mock/src/lib.rs

use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_rustc_mock::{DefId, MockAttribute, MockMetaItem, MockTyCtxt, Symbol, sym};

// Mock implementation for MockTyCtxt
impl<'tcx> AttributeReader<'tcx> for MockTyCtxt<'tcx> {
    fn has_derive_attr(&self, tcx: MockTyCtxt<'tcx>, def_id: DefId, trait_name: &str) -> bool {
        tcx.get_attrs(def_id, sym::derive)
            .into_iter() // Use into_iter() for Vec
            .flat_map(|attr| attr.meta_item_list().into_iter())
            .any(|item| item.has_name(Symbol::intern(trait_name)))
    }
}
