use toml_edit::{Document, Item, Table, Value};
use tool_traits_lib::toml_adapter::TomlAdapter;

pub struct RealTomlAdapter;

impl TomlAdapter for RealTomlAdapter {
    fn parse_document(&self, s: &str) -> Result<Document, String> {
        s.parse::<Document>()
            .map_err(|e| format!("Failed to parse TOML document: {:?}", e))
    }

    fn to_string_pretty(&self, doc: &Document) -> String {
        doc.to_string()
    }

    fn get_table_entry<'a>(&self, doc: &'a Document, key: &str) -> Option<&'a Table> {
        doc.get(key).and_then(|item| item.as_table())
    }

    fn get_value_entry<'a>(&self, doc: &'a Document, key: &str) -> Option<&'a Value> {
        doc.get(key).and_then(|item| item.as_value())
    }

    fn insert_table_entry(&self, doc: &mut Document, key: &str, table: Table) {
        doc.insert(key, Item::Table(table));
    }

    fn insert_value_entry(&self, doc: &mut Document, key: &str, value: Value) {
        doc.insert(key, Item::Value(value));
    }
}
