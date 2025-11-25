use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// syn::{Ident, Path, Type}; // These are not used directly here anymore

/// Represents a trait associated with a declaration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeclTrait {
    pub name: String,
    pub kind: DeclKind,
    pub generics: Vec<String>, // Generic parameters of the declaration
    pub bounds: Vec<String>,   // Trait bounds of the declaration
    pub associated_items: Vec<String>, // Names of associated types/fns
    pub godel_number: u64,     // The Gödel number of this trait
    pub monster_number: Option<u64>, // The Monster Number of this declaration at level zero
    pub enum_numbering: Option<EnumNumbering>, // Specific numbering for enums
}

/// The kind of declaration (e.g., struct, enum, function, impl).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeclKind {
    Struct,
    Enum,
    Function,
    Trait,
    Impl,
    Module,
    Constant,
    Static,
    TypeAlias,
    Use,
    Macro,
    Other,
}

/// Specific numbering schemes for enum declarations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EnumNumbering {
    Vector(Vec<u64>),
    Function(String), // Name of the function that generates numbers
    Constant(u64),    // A single constant number for the enum
}

/// Represents dependencies of a trait implementation on other traits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TraitDeps {
    pub impl_trait_name: String, // The trait being implemented
    pub impl_for_type: String,   // The type for which the trait is implemented
    pub dependencies: Vec<String>, // Names of traits that this impl depends on
    pub godel_number: u64,       // Gödel number of this set of dependencies
}

/// Represents the lattice structure of traits and their relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitLattice {
    pub traits: HashMap<String, DeclTrait>, // Trait name -> DeclTrait
    pub trait_dependencies: HashMap<String, Vec<TraitDeps>>, // Trait name -> list of its implementations' dependencies
    pub morphisms: HashMap<(String, String), TraitMorphism>, // (Source Trait, Target Trait) -> Morphism
}

/// Represents a relationship or transformation between two traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitMorphism {
    pub from_trait: String,
    pub to_trait: String,
    pub kind: MorphismKind,
    pub godel_number: u64, // Gödel number of this morphism
}

/// The kind of relationship between two traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MorphismKind {
    Implementation,  // Trait A is implemented for Type B, Type B has Trait A
    Inheritance,     // Trait A inherits from Trait B
    Composition,     // Trait A is composed of Trait B and C
    Conversion,      // Trait A can be converted to Trait B
    Dependency,      // Trait A uses Trait B
    Other,
}
