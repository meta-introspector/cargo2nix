use toml_edit::{Document, Table, Value};
use tool_traits_lib::toml_adapter::TomlAdapter;

pub struct RealTomlAdapter;

impl TomlAdapter for RealTomlAdapter {
    fn parse_document(&self, s: &str) -> Result<Document<String>, String> {
        s.parse::<Document<String>>()
            .map_err(|e| format!("Failed to parse TOML document: {:?}", e))
    }

    fn to_string_pretty(&self, doc: &Document<String>) -> String {
        doc.to_string()
    }

    fn get_table_entry<'a>(&self, doc: &'a Document<String>, key: &str) -> Option<&'a Table> {
        doc.get(key).and_then(|item| item.as_table())
    }

    fn get_value_entry<'a>(&self, doc: &'a Document<String>, key: &str) -> Option<&'a Value> {
        doc.get(key).and_then(|item| item.as_value())
    }

    fn insert_table_entry(&self, _doc: &mut Document<String>, _key: &str, _table: Table) {
        unimplemented!()
    }

    fn insert_value_entry(&self, _doc: &mut Document<String>, _key: &str, _value: Value) {
        unimplemented!()
    }
}
