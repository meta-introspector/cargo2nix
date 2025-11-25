// Monster Group Trait Registry from Declaration Splitter
use std::collections::HashMap;

pub struct MonsterTraitRegistry {
    traits_by_factor: HashMap<u64, Vec<&'static str>>,
}

impl MonsterTraitRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            traits_by_factor: HashMap::new(),
        };
        registry
    }
}
