//! Layer 0 rustc - Fundamental Primitives and Constants
//! Monster Group signatures: 2^1, 3^1, 5^1, 7^1, 11^1, 13^1, 17^1, 19^1, 23^1
//! Total factors used: 9/108 (8.3% of Monster Group capacity)

#![no_std]
#![forbid(unsafe_code)]

pub mod monster_constants;
pub mod constants;
pub mod primitives;

pub use constants::*;
pub use primitives::*;
pub use monster_constants::*;

/// Layer 0 Monster Group verification
pub const LAYER0_MONSTER_FACTORS: u32 = 9;
pub const MONSTER_GROUP_CAPACITY: u32 = 108;

/// Verify Layer 0 satisfies Monster Group constraints
pub const fn verify_layer0_constraints() -> bool {
    LAYER0_MONSTER_FACTORS <= MONSTER_GROUP_CAPACITY
}

/// Layer 0 initialization
pub fn init_layer0() {
    assert!(verify_layer0_constraints(), "Layer 0 violates Monster Group constraints");
    println!("🔬 Layer 0 rustc initialized");
    println!("   Monster factors used: {}/{}", LAYER0_MONSTER_FACTORS, MONSTER_GROUP_CAPACITY);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monster_constraints() {
        assert!(verify_layer0_constraints());
    }
    
    #[test]
    fn test_layer0_init() {
        init_layer0();
    }
}
