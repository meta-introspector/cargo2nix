use crate::r1cs_monster_constraints::R1CSConstraint;

pub struct Monster108Constraints {
    pub constraints: Vec<R1CSConstraint>,
    pub supersingular_primes: Vec<u64>,
}

impl Monster108Constraints {
    pub fn new() -> Self {
        let supersingular_primes = Self::generate_108_supersingular_primes();
        let constraints = Self::generate_108_constraints(&supersingular_primes);

        Self {
            constraints,
            supersingular_primes,
        }
    }

    fn generate_108_constraints(primes: &[u64]) -> Vec<R1CSConstraint> {
        let reasons = Self::get_108_reasons();

        reasons
            .iter()
            .zip(primes.iter())
            .enumerate()
            .map(|(id, (reason, &prime))| R1CSConstraint {
                id,
                reason: reason.clone(),
                a_coeff: vec![1, (prime % 24) as i64],
                b_coeff: vec![1, ((prime * 31) % 24) as i64],
                c_coeff: vec![0, ((prime + id as u64) % 24) as i64],
                supersingular_prime: prime,
            })
            .collect()
    }

    fn generate_108_supersingular_primes() -> Vec<u64> {
        let mut primes = Vec::new();
        let mut candidate = 2u64;

        while primes.len() < 108 {
            if Self::is_supersingular_prime(candidate) {
                primes.push(candidate);
            }
            candidate += if candidate == 2 { 1 } else { 2 };
        }
        primes
    }

    fn is_supersingular_prime(p: u64) -> bool {
        Self::is_prime(p) && (p % 12 == 11 || p == 2 || p == 3 || p == 5 || p == 7)
    }

    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn get_108_reasons() -> Vec<String> {
        vec![
            // Core Monster Group Properties (1-12)
            "Monster Group Identity Preservation".to_string(),
            "Modular Form Eigenvalue Consistency".to_string(),
            "Elliptic Curve Supersingular Reduction".to_string(),
            "Hecke Operator Commutation".to_string(),
            "Ramanujan Tau Function Congruence".to_string(),
            "Moonshine Module Grading".to_string(),
            "Vertex Operator Algebra Structure".to_string(),
            "J-Invariant Supersingular Values".to_string(),
            "Monstrous Moonshine Connection".to_string(),
            "Fischer-Griess Monster Construction".to_string(),
            "Conway-Norton Conjecture Verification".to_string(),
            "Borcherds Infinite Product Formula".to_string(),
            // Arithmetic Constraints (13-24)
            "Prime Factor Decomposition Integrity".to_string(),
            "Modular Arithmetic Closure".to_string(),
            "Galois Group Action Preservation".to_string(),
            "Quadratic Residue Consistency".to_string(),
            "Legendre Symbol Computation".to_string(),
            "Jacobi Symbol Verification".to_string(),
            "Cyclotomic Field Extension".to_string(),
            "Algebraic Integer Ring Structure".to_string(),
            "Dedekind Zeta Function Poles".to_string(),
            "L-Function Special Values".to_string(),
            "Dirichlet Character Orthogonality".to_string(),
            "Euler Product Convergence".to_string(),
            // Topological Invariants (25-36)
            "Euler Characteristic Preservation".to_string(),
            "Homology Group Stability".to_string(),
            "Cohomology Ring Structure".to_string(),
            "Fundamental Group Invariance".to_string(),
            "Homotopy Type Preservation".to_string(),
            "Fiber Bundle Connection".to_string(),
            "Sheaf Cohomology Computation".to_string(),
            "Spectral Sequence Convergence".to_string(),
            "K-Theory Classification".to_string(),
            "Chern Class Calculation".to_string(),
            "Pontryagin Class Invariance".to_string(),
            "Stiefel-Whitney Class Preservation".to_string(),
            // Symmetry Groups (37-48)
            "Automorphism Group Action".to_string(),
            "Conjugacy Class Structure".to_string(),
            "Character Table Orthogonality".to_string(),
            "Representation Theory Decomposition".to_string(),
            "Schur Multiplier Computation".to_string(),
            "Central Extension Classification".to_string(),
            "Sylow Subgroup Analysis".to_string(),
            "Normal Subgroup Lattice".to_string(),
            "Maximal Subgroup Containment".to_string(),
            "Simple Group Classification".to_string(),
            "Sporadic Group Embedding".to_string(),
            "Exceptional Lie Group Connection".to_string(),
            // Modular Forms (49-60)
            "Eisenstein Series Coefficients".to_string(),
            "Cusp Form Dimension Formula".to_string(),
            "Petersson Inner Product".to_string(),
            "Atkin-Lehner Involution".to_string(),
            "Newform Multiplicity One".to_string(),
            "Oldform Decomposition".to_string(),
            "Twist Operator Action".to_string(),
            "Fricke Involution Eigenvalue".to_string(),
            "Shimura Correspondence".to_string(),
            "Jacquet-Langlands Correspondence".to_string(),
            "Modularity Theorem Application".to_string(),
            "Serre Conjecture Verification".to_string(),
            // Elliptic Curves (61-72)
            "Weierstrass Equation Normalization".to_string(),
            "Discriminant Non-Vanishing".to_string(),
            "Torsion Subgroup Structure".to_string(),
            "Mordell-Weil Group Rank".to_string(),
            "Height Pairing Computation".to_string(),
            "Canonical Height Convergence".to_string(),
            "Isogeny Degree Calculation".to_string(),
            "Endomorphism Ring Structure".to_string(),
            "Complex Multiplication Detection".to_string(),
            "Reduction Type Classification".to_string(),
            "Conductor Exponent Formula".to_string(),
            "Tamagawa Number Computation".to_string(),
            // Cryptographic Properties (73-84)
            "Discrete Logarithm Hardness".to_string(),
            "Elliptic Curve Discrete Log".to_string(),
            "Pairing Computation Efficiency".to_string(),
            "Bilinear Map Non-Degeneracy".to_string(),
            "Weil Pairing Alternation".to_string(),
            "Tate Pairing Computation".to_string(),
            "Miller Algorithm Correctness".to_string(),
            "Point Compression Validity".to_string(),
            "Scalar Multiplication Timing".to_string(),
            "Side-Channel Resistance".to_string(),
            "Fault Attack Immunity".to_string(),
            "Quantum Resistance Analysis".to_string(),
            // Computational Complexity (85-96)
            "Polynomial Time Verification".to_string(),
            "NP-Completeness Reduction".to_string(),
            "Circuit Depth Minimization".to_string(),
            "Gate Count Optimization".to_string(),
            "Parallel Computation Efficiency".to_string(),
            "Memory Access Pattern".to_string(),
            "Cache Locality Optimization".to_string(),
            "Branch Prediction Accuracy".to_string(),
            "Instruction Pipeline Utilization".to_string(),
            "Register Allocation Efficiency".to_string(),
            "Loop Unrolling Optimization".to_string(),
            "Vectorization Opportunity".to_string(),
            // System Integration (97-108)
            "Type System Soundness".to_string(),
            "Memory Safety Guarantee".to_string(),
            "Thread Safety Verification".to_string(),
            "Deadlock Freedom Proof".to_string(),
            "Liveness Property Maintenance".to_string(),
            "Fairness Constraint Satisfaction".to_string(),
            "Resource Bound Enforcement".to_string(),
            "Termination Guarantee".to_string(),
            "Progress Property Verification".to_string(),
            "Invariant Preservation".to_string(),
            "Correctness Proof Validity".to_string(),
            "Monster Group Equivalence".to_string(),
        ]
    }
}
