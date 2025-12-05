// crates/monster_traits/src/implementations.rs

use crate::{MReason1, MReason9, MReason26, MReason27, MReason47, MReason67, EnumIndexable, TypeConverter};
use crate::number_properties; // Import the helper functions
use crate::nested_enums::{NestedEnum, Depth2Enum, Depth3Enum, Depth4Enum}; // Import needed enums for deeper paths

// --- Implementations for u32 ---

impl MReason1 for u32 {
    // Reason 1: Binary Duality. Prime Factor: 2.
    // Extracts 1 if the number is even, 0 if odd.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(*self, <Self as MReason1>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

impl MReason47 for u32 {
    // Reason 47: Triality Principle. Prime Factor: 3.
    // Extracts 1 if the number is divisible by 3, 0 otherwise.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(*self, <Self as MReason47>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

impl MReason67 for u32 {
    // Reason 67: The j-invariant Prime. Prime Factor: 5.
    // Extracts 1 if the number is divisible by 5, 0 otherwise.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(*self, <Self as MReason67>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

// --- Implementations for NestedEnum (which is Depth1Enum) ---

impl MReason1 for NestedEnum {
    // Reason 1: Binary Duality. Prime Factor: 2.
    // Extracts 1 if the MAX_DEPTH (8) is divisible by 2, 0 otherwise.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(Self::MAX_DEPTH, <Self as MReason1>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

impl MReason47 for NestedEnum {
    // Reason 47: Triality Principle. Prime Factor: 3.
    // Extracts 1 if the MAX_DEPTH (8) is divisible by 3, 0 otherwise.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(Self::MAX_DEPTH, <Self as MReason47>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

impl MReason67 for NestedEnum {
    // Reason 67: The j-invariant Prime. Prime Factor: 5.
    // Extracts 1 if the MAX_DEPTH (8) is divisible by 5, 0 otherwise.
    fn extract_numerical_property(&self) -> u32 {
        if number_properties::has_factor(Self::MAX_DEPTH, <Self as MReason67>::PRIME_FACTOR) {
            1
        } else {
            0
        }
    }
}

impl MReason26 for NestedEnum {
    // Reason 26: Complexity Weight. Prime Factor: 2.
    // Extracts the MAX_DEPTH of the enum.
    fn extract_numerical_property(&self) -> u32 {
        Self::MAX_DEPTH
    }
}

impl MReason27 for NestedEnum {
    // Reason 27: Level Setting. Prime Factor: 2.
    // Extracts the NUM_DIRECT_VARIANTS of the top-level enum.
    fn extract_numerical_property(&self) -> u32 {
        Self::NUM_DIRECT_VARIANTS
    }
}

impl MReason9 for NestedEnum {
    // Reason 9: Highest Weight Vectors. Prime Factor: 2.
    // Extracts the numerical property of the (PRIME_FACTOR - 1)th variant.
    // For MReason9, PRIME_FACTOR is 2, so it extracts the property of the 1st (0-indexed) variant.
    fn extract_numerical_property(&self) -> u32 {
        <Self as EnumIndexable>::get_numerical_property_of_nth_variant(<Self as MReason9>::PRIME_FACTOR - 1)
            .unwrap_or(0) // Default to 0 if the variant property is None
    }
}

// --- Implementation of EnumIndexable for NestedEnum ---
impl EnumIndexable for NestedEnum {
    const NUM_VARIANTS: u32 = Self::NUM_DIRECT_VARIANTS; // Total number of direct variants (3)

    fn get_numerical_property_of_nth_variant(n_index: u32) -> Option<u32> {
        match n_index {
            0 => Some(Depth2Enum::MAX_DEPTH), // Root1(Depth2Enum) -> return MAX_DEPTH of Depth2Enum (7)
            1 => Some(1), // Root2 (primitive-like at this level)
            2 => Some(Depth2Enum::MAX_DEPTH), // Root3(Depth2Enum) -> return MAX_DEPTH of Depth2Enum (7)
            _ => None, // Index out of bounds
        }
    }
}

// --- Implementation of TypeConverter for u32 ---
impl TypeConverter<u32> for u32 {
    fn to_8d_nested_enum_representation(value: &u32) -> NestedEnum {
        // Simple mapping:
        // Even numbers map to a variant that represents some "binary duality" or "on/off" state.
        // Odd numbers map to a variant that represents an "individual" or "singular" state.
        // This is conceptual and can be refined later.

        if value % 2 == 0 {
            // Map even numbers to a root variant that typically holds a nested structure.
            // Using a specific variant for demonstration.
            NestedEnum::Root1(Depth2Enum::Unit2)
        } else {
            // Map odd numbers to a simpler root variant.
            NestedEnum::Root2
        }
    }

    fn to_numerical_hash(value: &u32) -> u64 {
        // For u32, a simple and direct numerical hash is its value itself.
        *value as u64
    }
}

// --- Implementation of TypeConverter for String ---
impl TypeConverter<String> for String {
    fn to_8d_nested_enum_representation(value: &String) -> NestedEnum {
        // Map string based on its length parity.
        if value.len() % 2 == 0 {
            // Even length maps to a deeper path.
            // Example: Root1 -> Unit1 -> Tier1 -> Block2 (Depth 5)
            NestedEnum::Root1(Depth2Enum::Unit1(Depth3Enum::Tier1(Depth4Enum::Block2)))
        } else {
            // Odd length maps to a simpler path.
            // Example: Root3 -> Unit2
            NestedEnum::Root3(Depth2Enum::Unit2)
        }
    }

    fn to_numerical_hash(value: &String) -> u64 {
        // Return the length of the string as its numerical hash.
        value.len() as u64
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::nested_enums::NestedEnum;

    #[test]
    fn test_mreason1_for_u32() {
        // Prime factor for MReason1 is 2
        assert_eq!(<u32 as MReason1>::extract_numerical_property(&2u32), 1); // 2 is divisible by 2
        assert_eq!(<u32 as MReason1>::extract_numerical_property(&4u32), 1); // 4 is divisible by 2
        assert_eq!(<u32 as MReason1>::extract_numerical_property(&1u32), 0); // 1 is not divisible by 2
        assert_eq!(<u32 as MReason1>::extract_numerical_property(&3u32), 0); // 3 is not divisible by 2
    }

    #[test]
    fn test_mreason47_for_u32() {
        // Prime factor for MReason47 is 3
        assert_eq!(<u32 as MReason47>::extract_numerical_property(&3u32), 1); // 3 is divisible by 3
        assert_eq!(<u32 as MReason47>::extract_numerical_property(&6u32), 1); // 6 is divisible by 3
        assert_eq!(<u32 as MReason47>::extract_numerical_property(&1u32), 0); // 1 is not divisible by 3
        assert_eq!(<u32 as MReason47>::extract_numerical_property(&2u32), 0); // 2 is not divisible by 3
    }

    #[test]
    fn test_mreason67_for_u32() {
        // Prime factor for MReason67 is 5
        assert_eq!(<u32 as MReason67>::extract_numerical_property(&5u32), 1); // 5 is divisible by 5
        assert_eq!(<u32 as MReason67>::extract_numerical_property(&10u32), 1); // 10 is divisible by 5
        assert_eq!(<u32 as MReason67>::extract_numerical_property(&1u32), 0); // 1 is not divisible by 5
        assert_eq!(<u32 as MReason67>::extract_numerical_property(&4u32), 0); // 4 is not divisible by 5
    }

    #[test]
    fn test_mreason1_for_nested_enum_depth() {
        // NestedEnum (Depth1Enum) has MAX_DEPTH = 8. Prime factor for MReason1 is 2.
        // 8 is divisible by 2.
        let instance = NestedEnum::Root2; // Variant doesn't matter for MAX_DEPTH
        assert_eq!(<NestedEnum as MReason1>::extract_numerical_property(&instance), 1);
    }

    #[test]
    fn test_mreason47_for_nested_enum_depth() {
        // NestedEnum (Depth1Enum) has MAX_DEPTH = 8. Prime factor for MReason47 is 3.
        // 8 is not divisible by 3.
        let instance = NestedEnum::Root2;
        assert_eq!(<NestedEnum as MReason47>::extract_numerical_property(&instance), 0);
    }

    #[test]
    fn test_mreason67_for_nested_enum_depth() {
        // NestedEnum (Depth1Enum) has MAX_DEPTH = 8. Prime factor for MReason67 is 5.
        // 8 is not divisible by 5.
        let instance = NestedEnum::Root2;
        assert_eq!(<NestedEnum as MReason67>::extract_numerical_property(&instance), 0);
    }

    #[test]
    fn test_mreason26_for_nested_enum_depth() {
        // MReason26 extracts MAX_DEPTH (8)
        let instance = NestedEnum::Root1(crate::nested_enums::Depth2Enum::Unit1(crate::nested_enums::Depth3Enum::Tier1(crate::nested_enums::Depth4Enum::Block1(crate::nested_enums::Depth5Enum::Layer1(crate::nested_enums::Depth6Enum::Segment1(crate::nested_enums::Depth7Enum::Branch1(crate::nested_enums::Depth8Enum::Node1)))))));
        assert_eq!(<NestedEnum as MReason26>::extract_numerical_property(&instance), NestedEnum::MAX_DEPTH); // Should be 8
    }

    #[test]
    fn test_mreason27_for_nested_enum_variants() {
        // MReason27 extracts NUM_DIRECT_VARIANTS (3)
        let instance = NestedEnum::Root2;
        assert_eq!(<NestedEnum as MReason27>::extract_numerical_property(&instance), NestedEnum::NUM_DIRECT_VARIANTS); // Should be 3
    }

    #[test]
    fn test_enum_indexable_for_nested_enum() {
        // Test NUM_VARIANTS
        assert_eq!(<NestedEnum as EnumIndexable>::NUM_VARIANTS, 3);

        // Test get_numerical_property_of_nth_variant
        assert_eq!(<NestedEnum as EnumIndexable>::get_numerical_property_of_nth_variant(0), Some(crate::nested_enums::Depth2Enum::MAX_DEPTH)); // Root1
        assert_eq!(<NestedEnum as EnumIndexable>::get_numerical_property_of_nth_variant(1), Some(1)); // Root2
        assert_eq!(<NestedEnum as EnumIndexable>::get_numerical_property_of_nth_variant(2), Some(crate::nested_enums::Depth2Enum::MAX_DEPTH)); // Root3
        assert_eq!(<NestedEnum as EnumIndexable>::get_numerical_property_of_nth_variant(3), None); // Out of bounds
    }

    #[test]
    fn test_mreason9_for_nested_enum_nth_variant() {
        // MReason9 extracts the numerical property of the (PRIME_FACTOR - 1)th variant (1st, 0-indexed).
        // For NestedEnum, the 1st variant (Root2) has a numerical property of 1.
        let instance = NestedEnum::Root1(crate::nested_enums::Depth2Enum::Unit2); // Instance doesn't matter for this property
        assert_eq!(<NestedEnum as MReason9>::extract_numerical_property(&instance), 1);
    }

    #[test]
    fn test_type_converter_u32_to_nested_enum() {
        // Test even number
        let even_num: u32 = 4;
        let converted_even = <u32 as TypeConverter<u32>>::to_8d_nested_enum_representation(&even_num);
        assert_eq!(converted_even, NestedEnum::Root1(Depth2Enum::Unit2));

        // Test odd number
        let odd_num: u32 = 7;
        let converted_odd = <u32 as TypeConverter<u32>>::to_8d_nested_enum_representation(&odd_num);
        assert_eq!(converted_odd, NestedEnum::Root2);
    }

    #[test]
    fn test_type_converter_u32_to_numerical_hash() {
        let num: u32 = 123;
        let expected_hash: u64 = 123;
        assert_eq!(<u32 as TypeConverter<u32>>::to_numerical_hash(&num), expected_hash);
    }

    #[test]
    fn test_type_converter_string_to_nested_enum() {
        // Test even length string
        let even_str = String::from("even"); // Length 4 (even)
        let converted_even = <String as TypeConverter<String>>::to_8d_nested_enum_representation(&even_str);
        assert_eq!(converted_even, NestedEnum::Root1(Depth2Enum::Unit1(Depth3Enum::Tier1(Depth4Enum::Block2))));

        // Test odd length string
        let odd_str = String::from("odd"); // Length 3 (odd)
        let converted_odd = <String as TypeConverter<String>>::to_8d_nested_enum_representation(&odd_str);
        assert_eq!(converted_odd, NestedEnum::Root3(Depth2Enum::Unit2));
    }

    #[test]
    fn test_type_converter_string_to_numerical_hash() {
        let test_str = String::from("hello");
        let expected_hash = 5u64; // Length is 5
        assert_eq!(<String as TypeConverter<String>>::to_numerical_hash(&test_str), expected_hash);

        let empty_str = String::from("");
        assert_eq!(<String as TypeConverter<String>>::to_numerical_hash(&empty_str), 0u64);
    }
}