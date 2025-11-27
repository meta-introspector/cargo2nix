// crates/monster_multivector/src/monster_constants.rs

// The order of the Monster Group (M) as a string, since it exceeds u128.
pub const MONSTER_GROUP_ORDER_STR: &str = "808017424794512875886459904961710757005754368000000000";

// The 108 prime factors, including those from the Monster Group's order and
// the additional "Unique High Primes" from the "108 Supersingular Reasons" protocol.
// This list is ordered by prime value for consistency.
pub const CANONICAL_MONSTER_FACTORS: &[u32] = &[
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 46 x 2
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 20 x 3
    5, 5, 5, 5, 5, 5, 5, 5, // 8 x 5
    7, 7, 7, 7, 7, 7, // 6 x 7
    11, 11, 11, // 3 x 11 (Note: Monster's order has 11^2, but the 108 factors list has 3x11 for #80-82)
    13, 13, 13, // 3 x 13
    17, 17, // 2 x 17
    19, 19, // 2 x 19
    23, 23, // 2 x 23
    29, // 1 x 29
    31, // 1 x 31
    41, // 1 x 41
    43, // 1 x 43
    47, // 1 x 47 (Unique High Prime)
    53, // 1 x 53 (Unique High Prime)
    59, // 1 x 59 (Unique High Prime)
    71, // 1 x 71 (Unique High Prime)
    73, // 1 x 73 (Unique High Prime)
    79, // 1 x 79 (Unique High Prime)
    83, // 1 x 83 (Unique High Prime)
    89, // 1 x 89 (Unique High Prime)
    97, // 1 x 97 (Unique High Prime)
    107, // 1 x 107 (Unique High Prime)
    113, // 1 x 113 (Unique High Prime)
    127, // 1 x 127 (Unique High Prime)
    523, // 1 x 523 (Unique High Prime - The Final Prime)
];
