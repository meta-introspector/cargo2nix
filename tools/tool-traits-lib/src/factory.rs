use crate::regex_adapter::{CurrentRegexAdapter, RegexAdapter};
use crate::serde_adapter::{CurrentSerdeAdapter, SerdeAdapter};
use crate::toml_adapter::{CurrentTomlAdapter, TomlAdapter};
use crate::walkdir_adapter::{CurrentWalkDirAdapter, WalkDirAdapter};

pub trait AdapterFactory: Send + Sync {
    fn create_serde_adapter(&self) -> Box<dyn SerdeAdapter>;
    fn create_toml_adapter(&self) -> Box<dyn TomlAdapter>;
    fn create_regex_adapter(&self) -> Box<dyn RegexAdapter>;
    fn create_walkdir_adapter(&self) -> Box<dyn WalkDirAdapter>;
}

pub struct DefaultAdapterFactory;

impl AdapterFactory for DefaultAdapterFactory {
    fn create_serde_adapter(&self) -> Box<dyn SerdeAdapter> {
        Box::new(CurrentSerdeAdapter)
    }

    fn create_toml_adapter(&self) -> Box<dyn TomlAdapter> {
        Box::new(CurrentTomlAdapter)
    }

    fn create_regex_adapter(&self) -> Box<dyn RegexAdapter> {
        Box::new(CurrentRegexAdapter)
    }

    fn create_walkdir_adapter(&self) -> Box<dyn WalkDirAdapter> {
        Box::new(CurrentWalkDirAdapter)
    }
}
