// crates/monster_traits/src/nested_enums.rs

/// A deeply nested enum structure to test trait implementations on complex types.
/// Maximum recursion depth for this initial step is defined as 8.

// Depth 8 Enum (Innermost)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth8Enum {
    Node1,
    Node2,
    Node3,
}

impl Depth8Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 1; // It's the base of recursion
}

// Depth 7 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth7Enum {
    Branch1(Depth8Enum),
    Branch2,
    Branch3(Depth8Enum),
}

impl Depth7Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 2; // Depth7Enum -> Depth8Enum
}

// Depth 6 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth6Enum {
    Segment1(Depth7Enum),
    Segment2,
    Segment3(Depth7Enum),
}

impl Depth6Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 3; // Depth6Enum -> Depth7Enum -> Depth8Enum
}

// Depth 5 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth5Enum {
    Layer1(Depth6Enum),
    Layer2,
    Layer3(Depth6Enum),
}

impl Depth5Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 4; // Depth5Enum -> Depth6Enum -> Depth7Enum -> Depth8Enum
}

// Depth 4 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth4Enum {
    Block1(Depth5Enum),
    Block2,
    Block3(Depth5Enum),
}

impl Depth4Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 5; // Depth4Enum -> Depth5Enum -> ... -> Depth8Enum
}

// Depth 3 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth3Enum {
    Tier1(Depth4Enum),
    Tier2,
    Tier3(Depth4Enum),
}

impl Depth3Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 6; // Depth3Enum -> Depth4Enum -> ... -> Depth8Enum
}

// Depth 2 Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth2Enum {
    Unit1(Depth3Enum),
    Unit2,
    Unit3(Depth3Enum),
}

impl Depth2Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 7; // Depth2Enum -> Depth3Enum -> ... -> Depth8Enum
}

// Depth 1 Enum (Top-level)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth1Enum {
    Root1(Depth2Enum),
    Root2,
    Root3(Depth2Enum),
}

impl Depth1Enum {
    pub const NUM_DIRECT_VARIANTS: u32 = 3;
    pub const MAX_DEPTH: u32 = 8; // Depth1Enum -> Depth2Enum -> ... -> Depth8Enum
}

pub type NestedEnum = Depth1Enum; // Alias for convenience, making Depth1Enum the 'NestedEnum'
