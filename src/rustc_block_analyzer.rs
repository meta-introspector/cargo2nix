use crate::rustc_monster_equivalence::{MonsterGroupEquivalence, RustcBlock};
use crate::minizinc_data::MinizincInput;

pub struct RustcBlockAnalyzer {
    equivalence: MonsterGroupEquivalence,
}

impl RustcBlockAnalyzer {
    pub fn new() -> Self {
        Self {
            equivalence: MonsterGroupEquivalence::new(),
        }
    }

    pub fn analyze_rustc_components(&mut self) -> Vec<RustcBlock> {
        let rustc_components = [
            "rustc_parse", "rustc_ast", "rustc_hir", "rustc_middle", 
            "rustc_ty", "rustc_mir", "rustc_codegen", "rustc_driver",
            "rustc_lexer", "rustc_span", "rustc_errors", "rustc_session"
        ];

        rustc_components.iter()
            .map(|name| self.equivalence.construct_rustc_block(name))
            .collect()
    }

    pub fn find_supersingular_matches(&self, target_constraint: i32) -> Vec<String> {
        self.equivalence.query_matching_blocks(target_constraint)
            .iter()
            .map(|block| format!("{} (prime: {}, index: {})", 
                 block.name, block.prime_factor, block.supersingular_index))
            .collect()
    }

    pub fn verify_rustc_monster_equivalence(&self) -> (bool, String) {
        let is_equivalent = self.equivalence.verify_rustc_monster_equivalence();
        let status = if is_equivalent {
            "✓ rustc ≡ M: All blocks satisfy Monster Group constraints"
        } else {
            "✗ rustc ≢ M: Some blocks violate Monster Group constraints"
        };
        (is_equivalent, status.to_string())
    }

    pub fn generate_minizinc_constraints(&self) -> MinizincInput {
        let total_blocks = self.equivalence.rustc_blocks.len() as i32;
        let avg_constraint = self.equivalence.rustc_blocks.values()
            .map(|b| b.monster_constraint)
            .sum::<i32>() / total_blocks.max(1);

        MinizincInput {
            elliptic_fiber: avg_constraint % 24,
            torus_x: total_blocks % 24,
            torus_y: (avg_constraint + total_blocks) % 24,
            monster_stabilizer: 108, // 108 supersingular primes
        }
    }

    pub fn transform_all_blocks(&self) -> Vec<(String, u64, i32)> {
        self.equivalence.rustc_blocks.values()
            .map(|block| {
                let (monster_element, constraint) = self.equivalence.transform_block_to_monster(block);
                (block.name.clone(), monster_element, constraint)
            })
            .collect()
    }
}
