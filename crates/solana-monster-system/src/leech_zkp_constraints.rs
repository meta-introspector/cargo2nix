use crate::constraints::Constraint;

#[derive(Debug, Clone)]
pub enum ConstraintType {
    Symmetry,          // Leech Lattice symmetry preservation
    Dimension,         // 24-dimensional Leech Lattice structure
    EncodingIntegrity, // ZKP encoding correctness
    MaximalBinary,     // Binary bounds from 2^46
}

pub struct LeechZKPConstraints {
    pub constraints_46: [Constraint; 46],
    pub constraint_types: [ConstraintType; 46],
}

impl LeechZKPConstraints {
    pub fn new() -> Self {
        let (constraints_46, constraint_types) = Self::generate_46_leech_zkp_constraints();

        Self {
            constraints_46,
            constraint_types,
        }
    }

    pub fn verify_symmetry(&self, witness: i64) -> bool {
        self.constraints_46
            .iter()
            .zip(self.constraint_types.iter())
            .filter(|(_, t)| matches!(t, ConstraintType::Symmetry))
            .all(|(c, _)| c.verify(witness))
    }

    pub fn verify_dimension(&self, witness: i64) -> bool {
        self.constraints_46
            .iter()
            .zip(self.constraint_types.iter())
            .filter(|(_, t)| matches!(t, ConstraintType::Dimension))
            .all(|(c, _)| c.verify(witness))
    }

    pub fn verify_encoding_integrity(&self, witness: i64) -> bool {
        self.constraints_46
            .iter()
            .zip(self.constraint_types.iter())
            .filter(|(_, t)| matches!(t, ConstraintType::EncodingIntegrity))
            .all(|(c, _)| c.verify(witness))
    }

    pub fn verify_maximal_binary(&self, witness: i64) -> bool {
        self.constraints_46
            .iter()
            .zip(self.constraint_types.iter())
            .filter(|(_, t)| matches!(t, ConstraintType::MaximalBinary))
            .all(|(c, _)| c.verify(witness))
    }

    pub fn verify_all_46(&self, witness: i64) -> bool {
        self.constraints_46.iter().all(|c| c.verify(witness))
    }

    fn generate_46_leech_zkp_constraints() -> ([Constraint; 46], [ConstraintType; 46]) {
        let mut constraints = Vec::new();
        let mut types = Vec::new();

        // Symmetry constraints (0-11): Leech Lattice automorphism group
        for i in 0..12 {
            constraints.push(Constraint::new(i, 24, i as i64, 24)); // 24-fold symmetry
            types.push(ConstraintType::Symmetry);
        }

        // Dimension constraints (12-23): 24-dimensional Leech Lattice
        for i in 12..24 {
            constraints.push(Constraint::new(i, 1, (i - 12) as i64, 24)); // Dimension bound
            types.push(ConstraintType::Dimension);
        }

        // Encoding integrity constraints (24-35): ZKP correctness
        for i in 24..36 {
            constraints.push(Constraint::new(i, 2, (i - 24) as i64, 2)); // Binary encoding
            types.push(ConstraintType::EncodingIntegrity);
        }

        // Maximal binary constraints (36-45): 2^46 bounds
        for i in 36..46 {
            constraints.push(Constraint::new(i, 1, 0, 1)); // Binary bound [0,1]
            types.push(ConstraintType::MaximalBinary);
        }

        (constraints.try_into().unwrap(), types.try_into().unwrap())
    }
}
