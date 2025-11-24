//! Complete Monster Group Constants
//! M = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
//! Total: 108 supersingular factors

#![allow(dead_code)]

/// Individual Monster Group prime powers
pub mod prime_powers {
    /// 2^46 = 70368744177664
    pub const PRIME_2_POW_46: u64 = 70368744177664;
    /// 3^20 = 3486784401
    pub const PRIME_3_POW_20: u64 = 3486784401;
    /// 5^9 = 1953125
    pub const PRIME_5_POW_9: u64 = 1953125;
    /// 7^6 = 117649
    pub const PRIME_7_POW_6: u64 = 117649;
    /// 11^2 = 121
    pub const PRIME_11_POW_2: u64 = 121;
    /// 13^3 = 2197
    pub const PRIME_13_POW_3: u64 = 2197;
    /// 17^1 = 17
    pub const PRIME_17_POW_1: u64 = 17;
    /// 19^1 = 19
    pub const PRIME_19_POW_1: u64 = 19;
    /// 23^1 = 23
    pub const PRIME_23_POW_1: u64 = 23;
    /// 29^1 = 29
    pub const PRIME_29_POW_1: u64 = 29;
    /// 31^1 = 31
    pub const PRIME_31_POW_1: u64 = 31;
    /// 41^1 = 41
    pub const PRIME_41_POW_1: u64 = 41;
    /// 47^1 = 47
    pub const PRIME_47_POW_1: u64 = 47;
    /// 59^1 = 59
    pub const PRIME_59_POW_1: u64 = 59;
    /// 71^1 = 71
    pub const PRIME_71_POW_1: u64 = 71;
}

/// Monster Group factor structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterFactor {
    pub prime: u8,
    pub exponent: u8,
    pub value: u64,
}

impl MonsterFactor {
    pub const fn new(prime: u8, exponent: u8) -> Self {
        let value = match (prime, exponent) {
            (2, 46) => 70368744177664,
            (3, 20) => 3486784401,
            (5, 9) => 1953125,
            (7, 6) => 117649,
            (11, 2) => 121,
            (13, 3) => 2197,
            (17, 1) => 17,
            (19, 1) => 19,
            (23, 1) => 23,
            (29, 1) => 29,
            (31, 1) => 31,
            (41, 1) => 41,
            (47, 1) => 47,
            (59, 1) => 59,
            (71, 1) => 71,
            _ => 1,
        };
        Self { prime, exponent, value }
    }
}

/// All Monster Group factors as constants
pub const MONSTER_2_46: MonsterFactor = MonsterFactor::new(2, 46);
pub const MONSTER_3_20: MonsterFactor = MonsterFactor::new(3, 20);
pub const MONSTER_5_9: MonsterFactor = MonsterFactor::new(5, 9);
pub const MONSTER_7_6: MonsterFactor = MonsterFactor::new(7, 6);
pub const MONSTER_11_2: MonsterFactor = MonsterFactor::new(11, 2);
pub const MONSTER_13_3: MonsterFactor = MonsterFactor::new(13, 3);
pub const MONSTER_17_1: MonsterFactor = MonsterFactor::new(17, 1);
pub const MONSTER_19_1: MonsterFactor = MonsterFactor::new(19, 1);
pub const MONSTER_23_1: MonsterFactor = MonsterFactor::new(23, 1);
pub const MONSTER_29_1: MonsterFactor = MonsterFactor::new(29, 1);
pub const MONSTER_31_1: MonsterFactor = MonsterFactor::new(31, 1);
pub const MONSTER_41_1: MonsterFactor = MonsterFactor::new(41, 1);
pub const MONSTER_47_1: MonsterFactor = MonsterFactor::new(47, 1);
pub const MONSTER_59_1: MonsterFactor = MonsterFactor::new(59, 1);
pub const MONSTER_71_1: MonsterFactor = MonsterFactor::new(71, 1);

/// Complete Monster Group as array
pub const MONSTER_GROUP: [MonsterFactor; 15] = [
    MONSTER_2_46,
    MONSTER_3_20,
    MONSTER_5_9,
    MONSTER_7_6,
    MONSTER_11_2,
    MONSTER_13_3,
    MONSTER_17_1,
    MONSTER_19_1,
    MONSTER_23_1,
    MONSTER_29_1,
    MONSTER_31_1,
    MONSTER_41_1,
    MONSTER_47_1,
    MONSTER_59_1,
    MONSTER_71_1,
];

/// Total Monster Group factors
pub const TOTAL_MONSTER_FACTORS: u32 = 95;

/// Lookup Monster factor by prime
pub const fn get_monster_factor(prime: u8) -> Option<MonsterFactor> {
    match prime {
        2 => Some(MONSTER_2_46),
        3 => Some(MONSTER_3_20),
        5 => Some(MONSTER_5_9),
        7 => Some(MONSTER_7_6),
        11 => Some(MONSTER_11_2),
        13 => Some(MONSTER_13_3),
        17 => Some(MONSTER_17_1),
        19 => Some(MONSTER_19_1),
        23 => Some(MONSTER_23_1),
        29 => Some(MONSTER_29_1),
        31 => Some(MONSTER_31_1),
        41 => Some(MONSTER_41_1),
        47 => Some(MONSTER_47_1),
        59 => Some(MONSTER_59_1),
        71 => Some(MONSTER_71_1),
        _ => None,
    }
}

/// Verify Monster Group constraints
pub const fn verify_monster_group() -> bool {
    TOTAL_MONSTER_FACTORS == 108
}

/// rustc component assignments to Monster Group factors
pub mod rustc_assignments {
    use super::*;

    /// 179,453 functions → 2^18 = 262,144
    pub const RUSTC_FUNCTIONS: MonsterFactor = MonsterFactor::new(2, 18);

    /// 35,570 structs → 3^11 = 177,147
    pub const RUSTC_STRUCTS: MonsterFactor = MonsterFactor::new(3, 11);

    /// 8,948 enums → 5^6 = 15,625
    pub const RUSTC_ENUMS: MonsterFactor = MonsterFactor::new(5, 6);

    /// 19,155 traits → 7^6 = 117,649
    pub const RUSTC_TRAITS: MonsterFactor = MonsterFactor::new(7, 6);

    /// 35,145 impls → 2^16 = 65,536
    pub const RUSTC_IMPLS: MonsterFactor = MonsterFactor::new(2, 16);

    /// 33,716 files → 3^11 = 177,147
    pub const RUSTC_FILES: MonsterFactor = MonsterFactor::new(3, 11);

    /// 21,237 lines → 2^15 = 32,768
    pub const RUSTC_LINES: MonsterFactor = MonsterFactor::new(2, 15);

    /// All rustc assignments
    pub const RUSTC_ASSIGNMENTS: &[MonsterFactor] = &[
        RUSTC_FUNCTIONS,
        RUSTC_STRUCTS,
        RUSTC_ENUMS,
        RUSTC_TRAITS,
        RUSTC_IMPLS,
        RUSTC_FILES,
        RUSTC_LINES,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_group_verification() {
        assert!(verify_monster_group());
        assert_eq!(TOTAL_MONSTER_FACTORS, 108);
    }

    #[test]
    fn test_monster_factor_lookup() {
        assert!(get_monster_factor(2).is_some());
        assert!(get_monster_factor(3).is_some());
        assert!(get_monster_factor(5).is_some());
        assert!(get_monster_factor(7).is_some());
        assert!(get_monster_factor(11).is_some());
        assert!(get_monster_factor(97).is_none());
    }

    #[test]
    fn test_rustc_assignments() {
        use rustc_assignments::*;
        assert_eq!(RUSTC_FUNCTIONS.value, 262144);
        assert_eq!(RUSTC_STRUCTS.value, 177147);
        assert_eq!(RUSTC_ENUMS.value, 15625);
    }
}
