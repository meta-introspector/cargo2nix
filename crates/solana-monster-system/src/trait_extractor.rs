use crate::minizinc_data::{MinizincInput, EllipticFiber, TorusPoint, MonsterStabilizer};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitSignature {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub monster_element: i32, // Monster Group element (mod 196883)
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub id: String,
    pub consumes: HashSet<String>,
    pub produces: HashSet<String>,
    pub external_deps: HashSet<String>,
}

pub struct TraitExtractor {
    traits: HashMap<String, TraitSignature>,
    blocks: HashMap<String, CodeBlock>,
}

impl TraitExtractor {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn extract_trait(&mut self, name: &str, inputs: Vec<String>, outputs: Vec<String>) {
        let monster_element = self.hash_to_monster_element(&name);
        let trait_sig = TraitSignature {
            name: name.to_string(),
            inputs,
            outputs,
            monster_element,
        };
        self.traits.insert(name.to_string(), trait_sig);
    }

    pub fn add_code_block(&mut self, block: CodeBlock) {
        self.blocks.insert(block.id.clone(), block);
    }

    pub fn create_dummy_external(&self, external_name: &str) -> String {
        format!(
            "trait {} {{\n    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;\n}}",
            external_name
        )
    }

    pub fn to_minizinc_constraints(&self) -> MinizincInput {
        let total_traits = self.traits.len() as i32;
        let total_blocks = self.blocks.len() as i32;
        
        let fiber = EllipticFiber {
            fiber_id: total_traits % 24,
            modular_constraint: 24,
        };
        
        let point = TorusPoint {
            x: total_blocks % 24,
            y: (total_traits + total_blocks) % 24,
            resonance_level: self.calculate_resonance(),
        };
        
        let stabilizer = MonsterStabilizer {
            stabilizer_id: total_traits * total_blocks,
            eigenvalue: self.calculate_eigenvalue(),
        };

        MinizincInput::from_monster_data(&fiber, &point, &stabilizer)
    }

    fn hash_to_monster_element(&self, name: &str) -> i32 {
        let hash = name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        (hash % 196883) as i32 // Monster Group order
    }

    fn calculate_resonance(&self) -> i32 {
        self.traits.values()
            .map(|t| t.monster_element % 24)
            .sum::<i32>() % 100
    }

    fn calculate_eigenvalue(&self) -> f64 {
        let total_connections = self.blocks.values()
            .map(|b| b.consumes.len() + b.produces.len())
            .sum::<usize>();
        total_connections as f64 / self.blocks.len().max(1) as f64
    }
}
