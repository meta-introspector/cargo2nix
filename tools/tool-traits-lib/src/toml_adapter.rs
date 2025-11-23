use std::fmt::Debug;
use toml_edit::{Document, Item, Table, Value};

pub trait TomlAdapter: Send + Sync {
    fn parse_document(&self, s: &str) -> Result<Document, String>;
    fn to_string_pretty(&self, doc: &Document) -> String;
    fn get_table_entry<'a>(&self, doc: &'a Document, key: &str) -> Option<&'a Table>;
    fn get_value_entry<'a>(&self, doc: &'a Document, key: &str) -> Option<&'a Value>;
    fn insert_table_entry(&self, doc: &mut Document, key: &str, table: Table);
    fn insert_value_entry(&self, doc: &mut Document, key: &str, value: Value);
}

// Dummy implementation remains in tool-traits-lib
#[cfg(not(feature = "toml_edit_enabled"))]
pub struct DummyTomlAdapter;

#[cfg(not(feature = "toml_edit_enabled"))]
impl TomlAdapter for DummyTomlAdapter {
    fn parse_document(&self, _s: &str) -> Result<Document, String> {
        Err("TOML edit feature not enabled.".to_string())
    }

    fn to_string_pretty(&self, _doc: &Document) -> String {
        "TOML edit feature not enabled.".to_string()
    }

    fn get_table_entry<'a>(&self, _doc: &'a Document, _key: &str) -> Option<&'a Table> {
        None
    }

    fn get_value_entry<'a>(&self, _doc: &'a Document, _key: &str) -> Option<&'a Value> {
        None
    }

    fn insert_table_entry(&self, _doc: &mut Document, _key: &str, _table: Table) {
        // No-op
    }

    fn insert_value_entry(&self, _doc: &mut Document, _key: &str, _value: Value) {
        // No-op
    }
}

// CurrentTomlAdapter will now conditionally use RealTomlAdapter from the new crate
#[cfg(all(
    feature = "toml_edit_enabled",
    feature = "real_toml_adapter_lib_enabled"
))]
pub type CurrentTomlAdapter = real_toml_adapter_lib::RealTomlAdapter;
#[cfg(not(all(
    feature = "toml_edit_enabled",
    feature = "real_toml_adapter_lib_enabled"
)))]
pub type CurrentTomlAdapter = DummyTomlAdapter;
