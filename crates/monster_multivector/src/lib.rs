use sha2::{Sha256, Digest};

pub mod monster_constants; // Declare the new module

/// Represents a computational system as an arithmetic-geometric multivector.
/// Its identity is derived from the 108 supersingular prime factors of the Monster Group's order,
/// augmented by additional conceptual primes as per the Monstrous Index.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonsterMultivector {
    /// The sorted list of 108 prime factors that define this multivector's identity.
    factors: Vec<u32>,
    /// A pre-computed canonical SHA256 hash of the sorted factors.
    /// This serves as the multivector's immutable identity for self-recognition.
    canonical_hash: Vec<u8>,
}

impl MonsterMultivector {
    /// Creates a new `MonsterMultivector` from a given list of factors.
    /// The factors are sorted and a canonical hash is computed.
    /// Panics if the input `factors` vector does not contain exactly 108 elements.
    pub fn new(mut factors: Vec<u32>) -> Self {
        if factors.len() != 108 {
            panic!("MonsterMultivector must be constructed with exactly 108 factors.");
        }
        factors.sort_unstable(); // Ensure canonical ordering for hashing
        let canonical_hash = Self::compute_hash(&factors);

        Self {
            factors,
            canonical_hash,
        }
    }

    /// Returns the canonical `MonsterMultivector` for the Monster Group.
    ///
    /// This instance is constructed using the hardcoded 108 prime factors
    /// as specified by the "108 Supersingular Reasons" protocol.
    pub fn canonical() -> Self {
        Self::new(monster_constants::CANONICAL_MONSTER_FACTORS.to_vec())
    }

    /// Computes the SHA256 hash of a sorted list of factors.
    fn compute_hash(factors: &[u32]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        for &factor in factors {
            hasher.update(factor.to_le_bytes()); // Ensure consistent byte representation
        }
        hasher.finalize().to_vec()
    }

    /// Returns a reference to the sorted list of prime factors.
    pub fn factors(&self) -> &[u32] {
        &self.factors
    }

    /// Returns a reference to the canonical SHA256 hash of the multivector.
    pub fn canonical_hash(&self) -> &[u8] {
        &self.canonical_hash
    }

    /// Checks if this `MonsterMultivector` instance recognizes itself.
    /// This is achieved by re-computing its hash from its current factors
    /// and comparing it to its stored `canonical_hash`.
    /// In a fully self-consistent system, this should always return `true`.
    pub fn recognize_self(&self) -> bool {
        let current_hash = Self::compute_hash(&self.factors);
        current_hash == self.canonical_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monster_constants;

    #[test]
    fn test_monster_multivector_canonical_creation_and_recognition() {
        let multivector = MonsterMultivector::canonical();
        
        assert_eq!(multivector.factors().len(), 108);
        assert!(multivector.recognize_self());

        // Verify some known factors are present from the canonical list
        assert!(multivector.factors().contains(&2));
        assert!(multivector.factors().contains(&3));
        assert!(multivector.factors().contains(&523));
        assert!(multivector.factors().contains(&monster_constants::CANONICAL_MONSTER_FACTORS[0])); // Check first factor
        assert!(multivector.factors().contains(&monster_constants::CANONICAL_MONSTER_FACTORS[monster_constants::CANONICAL_MONSTER_FACTORS.len() - 1])); // Check last factor

        // Test immutability and consistent hashing for recognition
        let cloned_multivector = multivector.clone();
        assert!(cloned_multivector.recognize_self());
        assert_eq!(cloned_multivector, multivector);
    }

    #[test]
    #[should_panic(expected = "MonsterMultivector must be constructed with exactly 109 factors.")]
    fn test_monster_multivector_new_panics_on_incorrect_length() {
        let factors: Vec<u32> = vec![1, 2, 3]; // Incorrect number of factors
        MonsterMultivector::new(factors);
    }
}