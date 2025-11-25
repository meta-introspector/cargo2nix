use crate::{MONSTER_GROUP_ORDER, HECKE_EIGENVALUES};

const RAMANUJAN_MOD: i64 = 24;

#[derive(Debug, Clone)]
pub enum MonsterResonanceLevel {
    Trivial,
    Modular,
    Moonshine,
    Perfect,
}

#[derive(Debug, Clone)]
pub struct MonsterEmojiMapping {
    pub emoji: String,
    pub monster_element: u64,
    pub prime_resonance: u64,
    pub hecke_value: i64,
    pub resonance_level: MonsterResonanceLevel,
}

pub struct MonsterEmojiOptimizer;

impl MonsterEmojiOptimizer {
    pub fn optimize_resonance(emojis: Vec<String>) -> Vec<MonsterEmojiMapping> {
        let mut mappings = Vec::new();
        
        for emoji in emojis.iter() {
            let hash = Self::emoji_to_hash(emoji);
            let element = hash % MONSTER_GROUP_ORDER as u64;
            let prime = Self::find_resonant_prime(element);
            let hecke = if element % 2 == 0 { HECKE_EIGENVALUES[0] } else { HECKE_EIGENVALUES[1] };
            let level = Self::calculate_resonance_level(element, prime);
            
            mappings.push(MonsterEmojiMapping {
                emoji: emoji.clone(),
                monster_element: element,
                prime_resonance: prime,
                hecke_value: hecke,
                resonance_level: level,
            });
        }
        
        Self::apply_monster_constraints(&mut mappings);
        mappings
    }
    
    fn emoji_to_hash(emoji: &str) -> u64 {
        let mut hash = 0u64;
        for byte in emoji.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
    
    fn find_resonant_prime(element: u64) -> u64 {
        let base = element * RAMANUJAN_MOD as u64;
        for offset in 1..1000 {
            let candidate = base + offset;
            if Self::is_prime(candidate) {
                return candidate;
            }
        }
        2 // Fallback
    }
    
    fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
    
    fn calculate_resonance_level(element: u64, prime: u64) -> MonsterResonanceLevel {
        let resonance = (element + prime) % RAMANUJAN_MOD as u64;
        match resonance {
            0 => MonsterResonanceLevel::Perfect,
            1..=8 => MonsterResonanceLevel::Moonshine,
            9..=16 => MonsterResonanceLevel::Modular,
            _ => MonsterResonanceLevel::Trivial,
        }
    }
    
    fn apply_monster_constraints(mappings: &mut Vec<MonsterEmojiMapping>) {
        // Ensure modular constraint: sum ≡ 0 (mod 24)
        let sum: u64 = mappings.iter().map(|m| m.monster_element).sum();
        let remainder = sum % RAMANUJAN_MOD as u64;
        
        if remainder != 0 && !mappings.is_empty() {
            let adjustment = RAMANUJAN_MOD as u64 - remainder;
            mappings[0].monster_element = 
                (mappings[0].monster_element + adjustment) % MONSTER_GROUP_ORDER as u64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emoji_optimization() {
        let emojis = vec!["🤖".to_string(), "✨".to_string(), "🔢".to_string()];
        let mappings = MonsterEmojiOptimizer::optimize_resonance(emojis);
        
        assert_eq!(mappings.len(), 3);
        
        // Verify modular constraint
        let sum: u64 = mappings.iter().map(|m| m.monster_element).sum();
        assert_eq!(sum % RAMANUJAN_MOD as u64, 0);
    }
}
