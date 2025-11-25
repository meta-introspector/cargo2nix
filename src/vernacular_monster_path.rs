use crate::minizinc_data::MinizincInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VernacularEmbedding {
    pub rust_tokens: Vec<String>,
    pub semantic_weight: f64,
    pub embedding_vector: Vec<i32>, // Simplified embedding
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterTarget {
    pub element_id: i32, // Target Monster Group element
    pub stabilizer_class: i32,
    pub eigenvalue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPath {
    pub steps: Vec<i32>, // Path steps in Monster Group
    pub cost: f64,
    pub satisfiable: bool,
}

pub struct VernacularMonsterSolver {
    pub input_embedding: VernacularEmbedding,
    pub target_monster: MonsterTarget,
}

impl VernacularMonsterSolver {
    pub fn new(embedding: VernacularEmbedding, target: MonsterTarget) -> Self {
        Self {
            input_embedding: embedding,
            target_monster: target,
        }
    }

    pub fn to_minizinc_path_problem(&self) -> MinizincInput {
        let source_hash = self.hash_embedding(&self.input_embedding);
        let target_element = self.target_monster.element_id;
        
        MinizincInput {
            elliptic_fiber: source_hash % 24,
            torus_x: target_element % 24,
            torus_y: (source_hash + target_element) % 24,
            monster_stabilizer: self.target_monster.stabilizer_class,
        }
    }

    fn hash_embedding(&self, embedding: &VernacularEmbedding) -> i32 {
        let token_hash = embedding.rust_tokens.iter()
            .fold(0u64, |acc, token| acc.wrapping_mul(31).wrapping_add(
                token.bytes().fold(0u64, |a, b| a.wrapping_add(b as u64))
            ));
        
        let vector_hash = embedding.embedding_vector.iter()
            .fold(0i64, |acc, &val| acc.wrapping_add(val as i64));
        
        ((token_hash as i64 + vector_hash) % 196883) as i32
    }
}
