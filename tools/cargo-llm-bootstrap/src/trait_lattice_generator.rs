use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::error::AppError;
use crate::trait_extractor::TraitExtractor;
use crate::trait_numbering::TraitNumberer;
use crate::trait_types::{DeclTrait, TraitDeps, TraitLattice, TraitMorphism, MorphismKind};

pub struct TraitLatticeGenerator {
    extractor: TraitExtractor,
    numberer: TraitNumberer,
}

impl TraitLatticeGenerator {
    pub fn new() -> Self {
        TraitLatticeGenerator {
            extractor: TraitExtractor,
            numberer: TraitNumberer::new(),
        }
    }

    /// Generates the complete TraitLattice from a given Rust source path.
    pub fn generate_lattice(&mut self, rust_src_path: &Path) -> Result<TraitLattice, AppError> {
        println!("🏗️ Generating TraitLattice from: {:?}", rust_src_path);

        let mut all_decl_traits = Vec::new();
        let mut all_trait_deps = Vec::new();

        for entry in WalkDir::new(rust_src_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let (mut decl_traits, mut trait_deps) = self.extractor.extract_from_file(entry.path())?;
            all_decl_traits.append(&mut decl_traits);
            all_trait_deps.append(&mut trait_deps);
        }

        println!("Extracted {} declarations and {} trait dependencies.", all_decl_traits.len(), all_trait_deps.len());

        let mut traits_map = HashMap::new();
        for decl_trait in all_decl_traits {
            traits_map.insert(decl_trait.name.clone(), decl_trait);
        }
        
        // Group TraitDeps by the trait being implemented for easier lookup
        let mut grouped_trait_deps: HashMap<String, Vec<TraitDeps>> = HashMap::new();
        for deps in all_trait_deps {
            grouped_trait_deps.entry(deps.impl_trait_name.clone()).or_default().push(deps);
        }

        let mut lattice = TraitLattice {
            traits: traits_map,
            trait_dependencies: grouped_trait_deps,
            morphisms: HashMap::new(),
        };

        // Assign Gödel numbers
        self.numberer.assign_numbers(&mut lattice)?;
        println!("Assigned Gödel numbers to all declarations and dependencies.");

        // Construct morphisms
        self.construct_morphisms(&mut lattice);
        println!("Constructed {} morphisms.", lattice.morphisms.len());

        Ok(lattice)
    }

    /// Constructs the morphisms (relationships) between traits in the lattice.
    fn construct_morphisms(&self, lattice: &mut TraitLattice) {
        // Iterate through all declarations to find relationships
        for (_, decl_trait) in &lattice.traits {
            // If it's an Impl, create an Implementation morphism
            if decl_trait.kind == crate::trait_types::DeclKind::Impl {
                // The name of an Impl DeclTrait is "Type::Trait"
                let parts: Vec<&str> = decl_trait.name.split("::").collect();
                if parts.len() == 2 {
                    let impl_for_type = parts[0].to_string();
                    let impl_trait_name = parts[1].to_string();

                    // Morphism: Type -> Impl Trait (Implementation)
                    let morphism_key = (impl_for_type.clone(), decl_trait.name.clone());
                    let morphism = TraitMorphism {
                        from_trait: impl_for_type,
                        to_trait: decl_trait.name.clone(),
                        kind: MorphismKind::Implementation,
                        godel_number: (decl_trait.godel_number.saturating_add(
                            lattice.traits.get(parts[0]).map_or(0, |dt| dt.godel_number)
                        )), // Simplified Gödel for morphism
                    };
                    lattice.morphisms.insert(morphism_key, morphism);
                }
            }
            // Add other types of morphisms based on DeclTrait properties or TraitDeps
        }

        // Iterate through trait dependencies to create Dependency morphisms
        if let Some(impl_deps_list) = lattice.trait_dependencies.get("impls") { // Assuming TraitDeps are stored under "impls"
            for trait_deps in impl_deps_list {
                for dep_name in &trait_deps.dependencies {
                    let from_trait = format!("{}::{}", trait_deps.impl_for_type, trait_deps.impl_trait_name);
                    let morphism_key = (from_trait.clone(), dep_name.clone());
                    let morphism = TraitMorphism {
                        from_trait: from_trait.clone(),
                        to_trait: dep_name.clone(),
                        kind: MorphismKind::Dependency,
                        godel_number: (trait_deps.godel_number.saturating_add(
                            lattice.traits.get(dep_name).map_or(0, |dt| dt.godel_number)
                        )), // Simplified Gödel for morphism
                    };
                    lattice.morphisms.insert(morphism_key, morphism);
                }
            }
        }
    }
}
