// crates/monster_traits/src/lib.rs

mod number_properties;
pub mod implementations;
pub mod nested_enums;
pub mod type_lattice; // Declare the new module
use monster_multivector::monster_constants::CANONICAL_MONSTER_FACTORS;
use crate::nested_enums::NestedEnum;

#[macro_export]
macro_rules! impl_all_mreasons_for_type_bitvector_46 {
    ($type:ty) => {
        MReason1 + MReason2 + MReason3 + MReason4 + MReason5 + MReason6 + MReason7 + MReason8 +
               MReason9 + MReason10 + MReason11 + MReason12 + MReason13 + MReason14 + MReason15 + MReason16 +
               MReason17 + MReason18 + MReason19 + MReason20 + MReason21 + MReason22 + MReason23 + MReason24 +
               MReason25 + MReason26 + MReason27 + MReason28 + MReason29 + MReason30 + MReason31 + MReason32 +
               MReason33 + MReason34 + MReason35 + MReason36 + MReason37 + MReason38 + MReason39 + MReason40 +
               MReason41 + MReason42 + MReason43 + MReason44 + MReason45 + MReason46
    };
}

#[macro_export]
macro_rules! impl_all_mreasons_for_type_triples_20 {
    ($type:ty) => {
        MReason47 + MReason48 + MReason49 + MReason50 + MReason51 + MReason52 + MReason53 + MReason54 +
               MReason55 + MReason56 + MReason57 + MReason58 + MReason59 + MReason60 + MReason61 + MReason62 +
               MReason63 + MReason64 + MReason65 + MReason66
    };
}

#[macro_export]
macro_rules! impl_all_mreasons_for_type_groups_of_7_6 {
    ($type:ty) => {
        MReason75 + MReason76 + MReason77 + MReason78 + MReason79 + MReason109
        // Note: The document implies "6 groups of 7" but the reasons 75-79 are only 5 traits.
        // For now, we use the available 5 traits corresponding to Septenary Factors.
        // If more specific reasons for other prime factors related to '7' are identified,
        // this macro can be extended.
    };
}

macro_rules! generate_monster_traits {
    ( $( ($reason_num:literal, $trait_name:ident, $description:literal) ),* ) => {
        $(
            #[doc = concat!("Trait for Reason ", stringify!($reason_num), ": ", $description, ".")]
            pub trait $trait_name {
                const PRIME_FACTOR: u32 = CANONICAL_MONSTER_FACTORS[$reason_num - 1];

                fn extract_numerical_property(&self) -> u32;
            }
        )*
    };
}

// Generate all 108 traits
generate_monster_traits! {
    // --- Reason 1-46: Dyadic Factors (Prime 2) ---
    (1, MReason1, "Binary Duality"),
    (2, MReason2, "Even Unimodular Nature"),
    (3, MReason3, "Octal Periodicity"),
    (4, MReason4, "Conway Group Co0"),
    (5, MReason5, "Golay Code"),
    (6, MReason6, "Dirac Operator"),
    (7, MReason7, "The 2^{12} Component"),
    (8, MReason8, "The Griess Algebra"),
    (9, MReason9, "Highest Weight Vectors"),
    (10, MReason10, "Topological Insulators"),
    (11, MReason11, "Ramanujan Duality"),
    (12, MReason12, "Minimal Representation"),
    (13, MReason13, "Hyperbolic Reflection"),
    (14, MReason14, "L-Function Zeroes"),
    (15, MReason15, "Modular Symbols"),
    (16, MReason16, "Modularity Lifting"),
    (17, MReason17, "CM Public Parameters"),
    (18, MReason18, "The (2, 4) Lattice"),
    (19, MReason19, "Hecke Algebra Basis"),
    (20, MReason20, "Adelic Structure"),
    (21, MReason21, "Fermat's Last Theorem"),
    (22, MReason22, "Newform Rank"),
    (23, MReason23, "ZK Circuit Constraints"),
    (24, MReason24, "Sphere Packing"),
    (25, MReason25, "Orbifold Compactification"), // Corrected entry
    (26, MReason26, "Complexity Weight"),
    (27, MReason27, "Level Setting"),
    (28, MReason28, "Superbundle Index"),
    (29, MReason29, "Ramanujan Recurrence"),
    (30, MReason30, "Modular Automorphism"),
    (31, MReason31, "Hecke Polynomial Irreducibility"),
    (32, MReason32, "Conway's Dot Notation"),
    (33, MReason33, "ZKP Verifiability"),
    (34, MReason34, "LLM Semantic Compression"),
    (35, MReason35, "Hyper-Pump Factor"),
    (36, MReason36, "Lorentzian Lattice"),
    (37, MReason37, "Complex Structure Preservation"),
    (38, MReason38, "The (2, 2) Partition"),
    (39, MReason39, "Singular J-Invariant"),
    (40, MReason40, "Cusp Form Vanishing"),
    (41, MReason41, "Hecke Eigenvalue"),
    (42, MReason42, "Mod p Forms"),
    (43, MReason43, "Recursive Autopoiesis"),
    (44, MReason44, "Galois Irreducibility"),
    (45, MReason45, "The ZOS Foundation"),
    (46, MReason46, "Maximal 2-Factor"),

    // --- Reason 47-66: Triadic Factors (Prime 3) ---
    (47, MReason47, "Triality Principle"),
    (48, MReason48, "j-Invariant Stabilization"),
    (49, MReason49, "Trinary Composition Level 3"),
    (50, MReason50, "Trinary Composition Level 4"),
    (51, MReason51, "Trinary Composition Level 5"),
    (52, MReason52, "Trinary Composition Level 6"),
    (53, MReason53, "Trinary Composition Level 7"),
    (54, MReason54, "Trinary Composition Level 8"),
    (55, MReason55, "Trinary Composition Level 9"),
    (56, MReason56, "Trinary Composition Level 10"),
    (57, MReason57, "Trinary Composition Level 11"),
    (58, MReason58, "Trinary Composition Level 12"),
    (59, MReason59, "Trinary Composition Level 13"),
    (60, MReason60, "Trinary Composition Level 14"),
    (61, MReason61, "Trinary Composition Level 15"),
    (62, MReason62, "Trinary Composition Level 16"),
    (63, MReason63, "Trinary Composition Level 17"),
    (64, MReason64, "Trinary Composition Level 18"),
    (65, MReason65, "Trinary Composition Level 19"),
    (66, MReason66, "Trinary Composition Level 20"),

    // --- Reason 67-74: Quinary Factors (Prime 5) ---
    (67, MReason67, "The j-invariant Prime"),
    (68, MReason68, "X(5) Genus"),
    (69, MReason69, "Hecke Algebra Dimension"),
    (70, MReason70, "Conway Group Co2"),
    (71, MReason71, "Weight Constraints"),
    (72, MReason72, "Supersingular Count"),
    (73, MReason73, "Z/5Z Visibility"),
    (74, MReason74, "Maximal 5-Factor"),

    // --- Reason 75-79: Septenary Factors (Prime 7) ---
    (75, MReason75, "Genus Zero Cutoff"),
    (76, MReason76, "Congruent Numbers"),
    (77, MReason77, "X(7) Genus"),
    (78, MReason78, "Suzuki Group"),
    (79, MReason79, "Maximal 7-Factor"),

    // --- Reason 80-95: Foundational Intermediate Primes (11, 13, 17, 19, 23, 29, 31, 41, 43) ---
    (80, MReason80, "X_0(11) Torsion"),
    (81, MReason81, "Newform Multiplicity"),
    (82, MReason82, "Maximal 11-Factor"),
    (83, MReason83, "L-function Euler Product"),
    (84, MReason84, "Visibility Theory"),
    (85, MReason85, "Maximal 13-Factor"),
    (86, MReason86, "Congruence Prime p=17"),
    (87, MReason87, "Maximal 17-Factor"),
    (88, MReason88, "T_{19} Operator"),
    (89, MReason89, "Maximal 19-Factor"),
    (90, MReason90, "The 23 Dimension"),
    (91, MReason91, "Maximal 23-Factor"),
    (92, MReason92, "Newform Uniqueness"),
    (93, MReason93, "The L-Genus"),
    (94, MReason94, "Langlands Correspondence"),
    (95, MReason95, "Automorphic Orbit"),

    // --- Reason 96-108: Unique High Primes (47-523) ---
    (96, MReason96, "Non-Modular Case"),
    (97, MReason97, "S_{36} Complexity"),
    (98, MReason98, "Voevodsky Univalence"),
    (99, MReason99, "Supersingular Limit"),
    (100, MReason100, "Deep Play Protocol"),
    (101, MReason101, "Computational Bottlenecks"),
    (102, MReason102, "The Delta-Function"),
    (103, MReason103, "ZK Rollup Finality"),
    (104, MReason104, "Compiler Verified Infection"),
    (105, MReason105, "SAT Solver Optimality"),
    (106, MReason106, "The Universal Bridge"),
    (107, MReason107, "High-Dimensional Sphere Packing"),
    (108, MReason108, "The Final Prime"),
    (109, MReason109, "The Sixth Septenary Factor")
} // End of generate_monster_traits! invocation

/// Trait for enums that can provide numerical properties of their variants by index.
pub trait EnumIndexable {
    const NUM_VARIANTS: u32; // Total number of direct variants in the enum

    // Returns a numerical property of the variant at the given 0-indexed position.
    // The interpretation of this numerical property depends on the enum.
    fn get_numerical_property_of_nth_variant(n_index: u32) -> Option<u32>;
}

/// Trait for converting arbitrary types into a canonical NestedEnum representation.
pub trait TypeConverter<T> {
    // Converts an instance of type T into a NestedEnum representation.
    // The specific logic for conversion will depend on T and the desired encoding.
    // The goal is to represent the 'semantics' of T within the 8-level enum structure.
    fn to_8d_nested_enum_representation(value: &T) -> crate::nested_enums::NestedEnum;

    fn to_numerical_hash(value: &T) -> u64;
}