use std::path::Path;

pub trait CargoTomlParser: Send + Sync {
    fn get_package_repository(&self, path: &Path) -> std::result::Result<Option<String>, String>;
}
