use std::collections::HashMap;
use crate::trait_types::{DeclTrait, TraitDeps, TraitLattice};
use crate::semantic_constraints::GödelNumber;

pub struct TraitNumberer {
    // A mapping from declaration name (string) to its Gödel number.
    // This will be built up as numbers are assigned.
    pub assigned_numbers: HashMap<String, u64>,
    // A queue for declarations whose numbers are pending calculation due to unassigned dependencies.
    // Stored as (declaration_name, dependencies_names)
    pub pending_declarations: Vec<(String, Vec<String>)>,
    pub next_base_number: u64, // Used for assigning initial numbers to base declarations.
}

impl TraitNumberer {
    pub fn new() -> Self {
        TraitNumberer {
            assigned_numbers: HashMap::new(),
            pending_declarations: Vec::new(),
            next_base_number: 1, // Start Gödel numbers from 1.
        }
    }

    /// Assigns Gödel numbers to all declarations and trait dependencies in the lattice.
    pub fn assign_numbers(&mut self, lattice: &mut TraitLattice) -> Result<(), String> {
        self.assign_initial_numbers(lattice);
        self.resolve_pending_declarations(lattice)?;
        self.verify_all_numbered(lattice)?;
        Ok(())
    }

    /// Assigns initial Gödel numbers to declarations that have no dependencies (or whose dependencies are already numbered).
    fn assign_initial_numbers(&mut self, lattice: &mut TraitLattice) {
        let mut progress_made = true;
        while progress_made {
            progress_made = false;
            // First pass: assign numbers to declarations without external trait dependencies.
            // This is simplified: if an item is not an impl, it gets a base number.
            let mut declarations_to_process = Vec::new();
            for (name, decl_trait) in lattice.traits.iter_mut() {
                if decl_trait.godel_number == 0 && decl_trait.kind != crate::trait_types::DeclKind::Impl {
                    declarations_to_process.push(name.clone());
                }
            }

            for name in declarations_to_process {
                if !self.assigned_numbers.contains_key(&name) {
                    let number = self.get_next_base_number();
                    self.assigned_numbers.insert(name.clone(), number);
                    if let Some(decl_trait) = lattice.traits.get_mut(&name) {
                        decl_trait.godel_number = number;
                    }
                    progress_made = true;
                }
            }

            // Second pass: process impls whose dependencies are all numbered.
            let mut impls_to_process = Vec::new();
            if let Some(trait_deps_map) = lattice.trait_dependencies.get_mut("impls") { // Assuming "impls" key holds all TraitDeps
                for trait_deps in trait_deps_map.iter_mut() {
                    if trait_deps.godel_number == 0 {
                        let all_deps_numbered = trait_deps.dependencies.iter()
                            .all(|dep_name| self.assigned_numbers.contains_key(dep_name));
                        if all_deps_numbered {
                            impls_to_process.push(trait_deps.impl_for_type.clone());
                        } else {
                            // If not all dependencies are numbered, add to pending.
                            self.pending_declarations.push((
                                format!("{}::{}", trait_deps.impl_for_type, trait_deps.impl_trait_name),
                                trait_deps.dependencies.clone(),
                            ));
                        }
                    }
                }
            }

            for name in impls_to_process {
                // Remove duplicates from pending_declarations if already assigned.
                // This logic might be better placed in resolve_pending_declarations or similar.
            }
        }
    }

    /// Resolves Gödel numbers for declarations that depend on others.
    fn resolve_pending_declarations(&mut self, lattice: &mut TraitLattice) -> Result<(), String> {
        let mut progress_made = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000; // Prevent infinite loops for circular dependencies

        while progress_made && !self.pending_declarations.is_empty() && iterations < MAX_ITERATIONS {
            progress_made = false;
            iterations += 1;
            let mut next_pending = Vec::new();

            for (decl_name, deps) in std::mem::take(&mut self.pending_declarations) {
                let mut sum_of_deps_numbers = 0;
                let mut all_deps_resolved = true;

                for dep_name in &deps {
                    if let Some(&dep_num) = self.assigned_numbers.get(dep_name) {
                        sum_of_deps_numbers = sum_of_deps_numbers.checked_add(dep_num).ok_or_else(|| "Gödel number overflow".to_string())?;
                    } else {
                        all_deps_resolved = false;
                        break;
                    }
                }

                if all_deps_resolved {
                    if let Some(decl_trait) = lattice.traits.get_mut(&decl_name) {
                        decl_trait.godel_number = sum_of_deps_numbers;
                        self.assigned_numbers.insert(decl_name.clone(), sum_of_deps_numbers);
                        progress_made = true;
                    } else {
                        // This might be a TraitDeps entry directly
                        if let Some(trait_deps_list) = lattice.trait_dependencies.get_mut("impls") {
                            for trait_deps in trait_deps_list.iter_mut() {
                                if format!("{}::{}", trait_deps.impl_for_type, trait_deps.impl_trait_name) == decl_name {
                                    trait_deps.godel_number = sum_of_deps_numbers;
                                    self.assigned_numbers.insert(decl_name.clone(), sum_of_deps_numbers);
                                    progress_made = true;
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    next_pending.push((decl_name, deps));
                }
            }
            self.pending_declarations = next_pending;
        }

        if !self.pending_declarations.is_empty() {
            return Err(format!("Unresolved circular dependencies or unassigned base numbers after {} iterations. Pending: {:?}", iterations, self.pending_declarations));
        }

        Ok(())
    }


    /// Ensures all declarations and trait dependencies in the lattice have been assigned a Gödel number.
    fn verify_all_numbered(&self, lattice: &TraitLattice) -> Result<(), String> {
        for (name, decl_trait) in &lattice.traits {
            if decl_trait.godel_number == 0 {
                return Err(format!("Declaration '{}' was not assigned a Gödel number.", name));
            }
        }
        if let Some(trait_deps_map) = lattice.trait_dependencies.get("impls") {
            for trait_deps in trait_deps_map {
                if trait_deps.godel_number == 0 {
                    return Err(format!("TraitDeps for '{:?}' was not assigned a Gödel number.", trait_deps));
                }
            }
        }
        Ok(())
    }

    /// Provides a unique base Gödel number.
    fn get_next_base_number(&mut self) -> u64 {
        let number = self.next_base_number;
        self.next_base_number += 1;
        number
    }
}
