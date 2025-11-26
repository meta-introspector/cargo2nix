
use toml_edit::{Document, Table, Value};

pub trait TomlAdapter: Send + Sync {
    fn parse_document(&self, s: &str) -> Result<Document<String>, String>;
    fn to_string_pretty(&self, doc: &Document<String>) -> String;
    fn get_table_entry<'a>(&self, doc: &'a Document<String>, key: &str) -> Option<&'a Table>;
    fn get_value_entry<'a>(&self, doc: &'a Document<String>, key: &str) -> Option<&'a Value>;
    fn insert_table_entry(&self, doc: &mut Document<String>, key: &str, table: Table);
    fn insert_value_entry(&self, doc: &mut Document<String>, key: &str, value: Value);
}
