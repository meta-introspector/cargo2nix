use crate::minizinc_data::{EllipticFiber, MinizincInput, MonsterStabilizer, TorusPoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgebaseEntry {
    pub id: String,
    pub weight: f64,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KnowledgebaseFormatter {
    entries: HashMap<String, KnowledgebaseEntry>,
}

impl KnowledgebaseFormatter {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: KnowledgebaseEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn to_minizinc_input(&self) -> MinizincInput {
        let total_entries = self.entries.len() as i32;
        let avg_weight =
            self.entries.values().map(|e| e.weight).sum::<f64>() / total_entries as f64;

        let fiber = EllipticFiber {
            fiber_id: total_entries % 24, // Monster Group mod 24
            modular_constraint: 24,
        };

        let point = TorusPoint {
            x: (avg_weight * 10.0) as i32 % 24,
            y: self
                .entries
                .values()
                .map(|e| e.dependencies.len())
                .sum::<usize>() as i32
                % 24,
            resonance_level: (avg_weight * 100.0) as i32,
        };

        let stabilizer = MonsterStabilizer {
            stabilizer_id: total_entries,
            eigenvalue: avg_weight,
        };

        MinizincInput::from_monster_data(&fiber, &point, &stabilizer)
    }

    pub fn generate_dzn(&self) -> String {
        self.to_minizinc_input().to_string()
    }
}
