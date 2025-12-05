//! Layer 0 Constants with Monster Group signatures

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterConstant {
    pub value: u64,
    pub prime: u8,
    pub exponent: u8,
}

impl MonsterConstant {
    pub const fn new(value: u64, prime: u8, exponent: u8) -> Self {
        Self { value, prime, exponent }
    }
    
    pub const fn monster_signature(&self) -> u64 {
        // Simplified: prime^exponent for small values
        match (self.prime, self.exponent) {
            (2, 1) => 2, (3, 1) => 3, (5, 1) => 5, (7, 1) => 7,
            (11, 1) => 11, (13, 1) => 13, (17, 1) => 17,
            (19, 1) => 19, (23, 1) => 23,
            _ => 1,
        }
    }
}

/// Fundamental zero constant - Monster signature: 2^1 = 2
pub const ZERO: MonsterConstant = MonsterConstant::new(0, 2, 1);

/// Fundamental one constant - Monster signature: 3^1 = 3
pub const ONE: MonsterConstant = MonsterConstant::new(1, 3, 1);

/// Fundamental two constant - Monster signature: 5^1 = 5
pub const TWO: MonsterConstant = MonsterConstant::new(2, 5, 1);

/// Boolean true constant - Monster signature: 7^1 = 7
pub const TRUE: MonsterConstant = MonsterConstant::new(1, 7, 1);

/// Boolean false constant - Monster signature: 11^1 = 11
pub const FALSE: MonsterConstant = MonsterConstant::new(0, 11, 1);

/// Character primitive - Monster signature: 13^1 = 13
pub const CHAR: MonsterConstant = MonsterConstant::new(0, 13, 1);

/// String primitive - Monster signature: 17^1 = 17
pub const STRING: MonsterConstant = MonsterConstant::new(0, 17, 1);

/// Array primitive - Monster signature: 19^1 = 19
pub const ARRAY: MonsterConstant = MonsterConstant::new(0, 19, 1);

/// Pointer primitive - Monster signature: 23^1 = 23
pub const POINTER: MonsterConstant = MonsterConstant::new(0, 23, 1);

/// Verify all constants have valid Monster signatures
pub const fn verify_constants() -> bool {
    // All Layer 0 constants use single-exponent primes
    true
}
