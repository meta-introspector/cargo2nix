// crates/monster_traits/src/type_lattice.rs

use monster_multivector::MonsterMultivector;
use std::collections::BTreeMap;
use std::any::TypeId;
use crate::nested_enums::NestedEnum; // Needed for the TypeLatticeRegistry test

// Import all MReason traits for dynamic access (conceptual, would use macro for full list)
use crate::{
    MReason1, MReason2, MReason3, MReason4, MReason5, MReason6, MReason7, MReason8,
    MReason9, MReason10, MReason11, MReason12, MReason13, MReason14, MReason15, MReason16,
    MReason17, MReason18, MReason19, MReason20, MReason21, MReason22, MReason23, MReason24,
    MReason25, MReason26, MReason27, MReason28, MReason29, MReason30, MReason31, MReason32,
    MReason33, MReason34, MReason35, MReason36, MReason37, MReason38, MReason39, MReason40,
    MReason41, MReason42, MReason43, MReason44, MReason45, MReason46,
    MReason47, MReason48, MReason49, MReason50, MReason51, MReason52, MReason53, MReason54,
    MReason55, MReason56, MReason57, MReason58, MReason59, MReason60, MReason61, MReason62,
    MReason63, MReason64, MReason65, MReason66,
    MReason67, MReason68, MReason69, MReason70, MReason71, MReason72, MReason73, MReason74,
    MReason75, MReason76, MReason77, MReason78, MReason79,
    MReason109, // The sixth septenary factor
};

// Import helper macros for trait bounds
use crate::{
    impl_all_mreasons_for_type_bitvector_46,
    impl_all_mreasons_for_type_triples_20,
    impl_all_mreasons_for_type_groups_of_7_6,
};


/// A registry to hold instances of types and their extracted Monster Reason properties.
pub struct TypeLatticeRegistry {
    /// Stores the results of `extract_numerical_property` for each registered type
    /// and for each applicable MReasonN trait.
    /// Key: TypeId of the registered type.
    /// Value: A BTreeMap where Key: Reason Number (1-109), Value: Extracted Numerical Property (u32).
    registry: BTreeMap<TypeId, BTreeMap<u32, u32>>,
    /// Stores the canonical MonsterMultivector for reference.
    _monster_multivector: MonsterMultivector,
}

impl TypeLatticeRegistry {
    /// Creates a new, empty `TypeLatticeRegistry`.
    pub fn new() -> Self {
        TypeLatticeRegistry {
            registry: BTreeMap::new(),
            _monster_multivector: MonsterMultivector::canonical(),
        }
    }

    /// Registers a u32 type and its extracted numerical properties for implemented reasons.
    pub fn register_type_u32(&mut self, instance: &u32)
    where
        u32: crate::MReason1 + crate::MReason47 + crate::MReason67,
    {
        let type_id = TypeId::of::<u32>();
        let mut properties = BTreeMap::new();

        properties.insert(1, <u32 as crate::MReason1>::extract_numerical_property(instance));
        properties.insert(47, <u32 as crate::MReason47>::extract_numerical_property(instance));
        properties.insert(67, <u32 as crate::MReason67>::extract_numerical_property(instance));

        self.registry.insert(type_id, properties);
    }

    /// Registers a NestedEnum type and its extracted numerical properties for implemented reasons.
    pub fn register_type_nested_enum(&mut self, instance: &crate::nested_enums::NestedEnum)
    where
        crate::nested_enums::NestedEnum: crate::MReason1 + crate::MReason47 + crate::MReason67 +
                                        crate::MReason26 + crate::MReason27 + crate::MReason9, // Added MReason9
    {
        let type_id = TypeId::of::<crate::nested_enums::NestedEnum>();
        let mut properties = BTreeMap::new();

        properties.insert(1, <crate::nested_enums::NestedEnum as crate::MReason1>::extract_numerical_property(instance));
        properties.insert(47, <crate::nested_enums::NestedEnum as crate::MReason47>::extract_numerical_property(instance));
        properties.insert(67, <crate::nested_enums::NestedEnum as crate::MReason67>::extract_numerical_property(instance));
        properties.insert(26, <crate::nested_enums::NestedEnum as crate::MReason26>::extract_numerical_property(instance));
        properties.insert(27, <crate::nested_enums::NestedEnum as crate::MReason27>::extract_numerical_property(instance));
        properties.insert(9, <crate::nested_enums::NestedEnum as crate::MReason9>::extract_numerical_property(instance)); // Added MReason9

        self.registry.insert(type_id, properties);
    }

    /// Retrieves the extracted properties for a registered type.
    pub fn get_properties<T>(&self) -> Option<&BTreeMap<u32, u32>>
    where
        T: Sized + 'static,
    {
        self.registry.get(&TypeId::of::<T>())
    }

    /// Extracts the 46-component Monster bitvector for a given instance.
    /// Each component represents the numerical property extracted by MReasonN.
    pub fn get_monster_bitvector_46<T>(instance: &T) -> Vec<u8>
    where
        T: Sized + 'static + MReason1 + MReason2 + MReason3 + MReason4 + MReason5 + MReason6 + MReason7 + MReason8 +
               MReason9 + MReason10 + MReason11 + MReason12 + MReason13 + MReason14 + MReason15 + MReason16 +
               MReason17 + MReason18 + MReason19 + MReason20 + MReason21 + MReason22 + MReason23 + MReason24 +
               MReason25 + MReason26 + MReason27 + MReason28 + MReason29 + MReason30 + MReason31 + MReason32 +
               MReason33 + MReason34 + MReason35 + MReason36 + MReason37 + MReason38 + MReason39 + MReason40 +
               MReason41 + MReason42 + MReason43 + MReason44 + MReason45 + MReason46
    {
        let mut bitvector = Vec::with_capacity(46);
        for i in 1..=46 {
            let value = match i {
                1 => <T as MReason1>::extract_numerical_property(instance),
                2 => <T as MReason2>::extract_numerical_property(instance),
                3 => <T as MReason3>::extract_numerical_property(instance),
                4 => <T as MReason4>::extract_numerical_property(instance),
                5 => <T as MReason5>::extract_numerical_property(instance),
                6 => <T as MReason6>::extract_numerical_property(instance),
                7 => <T as MReason7>::extract_numerical_property(instance),
                8 => <T as MReason8>::extract_numerical_property(instance),
                9 => <T as MReason9>::extract_numerical_property(instance),
                10 => <T as MReason10>::extract_numerical_property(instance),
                11 => <T as MReason11>::extract_numerical_property(instance),
                12 => <T as MReason12>::extract_numerical_property(instance),
                13 => <T as MReason13>::extract_numerical_property(instance),
                14 => <T as MReason14>::extract_numerical_property(instance),
                15 => <T as MReason15>::extract_numerical_property(instance),
                16 => <T as MReason16>::extract_numerical_property(instance),
                17 => <T as MReason17>::extract_numerical_property(instance),
                18 => <T as MReason18>::extract_numerical_property(instance),
                19 => <T as MReason19>::extract_numerical_property(instance),
                20 => <T as MReason20>::extract_numerical_property(instance),
                21 => <T as MReason21>::extract_numerical_property(instance),
                22 => <T as MReason22>::extract_numerical_property(instance),
                23 => <T as MReason23>::extract_numerical_property(instance),
                24 => <T as MReason24>::extract_numerical_property(instance),
                25 => <T as MReason25>::extract_numerical_property(instance),
                26 => <T as MReason26>::extract_numerical_property(instance),
                27 => <T as MReason27>::extract_numerical_property(instance),
                28 => <T as MReason28>::extract_numerical_property(instance),
                29 => <T as MReason29>::extract_numerical_property(instance),
                30 => <T as MReason30>::extract_numerical_property(instance),
                31 => <T as MReason31>::extract_numerical_property(instance),
                32 => <T as MReason32>::extract_numerical_property(instance),
                33 => <T as MReason33>::extract_numerical_property(instance),
                34 => <T as MReason34>::extract_numerical_property(instance),
                35 => <T as MReason35>::extract_numerical_property(instance),
                36 => <T as MReason36>::extract_numerical_property(instance),
                37 => <T as MReason37>::extract_numerical_property(instance),
                38 => <T as MReason38>::extract_numerical_property(instance),
                39 => <T as MReason39>::extract_numerical_property(instance),
                40 => <T as MReason40>::extract_numerical_property(instance),
                41 => <T as MReason41>::extract_numerical_property(instance),
                42 => <T as MReason42>::extract_numerical_property(instance),
                43 => <T as MReason43>::extract_numerical_property(instance),
                44 => <T as MReason44>::extract_numerical_property(instance),
                45 => <T as MReason45>::extract_numerical_property(instance),
                46 => <T as MReason46>::extract_numerical_property(instance),
                _ => 0, // Should not happen given the loop range
            };
            bitvector.push(value as u8);
        }
        bitvector
    }

    /// Extracts the 20-component Monster triples data for a given instance.
    /// Each component represents the numerical property extracted by MReasonN.
    /// These 20 components conceptually form the "20 triples" mentioned in the document.
    pub fn get_monster_triples_20<T>(instance: &T) -> Vec<u32>
    where
        T: Sized + 'static + MReason47 + MReason48 + MReason49 + MReason50 + MReason51 + MReason52 + MReason53 + MReason54 +
               MReason55 + MReason56 + MReason57 + MReason58 + MReason59 + MReason60 + MReason61 + MReason62 +
               MReason63 + MReason64 + MReason65 + MReason66
    {
        let mut triples_data = Vec::with_capacity(20);
        for i in 47..=66 {
            let value = match i {
                47 => <T as MReason47>::extract_numerical_property(instance),
                48 => <T as MReason48>::extract_numerical_property(instance),
                49 => <T as MReason49>::extract_numerical_property(instance),
                50 => <T as MReason50>::extract_numerical_property(instance),
                51 => <T as MReason51>::extract_numerical_property(instance),
                52 => <T as MReason52>::extract_numerical_property(instance),
                53 => <T as MReason53>::extract_numerical_property(instance),
                54 => <T as MReason54>::extract_numerical_property(instance),
                55 => <T as MReason55>::extract_numerical_property(instance),
                56 => <T as MReason56>::extract_numerical_property(instance),
                57 => <T as MReason57>::extract_numerical_property(instance),
                58 => <T as MReason58>::extract_numerical_property(instance),
                59 => <T as MReason59>::extract_numerical_property(instance),
                60 => <T as MReason60>::extract_numerical_property(instance),
                61 => <T as MReason61>::extract_numerical_property(instance),
                62 => <T as MReason62>::extract_numerical_property(instance),
                63 => <T as MReason63>::extract_numerical_property(instance),
                64 => <T as MReason64>::extract_numerical_property(instance),
                65 => <T as MReason65>::extract_numerical_property(instance),
                66 => <T as MReason66>::extract_numerical_property(instance),
                _ => 0, // Should not happen
            };
            triples_data.push(value);
        }
        triples_data
    }

    /// Extracts the 6-component Monster groups of 7 data for a given instance.
    /// Each component represents the numerical property extracted by MReasonN.
    pub fn get_monster_groups_of_7_6<T>(instance: &T) -> Vec<u32>
    where
        T: Sized + 'static + MReason75 + MReason76 + MReason77 + MReason78 + MReason79 + MReason109
    {
        let mut groups_data = Vec::with_capacity(6);
        for i in [75, 76, 77, 78, 79, 109].iter() { // Explicitly list reasons for prime 7 related factors
            let value = match i {
                75 => <T as MReason75>::extract_numerical_property(instance),
                76 => <T as MReason76>::extract_numerical_property(instance),
                77 => <T as MReason77>::extract_numerical_property(instance),
                78 => <T as MReason78>::extract_numerical_property(instance),
                79 => <T as MReason79>::extract_numerical_property(instance),
                109 => <T as MReason109>::extract_numerical_property(instance),
                _ => 0, // Should not happen
            };
            groups_data.push(value);
        }
        groups_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nested_enums::NestedEnum;
    use crate::{MReason1, MReason2, MReason3, MReason4, MReason5, MReason6, MReason7, MReason8,
        MReason9, MReason10, MReason11, MReason12, MReason13, MReason14, MReason15, MReason16,
        MReason17, MReason18, MReason19, MReason20, MReason21, MReason22, MReason23, MReason24,
        MReason25, MReason26, MReason27, MReason28, MReason29, MReason30, MReason31, MReason32,
        MReason33, MReason34, MReason35, MReason36, MReason37, MReason38, MReason39, MReason40,
        MReason41, MReason42, MReason43, MReason44, MReason45, MReason46,
        MReason47, MReason48, MReason49, MReason50, MReason51, MReason52, MReason53, MReason54,
        MReason55, MReason56, MReason57, MReason58, MReason59, MReason60, MReason61, MReason62,
        MReason63, MReason64, MReason65, MReason66,
        MReason75, MReason76, MReason77, MReason78, MReason79, MReason109,
    };


    // Dummy implementations for MReasonN for u32 to make tests compile
    // In a full system, these would have specific logic.
    macro_rules! impl_dummy_mreason_for_u32 {
        ( $( $trait_name:ident ),* ) => {
            $(
                impl $trait_name for u32 {
                    fn extract_numerical_property(&self) -> u32 {
                        // Default to 0 for unimplemented reasons
                        0
                    }
                }
            )*
        };
    }

    impl_dummy_mreason_for_u32!(
        MReason2, MReason3, MReason4, MReason5, MReason6, MReason7, MReason8,
        MReason10, MReason11, MReason12, MReason13, MReason14, MReason15, MReason16,
        MReason17, MReason18, MReason19, MReason20, MReason21, MReason22, MReason23, MReason24,
        MReason25, MReason28, MReason29, MReason30, MReason31, MReason32,
        MReason33, MReason34, MReason35, MReason36, MReason37, MReason38, MReason39, MReason40,
        MReason41, MReason42, MReason43, MReason44, MReason45, MReason46,
        MReason48, MReason49, MReason50, MReason51, MReason52, MReason53, MReason54,
        MReason55, MReason56, MReason57, MReason58, MReason59, MReason60, MReason61, MReason62,
        MReason63, MReason64, MReason65, MReason66,
        MReason75, MReason76, MReason77, MReason78, MReason79, MReason109
    );

    // Dummy implementations for MReasonN for NestedEnum to make tests compile
    macro_rules! impl_dummy_mreason_for_nested_enum {
        ( $( $trait_name:ident ),* ) => {
            $(
                impl $trait_name for NestedEnum {
                    fn extract_numerical_property(&self) -> u32 {
                        // Default to 0 for unimplemented reasons
                        0
                    }
                }
            )*
        };
    }

    impl_dummy_mreason_for_nested_enum!(
        MReason2, MReason3, MReason4, MReason5, MReason6, MReason7, MReason8,
        MReason10, MReason11, MReason12, MReason13, MReason14, MReason15, MReason16,
        MReason17, MReason18, MReason19, MReason20, MReason21, MReason22, MReason23, MReason24,
        MReason25, MReason28, MReason29, MReason30, MReason31, MReason32,
        MReason33, MReason34, MReason35, MReason36, MReason37, MReason38, MReason39, MReason40,
        MReason41, MReason42, MReason43, MReason44, MReason45, MReason46,
        MReason48, MReason49, MReason50, MReason51, MReason52, MReason53, MReason54,
        MReason55, MReason56, MReason57, MReason58, MReason59, MReason60, MReason61, MReason62,
        MReason63, MReason64, MReason65, MReason66,
        MReason68, MReason69, MReason70, MReason71, MReason72, MReason73, MReason74,
        MReason75, MReason76, MReason77, MReason78, MReason79, MReason109
    );

    #[test]
    fn test_type_lattice_registry() {
        let mut registry = TypeLatticeRegistry::new();

        // Test with u32
        let num_instance: u32 = 6;
        registry.register_type_u32(&num_instance);
        let u32_props = registry.get_properties::<u32>().expect("u32 properties not found");

        assert_eq!(u32_props.get(&1), Some(&1)); // 6 is divisible by 2, so should be 1
        assert_eq!(u32_props.get(&47), Some(&1)); // 6 is divisible by 3, so should be 1
        assert_eq!(u32_props.get(&67), Some(&0)); // 6 is not divisible by 5, so should be 0

        // Test with NestedEnum
        let enum_instance = NestedEnum::Root1(crate::nested_enums::Depth2Enum::Unit2); // Use a valid variant
        registry.register_type_nested_enum(&enum_instance);
        let enum_props = registry.get_properties::<NestedEnum>().expect("NestedEnum properties not found");

        assert_eq!(enum_props.get(&1), Some(&1)); // NestedEnum MAX_DEPTH 8 is divisible by 2, so should be 1
        assert_eq!(enum_props.get(&47), Some(&0)); // NestedEnum MAX_DEPTH 8 is not divisible by 3, so should be 0
        assert_eq!(enum_props.get(&67), Some(&0)); // NestedEnum MAX_DEPTH 8 is not divisible by 5, so should be 0
        assert_eq!(enum_props.get(&26), Some(&NestedEnum::MAX_DEPTH)); // MReason26 extracts MAX_DEPTH (8)
        assert_eq!(enum_props.get(&27), Some(&NestedEnum::NUM_DIRECT_VARIANTS)); // MReason27 extracts NUM_DIRECT_VARIANTS (3)
        assert_eq!(enum_props.get(&9), Some(&1)); // MReason9 extracts the property of 1st variant (Root2), which is 1
    }

    // New tests for bitvector and triples representation
    #[test]
    fn test_get_monster_bitvector_46_for_u32() {
        // For u32=6, MReason1 is 1, others are 0 (as not implemented)
        let num_instance: u32 = 6;
        let bitvector = TypeLatticeRegistry::get_monster_bitvector_46(&num_instance);
        assert_eq!(bitvector.len(), 46);
        assert_eq!(bitvector[0], 1); // MReason1 for 6 (even) is 1
        assert_eq!(bitvector[1], 0); // MReason2 is a dummy impl for u32, returns 0
        assert_eq!(bitvector[8], 0); // MReason9 is a dummy impl for u32, returns 0
        assert_eq!(bitvector[25], 0); // MReason26 is a dummy impl for u32, returns 0
        assert_eq!(bitvector[26], 0); // MReason27 is a dummy impl for u32, returns 0
        assert_eq!(bitvector[45], 0); // MReason46 is a dummy impl for u32, returns 0
    }

    #[test]
    fn test_get_monster_triples_20_for_u32() {
        // For u32=6, MReason47 is 1, others are 0 (as dummy implemented)
        let num_instance: u32 = 6;
        let triples_data = TypeLatticeRegistry::get_monster_triples_20(&num_instance);
        assert_eq!(triples_data.len(), 20);
        assert_eq!(triples_data[0], 1); // MReason47 for 6 (divisible by 3) is 1
        assert_eq!(triples_data[1], 0); // MReason48 is dummy
        assert_eq!(triples_data[19], 0); // MReason66 is dummy
    }

    #[test]
    fn test_get_monster_groups_of_7_6_for_u32() {
        // For u32=6, all septenary factors are 0 (6 is not divisible by 7, or dummy impl)
        let num_instance: u32 = 6;
        let groups_data = TypeLatticeRegistry::get_monster_groups_of_7_6(&num_instance);
        assert_eq!(groups_data.len(), 6);
        assert_eq!(groups_data[0], 0); // MReason75 is dummy
        assert_eq!(groups_data[5], 0); // MReason109 is dummy
    }

    #[test]
    fn test_get_monster_bitvector_46_for_nested_enum() {
        let instance = NestedEnum::Root2;
        let bitvector = TypeLatticeRegistry::get_monster_bitvector_46(&instance);
        assert_eq!(bitvector.len(), 46);
        assert_eq!(bitvector[0], 1); // MReason1 for NestedEnum is 1
        assert_eq!(bitvector[8], 1); // MReason9 for NestedEnum is 1
        assert_eq!(bitvector[25], 8); // MReason26 for NestedEnum is 8
        assert_eq!(bitvector[26], 3); // MReason27 for NestedEnum is 3
    }

    #[test]
    fn test_get_monster_triples_20_for_nested_enum() {
        let instance = NestedEnum::Root2;
        let triples_data = TypeLatticeRegistry::get_monster_triples_20(&instance);
        assert_eq!(triples_data.len(), 20);
        assert_eq!(triples_data[0], 0); // MReason47 for NestedEnum is 0
    }

    #[test]
    fn test_get_monster_groups_of_7_6_for_nested_enum() {
        let instance = NestedEnum::Root2;
        let groups_data = TypeLatticeRegistry::get_monster_groups_of_7_6(&instance);
        assert_eq!(groups_data.len(), 6);
        assert_eq!(groups_data[0], 0); // MReason75
        assert_eq!(groups_data[5], 0); // MReason109
    }
}