use crate::r1cs_monster_constraints::MonsterR1CS;
use crate::rustc_monster_equivalence::RustcBlock;
use crate::minizinc_data::MinizincInput;

pub struct ZKPMonsterCircuit {
    pub r1cs: MonsterR1CS,
    pub rustc_blocks: Vec<RustcBlock>,
}

impl ZKPMonsterCircuit {
    pub fn new() -> Self {
        Self {
            r1cs: MonsterR1CS::new(),
            rustc_blocks: Vec::new(),
        }
    }

    pub fn setup_rustc_constraints(&mut self, blocks: Vec<RustcBlock>, supersingular_primes: &[u64]) {
        self.rustc_blocks = blocks;
        self.r1cs.generate_108_constraints(supersingular_primes);
        
        // Generate witness from rustc blocks
        let witness: Vec<i64> = self.rustc_blocks.iter()
            .map(|block| block.monster_constraint as i64)
            .chain(std::iter::repeat(0)) // Pad to required length
            .take(108)
            .collect();
        
        self.r1cs.witness = witness;
        
        // Public inputs from Monster Group constants
        self.r1cs.public_inputs = vec![
            24, // Monster Group mod 24
            196883 % 1000, // Monster Group order (truncated)
            -5472 % 1000, // Hecke eigenvalue (truncated)
        ];
    }

    pub fn verify_all_constraints(&self) -> (usize, usize) {
        let mut passed = 0;
        let total = self.r1cs.constraints.len();
        
        for i in 0..total {
            if self.r1cs.verify_constraint(i, &self.r1cs.witness) {
                passed += 1;
            }
        }
        
        (passed, total)
    }

    pub fn generate_proof_input(&self) -> String {
        format!(
            "{{\"witness\": {:?}, \"public_inputs\": {:?}}}",
            self.r1cs.witness,
            self.r1cs.public_inputs
        )
    }

    pub fn to_minizinc_zkp_constraints(&self) -> MinizincInput {
        let (passed, total) = self.verify_all_constraints();
        let success_rate = if total > 0 { passed * 24 / total } else { 0 };
        
        MinizincInput {
            elliptic_fiber: success_rate as i32,
            torus_x: passed as i32 % 24,
            torus_y: total as i32 % 24,
            monster_stabilizer: 108, // 108 supersingular constraints
        }
    }

    pub fn export_circom_circuit(&self, filename: &str) -> Result<(), std::io::Error> {
        let circuit_code = self.r1cs.generate_circom_circuit();
        std::fs::write(filename, circuit_code)
    }
}
