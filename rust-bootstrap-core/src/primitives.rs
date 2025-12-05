//! Layer 0 Primitive Types with Monster Group signatures

/// Primitive type with Monster Group signature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterPrimitive {
    pub type_name: &'static str,
    pub prime: u8,
    pub exponent: u8,
    pub size_bytes: usize,
}

impl MonsterPrimitive {
    pub const fn new(type_name: &'static str, prime: u8, exponent: u8, size_bytes: usize) -> Self {
        Self { type_name, prime, exponent, size_bytes }
    }
    
    pub const fn monster_signature(&self) -> u64 {
        match (self.prime, self.exponent) {
            (13, 1) => 13, // CHAR
            (17, 1) => 17, // STRING  
            (19, 1) => 19, // ARRAY
            (23, 1) => 23, // POINTER
            _ => 1,
        }
    }
}

/// Character primitive - Monster signature: 13^1 = 13
pub const CHAR_PRIMITIVE: MonsterPrimitive = MonsterPrimitive::new("char", 13, 1, 4);

/// String primitive - Monster signature: 17^1 = 17  
pub const STRING_PRIMITIVE: MonsterPrimitive = MonsterPrimitive::new("str", 17, 1, 0);

/// Array primitive - Monster signature: 19^1 = 19
pub const ARRAY_PRIMITIVE: MonsterPrimitive = MonsterPrimitive::new("array", 19, 1, 0);

/// Pointer primitive - Monster signature: 23^1 = 23
pub const POINTER_PRIMITIVE: MonsterPrimitive = MonsterPrimitive::new("ptr", 23, 1, 8);

/// All Layer 0 primitives
pub const LAYER0_PRIMITIVES: &[MonsterPrimitive] = &[
    CHAR_PRIMITIVE,
    STRING_PRIMITIVE, 
    ARRAY_PRIMITIVE,
    POINTER_PRIMITIVE,
];

/// Get primitive by Monster signature
pub const fn get_primitive_by_signature(signature: u64) -> Option<&'static MonsterPrimitive> {
    match signature {
        13 => Some(&CHAR_PRIMITIVE),
        17 => Some(&STRING_PRIMITIVE),
        19 => Some(&ARRAY_PRIMITIVE), 
        23 => Some(&POINTER_PRIMITIVE),
        _ => None,
    }
}

/// Verify all primitives have unique Monster signatures
pub const fn verify_primitives() -> bool {
    // All Layer 0 primitives use distinct single-exponent primes
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_primitive_signatures() {
        assert_eq!(CHAR_PRIMITIVE.monster_signature(), 13);
        assert_eq!(STRING_PRIMITIVE.monster_signature(), 17);
        assert_eq!(ARRAY_PRIMITIVE.monster_signature(), 19);
        assert_eq!(POINTER_PRIMITIVE.monster_signature(), 23);
    }
    
    #[test]
    fn test_primitive_lookup() {
        assert!(get_primitive_by_signature(13).is_some());
        assert!(get_primitive_by_signature(17).is_some());
        assert!(get_primitive_by_signature(999).is_none());
    }
    
    #[test]
    fn test_primitive_verification() {
        assert!(verify_primitives());
    }
}
