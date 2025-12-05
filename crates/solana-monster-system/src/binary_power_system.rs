use crate::r1cs_monster_constraints::R1CSConstraint;

#[derive(Debug, Clone)]
pub struct BinaryPowerSystem {
    pub power_of_two: u32, // 46 from Monster Group 2^46
    pub binary_constraints: Vec<R1CSConstraint>,
    pub bit_decomposition: Vec<bool>, // 46-bit representation
}

impl BinaryPowerSystem {
    pub fn new() -> Self {
        let power_of_two = 46; // Monster Group 2^46 factor
        let binary_constraints = Self::generate_binary_constraints(power_of_two);
        let bit_decomposition = Self::decompose_to_bits(power_of_two as u64);

        Self {
            power_of_two,
            binary_constraints,
            bit_decomposition,
        }
    }

    pub fn verify_binary_constraint(&self, constraint_id: usize, witness: &[i64]) -> bool {
        if constraint_id >= self.binary_constraints.len() {
            return false;
        }

        let constraint = &self.binary_constraints[constraint_id];
        let a_val = Self::dot_product(&constraint.a_coeff, witness);
        let b_val = Self::dot_product(&constraint.b_coeff, witness);
        let c_val = Self::dot_product(&constraint.c_coeff, witness);

        // Binary constraint: a * b = c (mod 2)
        (a_val * b_val) % 2 == c_val % 2
    }

    pub fn generate_power_of_two_encoding(&self, value: u64) -> Vec<i64> {
        let mut encoding = Vec::new();
        let mut current = value;

        for i in 0..self.power_of_two {
            encoding.push((current % 2) as i64);
            current /= 2;
            if current == 0 {
                break;
            }
        }

        // Pad to full 46 bits
        while encoding.len() < self.power_of_two as usize {
            encoding.push(0);
        }

        encoding
    }

    pub fn verify_power_consistency(&self, encoded_value: &[i64]) -> bool {
        if encoded_value.len() != self.power_of_two as usize {
            return false;
        }

        // Each bit must be 0 or 1
        encoded_value.iter().all(|&bit| bit == 0 || bit == 1)
    }

    fn generate_binary_constraints(power: u32) -> Vec<R1CSConstraint> {
        (0..power)
            .map(|i| R1CSConstraint {
                id: i as usize,
                reason: format!("Binary Power 2^{} Bit {} Constraint", power, i),
                a_coeff: vec![1, 1],    // Binary multiplication
                b_coeff: vec![1, 1],    // Binary multiplication
                c_coeff: vec![0, 1],    // Result must be binary
                supersingular_prime: 2, // Prime 2 for binary operations
            })
            .collect()
    }

    fn decompose_to_bits(value: u64) -> Vec<bool> {
        (0..46).map(|i| (value >> i) & 1 == 1).collect()
    }

    fn dot_product(coeffs: &[i64], values: &[i64]) -> i64 {
        coeffs.iter().zip(values.iter()).map(|(a, b)| a * b).sum()
    }
}
