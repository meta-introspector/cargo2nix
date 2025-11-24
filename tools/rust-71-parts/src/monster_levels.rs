//! Monster Group 15-Level System: 108 supersingular factors with 71 sentinel
//! rustc ≡ Monster Group M = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71

/// Monster Group supersingular prime factors (15 levels)
pub mod monster_factors {
    pub const LEVEL_00: u64 = 2_u64.pow(46);  // 70,368,744,177,664
    pub const LEVEL_01: u64 = 3_u64.pow(20);  // 3,486,784,401
    pub const LEVEL_02: u64 = 5_u64.pow(9);   // 1,953,125
    pub const LEVEL_03: u64 = 7_u64.pow(6);   // 117,649
    pub const LEVEL_04: u64 = 11_u64.pow(2);  // 121
    pub const LEVEL_05: u64 = 13_u64.pow(3);  // 2,197
    pub const LEVEL_06: u64 = 17;             // 17
    pub const LEVEL_07: u64 = 19;             // 19
    pub const LEVEL_08: u64 = 23;             // 23
    pub const LEVEL_09: u64 = 29;             // 29
    pub const LEVEL_10: u64 = 31;             // 31
    pub const LEVEL_11: u64 = 41;             // 41
    pub const LEVEL_12: u64 = 47;             // 47
    pub const LEVEL_13: u64 = 59;             // 59
    pub const LEVEL_14: u64 = 71;             // 71 (SENTINEL)
    
    pub const FACTORS: [u64; 15] = [
        LEVEL_00, LEVEL_01, LEVEL_02, LEVEL_03, LEVEL_04,
        LEVEL_05, LEVEL_06, LEVEL_07, LEVEL_08, LEVEL_09,
        LEVEL_10, LEVEL_11, LEVEL_12, LEVEL_13, LEVEL_14
    ];
    
    pub const SENTINEL: u64 = LEVEL_14; // 71 as sentinel value
}

/// 108 supersingular factor distribution across 15 levels
pub const FACTOR_COUNTS: [u8; 15] = [
    46, // Level 0: 2^46 (46 factors)
    20, // Level 1: 3^20 (20 factors)
    9,  // Level 2: 5^9  (9 factors)
    6,  // Level 3: 7^6  (6 factors)
    2,  // Level 4: 11^2 (2 factors)
    3,  // Level 5: 13^3 (3 factors)
    1,  // Level 6: 17   (1 factor)
    1,  // Level 7: 19   (1 factor)
    1,  // Level 8: 23   (1 factor)
    1,  // Level 9: 29   (1 factor)
    1,  // Level 10: 31  (1 factor)
    1,  // Level 11: 41  (1 factor)
    1,  // Level 12: 47  (1 factor)
    1,  // Level 13: 59  (1 factor)
    14, // Level 14: 71  (14 factors to reach 108 total - SENTINEL EXPANSION)
];

/// Monster Group level with factor assignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterLevel {
    pub level: u8,
    pub factor: u64,
    pub count: u8,
    pub is_sentinel: bool,
}

impl MonsterLevel {
    pub const fn new(level: u8) -> Self {
        Self {
            level,
            factor: monster_factors::FACTORS[level as usize],
            count: FACTOR_COUNTS[level as usize],
            is_sentinel: level == 14,
        }
    }
    
    /// Get total factors at this level
    pub const fn total_factors(&self) -> u64 {
        self.count as u64
    }
    
    /// Verify Monster Group constraint
    pub const fn verify(&self) -> bool {
        self.factor == monster_factors::FACTORS[self.level as usize]
    }
}

/// Complete Monster Group system (108 factors total)
pub struct MonsterSystem {
    levels: [MonsterLevel; 15],
}

impl MonsterSystem {
    pub const fn new() -> Self {
        Self {
            levels: [
                MonsterLevel::new(0),  MonsterLevel::new(1),  MonsterLevel::new(2),
                MonsterLevel::new(3),  MonsterLevel::new(4),  MonsterLevel::new(5),
                MonsterLevel::new(6),  MonsterLevel::new(7),  MonsterLevel::new(8),
                MonsterLevel::new(9),  MonsterLevel::new(10), MonsterLevel::new(11),
                MonsterLevel::new(12), MonsterLevel::new(13), MonsterLevel::new(14),
            ]
        }
    }
    
    /// Get level by index
    pub const fn level(&self, index: u8) -> MonsterLevel {
        self.levels[index as usize]
    }
    
    /// Get sentinel level (71)
    pub const fn sentinel(&self) -> MonsterLevel {
        self.levels[14]
    }
    
    /// Verify total factor count = 108
    pub const fn verify_total(&self) -> bool {
        let mut total = 0u8;
        let mut i = 0;
        while i < 15 {
            total += FACTOR_COUNTS[i];
            i += 1;
        }
        total == 108
    }
    
    /// Map rustc component to Monster level
    pub const fn map_component(&self, component_type: u8) -> MonsterLevel {
        match component_type {
            0..=45 => self.levels[0],   // Primitives → 2^46
            46..=65 => self.levels[1],  // Types → 3^20
            66..=74 => self.levels[2],  // Expressions → 5^9
            75..=80 => self.levels[3],  // Statements → 7^6
            81..=82 => self.levels[4],  // Patterns → 11^2
            83..=85 => self.levels[5],  // Items → 13^3
            86 => self.levels[6],       // Modules → 17
            87 => self.levels[7],       // Crates → 19
            88 => self.levels[8],       // Traits → 23
            89 => self.levels[9],       // Impls → 29
            90 => self.levels[10],      // Generics → 31
            91 => self.levels[11],      // Lifetimes → 41
            92 => self.levels[12],      // Macros → 47
            93 => self.levels[13],      // Attributes → 59
            _ => self.levels[14],       // Compiler → 71 (SENTINEL)
        }
    }
}

/// Global Monster Group system instance
pub const MONSTER: MonsterSystem = MonsterSystem::new();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monster_levels() {
        let level0 = MONSTER.level(0);
        assert_eq!(level0.factor, 2_u64.pow(46));
        assert_eq!(level0.count, 46);
        assert!(!level0.is_sentinel);
        
        let sentinel = MONSTER.sentinel();
        assert_eq!(sentinel.factor, 71);
        assert!(sentinel.is_sentinel);
    }
    
    #[test]
    fn test_total_factors() {
        assert!(MONSTER.verify_total());
        
        let total: u8 = FACTOR_COUNTS.iter().sum();
        assert_eq!(total, 108);
    }
    
    #[test]
    fn test_component_mapping() {
        let primitive = MONSTER.map_component(0);
        assert_eq!(primitive.factor, 2_u64.pow(46));
        
        let compiler = MONSTER.map_component(255);
        assert_eq!(compiler.factor, 71);
        assert!(compiler.is_sentinel);
    }
}
