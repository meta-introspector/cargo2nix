use crate::r1cs_monster_constraints::R1CSConstraint;

pub struct FortySixSystem {
    pub exponent: u32, // 46
    pub constraints_46: [R1CSConstraint; 46],
}

impl FortySixSystem {
    pub fn new() -> Self {
        let exponent = 46;
        let constraints_46 = Self::generate_46_constraints();
        
        Self {
            exponent,
            constraints_46,
        }
    }

    pub fn verify_46_bit(&self, bit_position: usize, bit_value: i64) -> bool {
        if bit_position >= 46 { return false; }
        
        let constraint = &self.constraints_46[bit_position];
        // Verify bit is 0 or 1
        bit_value == 0 || bit_value == 1
    }

    pub fn encode_to_46_bits(&self, value: u64) -> [i64; 46] {
        let mut bits = [0i64; 46];
        let mut val = value;
        
        for i in 0..46 {
            bits[i] = (val & 1) as i64;
            val >>= 1;
        }
        
        bits
    }

    pub fn decode_from_46_bits(&self, bits: &[i64; 46]) -> u64 {
        bits.iter().enumerate()
            .map(|(i, &bit)| (bit as u64) << i)
            .sum()
    }

    fn generate_46_constraints() -> [R1CSConstraint; 46] {
        let mut constraints = Vec::new();
        
        for i in 0..46 {
            constraints.push(R1CSConstraint {
                id: i,
                reason: format!("Bit {} of 2^46 Monster Factor", i),
                a_coeff: vec![1, 1],
                b_coeff: vec![1, 0],
                c_coeff: vec![0, 1],
                supersingular_prime: 2,
            });
        }
        
        constraints.try_into().unwrap()
    }
}
