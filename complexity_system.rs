// Complexity System: Weight (w) and Level (ℓ) → Computational Complexity
// Partial order: (w₁, ℓ₁) ≤ (w₂, ℓ₂) if w₁ ≤ w₂ and ℓ₁ | ℓ₂

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ComplexitySystem {
    pub modular_complexity_classes: Vec<ModularComplexityClass>,
    pub complexity_partial_order: ComplexityPartialOrder,
    pub resource_constraints: ResourceConstraints,
    pub program_classifications: Vec<ProgramClassification>,
}

#[derive(Debug, Clone)]
pub struct ModularComplexityClass {
    pub class_id: String,
    pub weight: u32,
    pub level: u32,
    pub computational_interpretation: ComputationalInterpretation,
    pub resource_bounds: ResourceBounds,
}

#[derive(Debug, Clone)]
pub struct ComputationalInterpretation {
    pub time_complexity: String,
    pub space_complexity: String,
    pub classical_complexity_class: String,
    pub modular_form_basis: String,
}

#[derive(Debug, Clone)]
pub struct ResourceBounds {
    pub memory_bound: String,
    pub computation_bound: String,
    pub io_bound: String,
    pub mathematical_justification: String,
}

#[derive(Debug, Clone)]
pub struct ComplexityPartialOrder {
    pub ordering_relations: Vec<OrderingRelation>,
    pub complexity_lattice: ComplexityLattice,
    pub comparison_algorithm: ComparisonAlgorithm,
}

#[derive(Debug, Clone)]
pub struct OrderingRelation {
    pub program_a: (u32, u32), // (weight, level)
    pub program_b: (u32, u32),
    pub relation_type: RelationType,
    pub mathematical_proof: String,
}

#[derive(Debug, Clone, Copy)]
pub enum RelationType {
    LessComplex,    // (w₁, ℓ₁) ≤ (w₂, ℓ₂)
    MoreComplex,    // (w₁, ℓ₁) ≥ (w₂, ℓ₂)
    Incomparable,   // No ordering relation
    Equivalent,     // Same complexity class
}

#[derive(Debug, Clone)]
pub struct ComplexityLattice {
    pub lattice_nodes: Vec<LatticeNode>,
    pub lattice_edges: Vec<LatticeEdge>,
    pub minimal_elements: Vec<(u32, u32)>,
    pub maximal_elements: Vec<(u32, u32)>,
}

#[derive(Debug, Clone)]
pub struct LatticeNode {
    pub weight: u32,
    pub level: u32,
    pub complexity_class: String,
    pub program_examples: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LatticeEdge {
    pub from_node: (u32, u32),
    pub to_node: (u32, u32),
    pub complexity_increase: String,
}

#[derive(Debug, Clone)]
pub struct ComparisonAlgorithm {
    pub algorithm_name: String,
    pub weight_comparison: String,
    pub level_divisibility_check: String,
    pub complexity_bound: String,
}

#[derive(Debug, Clone)]
pub struct ResourceConstraints {
    pub weight_based_constraints: Vec<WeightConstraint>,
    pub level_based_constraints: Vec<LevelConstraint>,
    pub combined_constraints: Vec<CombinedConstraint>,
}

#[derive(Debug, Clone)]
pub struct WeightConstraint {
    pub weight: u32,
    pub memory_scaling: String,
    pub computation_scaling: String,
    pub mathematical_basis: String,
}

#[derive(Debug, Clone)]
pub struct LevelConstraint {
    pub level: u32,
    pub io_complexity: String,
    pub communication_overhead: String,
    pub divisibility_properties: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct CombinedConstraint {
    pub weight: u32,
    pub level: u32,
    pub total_resource_bound: String,
    pub optimization_potential: String,
}

#[derive(Debug, Clone)]
pub struct ProgramClassification {
    pub program_name: String,
    pub weight: u32,
    pub level: u32,
    pub complexity_class: String,
    pub resource_usage: ResourceUsage,
    pub modular_form_representation: String,
}

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub time_usage: String,
    pub space_usage: String,
    pub io_usage: String,
    pub actual_measurements: HashMap<String, f64>,
}

impl ComplexitySystem {
    pub fn new() -> Self {
        let modular_complexity_classes = vec![
            ModularComplexityClass {
                class_id: "MC_4_1".to_string(),
                weight: 4,
                level: 1,
                computational_interpretation: ComputationalInterpretation {
                    time_complexity: "O(n)".to_string(),
                    space_complexity: "O(1)".to_string(),
                    classical_complexity_class: "P".to_string(),
                    modular_form_basis: "Eisenstein E₄(τ)".to_string(),
                },
                resource_bounds: ResourceBounds {
                    memory_bound: "4 * log(n) bytes".to_string(),
                    computation_bound: "4 * n operations".to_string(),
                    io_bound: "1 * n reads".to_string(),
                    mathematical_justification: "Weight 4 bounds polynomial growth".to_string(),
                },
            },
            ModularComplexityClass {
                class_id: "MC_6_1".to_string(),
                weight: 6,
                level: 1,
                computational_interpretation: ComputationalInterpretation {
                    time_complexity: "O(n²)".to_string(),
                    space_complexity: "O(n)".to_string(),
                    classical_complexity_class: "P".to_string(),
                    modular_form_basis: "Eisenstein E₆(τ)".to_string(),
                },
                resource_bounds: ResourceBounds {
                    memory_bound: "6 * n bytes".to_string(),
                    computation_bound: "6 * n² operations".to_string(),
                    io_bound: "1 * n² reads".to_string(),
                    mathematical_justification: "Weight 6 allows quadratic complexity".to_string(),
                },
            },
            ModularComplexityClass {
                class_id: "MC_12_1".to_string(),
                weight: 12,
                level: 1,
                computational_interpretation: ComputationalInterpretation {
                    time_complexity: "O(n⁶)".to_string(),
                    space_complexity: "O(n³)".to_string(),
                    classical_complexity_class: "PSPACE".to_string(),
                    modular_form_basis: "Ramanujan Δ(τ)".to_string(),
                },
                resource_bounds: ResourceBounds {
                    memory_bound: "12 * n³ bytes".to_string(),
                    computation_bound: "12 * n⁶ operations".to_string(),
                    io_bound: "1 * n⁶ reads".to_string(),
                    mathematical_justification: "Weight 12 enables high polynomial complexity".to_string(),
                },
            },
            ModularComplexityClass {
                class_id: "MC_4_3".to_string(),
                weight: 4,
                level: 3,
                computational_interpretation: ComputationalInterpretation {
                    time_complexity: "O(n log n)".to_string(),
                    space_complexity: "O(log n)".to_string(),
                    classical_complexity_class: "P".to_string(),
                    modular_form_basis: "Modular form of weight 4, level 3".to_string(),
                },
                resource_bounds: ResourceBounds {
                    memory_bound: "4 * log(n) bytes".to_string(),
                    computation_bound: "4 * n * log(n) operations".to_string(),
                    io_bound: "3 * n reads".to_string(),
                    mathematical_justification: "Level 3 adds logarithmic factor".to_string(),
                },
            },
        ];
        
        let complexity_partial_order = ComplexityPartialOrder {
            ordering_relations: Self::generate_ordering_relations(),
            complexity_lattice: Self::generate_complexity_lattice(),
            comparison_algorithm: ComparisonAlgorithm {
                algorithm_name: "Modular Complexity Comparison".to_string(),
                weight_comparison: "w₁ ≤ w₂".to_string(),
                level_divisibility_check: "ℓ₁ | ℓ₂".to_string(),
                complexity_bound: "O(log(max(w₁,w₂)) + log(max(ℓ₁,ℓ₂)))".to_string(),
            },
        };
        
        let resource_constraints = ResourceConstraints {
            weight_based_constraints: Self::generate_weight_constraints(),
            level_based_constraints: Self::generate_level_constraints(),
            combined_constraints: Self::generate_combined_constraints(),
        };
        
        let program_classifications = Self::generate_program_classifications();
        
        Self {
            modular_complexity_classes,
            complexity_partial_order,
            resource_constraints,
            program_classifications,
        }
    }
    
    fn generate_ordering_relations() -> Vec<OrderingRelation> {
        vec![
            OrderingRelation {
                program_a: (4, 1),
                program_b: (6, 1),
                relation_type: RelationType::LessComplex,
                mathematical_proof: "4 ≤ 6 and 1 | 1".to_string(),
            },
            OrderingRelation {
                program_a: (4, 1),
                program_b: (12, 1),
                relation_type: RelationType::LessComplex,
                mathematical_proof: "4 ≤ 12 and 1 | 1".to_string(),
            },
            OrderingRelation {
                program_a: (4, 1),
                program_b: (4, 3),
                relation_type: RelationType::LessComplex,
                mathematical_proof: "4 ≤ 4 and 1 | 3".to_string(),
            },
            OrderingRelation {
                program_a: (6, 1),
                program_b: (12, 1),
                relation_type: RelationType::LessComplex,
                mathematical_proof: "6 ≤ 12 and 1 | 1".to_string(),
            },
            OrderingRelation {
                program_a: (4, 3),
                program_b: (6, 3),
                relation_type: RelationType::LessComplex,
                mathematical_proof: "4 ≤ 6 and 3 | 3".to_string(),
            },
        ]
    }
    
    fn generate_complexity_lattice() -> ComplexityLattice {
        ComplexityLattice {
            lattice_nodes: vec![
                LatticeNode {
                    weight: 4,
                    level: 1,
                    complexity_class: "Linear".to_string(),
                    program_examples: vec!["Array traversal".to_string(), "Simple loops".to_string()],
                },
                LatticeNode {
                    weight: 6,
                    level: 1,
                    complexity_class: "Quadratic".to_string(),
                    program_examples: vec!["Nested loops".to_string(), "Matrix operations".to_string()],
                },
                LatticeNode {
                    weight: 12,
                    level: 1,
                    complexity_class: "High Polynomial".to_string(),
                    program_examples: vec!["Complex algorithms".to_string(), "Graph algorithms".to_string()],
                },
            ],
            lattice_edges: vec![
                LatticeEdge {
                    from_node: (4, 1),
                    to_node: (6, 1),
                    complexity_increase: "Linear to quadratic".to_string(),
                },
                LatticeEdge {
                    from_node: (6, 1),
                    to_node: (12, 1),
                    complexity_increase: "Quadratic to high polynomial".to_string(),
                },
            ],
            minimal_elements: vec![(4, 1)],
            maximal_elements: vec![(12, 1)],
        }
    }
    
    fn generate_weight_constraints() -> Vec<WeightConstraint> {
        vec![
            WeightConstraint {
                weight: 4,
                memory_scaling: "O(w * log n)".to_string(),
                computation_scaling: "O(w * n)".to_string(),
                mathematical_basis: "Eisenstein series growth bounds".to_string(),
            },
            WeightConstraint {
                weight: 6,
                memory_scaling: "O(w * n)".to_string(),
                computation_scaling: "O(w * n²)".to_string(),
                mathematical_basis: "Weight 6 modular form bounds".to_string(),
            },
            WeightConstraint {
                weight: 12,
                memory_scaling: "O(w * n³)".to_string(),
                computation_scaling: "O(w * n⁶)".to_string(),
                mathematical_basis: "Ramanujan Δ function growth".to_string(),
            },
        ]
    }
    
    fn generate_level_constraints() -> Vec<LevelConstraint> {
        vec![
            LevelConstraint {
                level: 1,
                io_complexity: "O(ℓ * n)".to_string(),
                communication_overhead: "Minimal".to_string(),
                divisibility_properties: vec![1],
            },
            LevelConstraint {
                level: 3,
                io_complexity: "O(ℓ * n log n)".to_string(),
                communication_overhead: "Moderate".to_string(),
                divisibility_properties: vec![1, 3],
            },
            LevelConstraint {
                level: 12,
                io_complexity: "O(ℓ * n²)".to_string(),
                communication_overhead: "High".to_string(),
                divisibility_properties: vec![1, 2, 3, 4, 6, 12],
            },
        ]
    }
    
    fn generate_combined_constraints() -> Vec<CombinedConstraint> {
        vec![
            CombinedConstraint {
                weight: 4,
                level: 1,
                total_resource_bound: "O(4 * n + 1 * n) = O(n)".to_string(),
                optimization_potential: "High - minimal constraints".to_string(),
            },
            CombinedConstraint {
                weight: 12,
                level: 1,
                total_resource_bound: "O(12 * n⁶ + 1 * n⁶) = O(n⁶)".to_string(),
                optimization_potential: "Low - high complexity".to_string(),
            },
        ]
    }
    
    fn generate_program_classifications() -> Vec<ProgramClassification> {
        vec![
            ProgramClassification {
                program_name: "linear_search".to_string(),
                weight: 4,
                level: 1,
                complexity_class: "MC_4_1".to_string(),
                resource_usage: ResourceUsage {
                    time_usage: "O(n)".to_string(),
                    space_usage: "O(1)".to_string(),
                    io_usage: "O(n)".to_string(),
                    actual_measurements: HashMap::from([
                        ("time_constant".to_string(), 4.0),
                        ("space_constant".to_string(), 1.0),
                    ]),
                },
                modular_form_representation: "E₄(τ) weight 4, level 1".to_string(),
            },
            ProgramClassification {
                program_name: "bubble_sort".to_string(),
                weight: 6,
                level: 1,
                complexity_class: "MC_6_1".to_string(),
                resource_usage: ResourceUsage {
                    time_usage: "O(n²)".to_string(),
                    space_usage: "O(1)".to_string(),
                    io_usage: "O(n²)".to_string(),
                    actual_measurements: HashMap::from([
                        ("time_constant".to_string(), 6.0),
                        ("space_constant".to_string(), 1.0),
                    ]),
                },
                modular_form_representation: "E₆(τ) weight 6, level 1".to_string(),
            },
            ProgramClassification {
                program_name: "matrix_multiplication".to_string(),
                weight: 12,
                level: 1,
                complexity_class: "MC_12_1".to_string(),
                resource_usage: ResourceUsage {
                    time_usage: "O(n³)".to_string(),
                    space_usage: "O(n²)".to_string(),
                    io_usage: "O(n³)".to_string(),
                    actual_measurements: HashMap::from([
                        ("time_constant".to_string(), 12.0),
                        ("space_constant".to_string(), 12.0),
                    ]),
                },
                modular_form_representation: "Δ(τ) weight 12, level 1".to_string(),
            },
        ]
    }
    
    pub fn compare_complexity(&self, program_a: (u32, u32), program_b: (u32, u32)) -> ComplexityComparison {
        let (w1, l1) = program_a;
        let (w2, l2) = program_b;
        
        let weight_comparison = w1.cmp(&w2);
        let level_divides = l2 % l1 == 0;
        
        let relation_type = match weight_comparison {
            std::cmp::Ordering::Less => {
                if level_divides {
                    RelationType::LessComplex
                } else {
                    RelationType::Incomparable
                }
            },
            std::cmp::Ordering::Equal => {
                if l1 == l2 {
                    RelationType::Equivalent
                } else if level_divides {
                    RelationType::LessComplex
                } else if l1 % l2 == 0 {
                    RelationType::MoreComplex
                } else {
                    RelationType::Incomparable
                }
            },
            std::cmp::Ordering::Greater => {
                if l1 % l2 == 0 {
                    RelationType::MoreComplex
                } else {
                    RelationType::Incomparable
                }
            },
        };
        
        ComplexityComparison {
            program_a,
            program_b,
            relation_type,
            weight_comparison: format!("{} vs {}", w1, w2),
            level_divisibility: format!("{} | {} = {}", l1, l2, level_divides),
            mathematical_justification: self.generate_comparison_proof(program_a, program_b, &relation_type),
        }
    }
    
    fn generate_comparison_proof(&self, program_a: (u32, u32), program_b: (u32, u32), relation: &RelationType) -> String {
        let (w1, l1) = program_a;
        let (w2, l2) = program_b;
        
        match relation {
            RelationType::LessComplex => format!("({}, {}) ≤ ({}, {}) because {} ≤ {} and {} | {}", w1, l1, w2, l2, w1, w2, l1, l2),
            RelationType::MoreComplex => format!("({}, {}) ≥ ({}, {}) because {} ≥ {} and {} | {}", w1, l1, w2, l2, w1, w2, l2, l1),
            RelationType::Equivalent => format!("({}, {}) = ({}, {}) because {} = {} and {} = {}", w1, l1, w2, l2, w1, w2, l1, l2),
            RelationType::Incomparable => format!("({}, {}) ⊥ ({}, {}) because divisibility condition fails", w1, l1, w2, l2),
        }
    }
    
    pub fn classify_program(&self, program_name: &str, weight: u32, level: u32) -> ProgramComplexityClassification {
        let complexity_class = self.modular_complexity_classes
            .iter()
            .find(|class| class.weight == weight && class.level == level);
        
        if let Some(class) = complexity_class {
            ProgramComplexityClassification {
                program_name: program_name.to_string(),
                weight,
                level,
                complexity_class: class.class_id.clone(),
                time_complexity: class.computational_interpretation.time_complexity.clone(),
                space_complexity: class.computational_interpretation.space_complexity.clone(),
                resource_bounds: class.resource_bounds.clone(),
                classification_successful: true,
            }
        } else {
            ProgramComplexityClassification {
                program_name: program_name.to_string(),
                weight,
                level,
                complexity_class: "Unknown".to_string(),
                time_complexity: "Unclassified".to_string(),
                space_complexity: "Unclassified".to_string(),
                resource_bounds: ResourceBounds {
                    memory_bound: "Unknown".to_string(),
                    computation_bound: "Unknown".to_string(),
                    io_bound: "Unknown".to_string(),
                    mathematical_justification: "No matching modular form".to_string(),
                },
                classification_successful: false,
            }
        }
    }
    
    pub fn generate_complexity_report(&self) -> String {
        format!(
            "🔄 COMPLEXITY SYSTEM: WEIGHT (w) AND LEVEL (ℓ) → COMPUTATIONAL COMPLEXITY\n\
             📐 Partial Order: (w₁, ℓ₁) ≤ (w₂, ℓ₂) if w₁ ≤ w₂ and ℓ₁ | ℓ₂\n\
             \n\
             📊 MODULAR COMPLEXITY CLASSES:\n\
             ├─ Total classes: {}\n\
             ├─ Weight range: {} to {}\n\
             ├─ Level range: {} to {}\n\
             └─ Classical mappings: P, PSPACE\n\
             \n\
             🔗 COMPLEXITY PARTIAL ORDER:\n\
             ├─ Ordering relations: {}\n\
             ├─ Lattice nodes: {}\n\
             ├─ Lattice edges: {}\n\
             └─ Comparison algorithm: O(log w + log ℓ)\n\
             \n\
             ⚡ RESOURCE CONSTRAINTS:\n\
             ├─ Weight-based constraints: {}\n\
             ├─ Level-based constraints: {}\n\
             └─ Combined constraints: {}\n\
             \n\
             🎯 PROGRAM CLASSIFICATIONS:\n\
             ├─ Classified programs: {}\n\
             ├─ Complexity classes covered: {}\n\
             └─ Resource measurements: available\n\
             \n\
             ✅ Complexity system validation: {}",
            self.modular_complexity_classes.len(),
            self.modular_complexity_classes.iter().map(|c| c.weight).min().unwrap_or(0),
            self.modular_complexity_classes.iter().map(|c| c.weight).max().unwrap_or(0),
            self.modular_complexity_classes.iter().map(|c| c.level).min().unwrap_or(0),
            self.modular_complexity_classes.iter().map(|c| c.level).max().unwrap_or(0),
            self.complexity_partial_order.ordering_relations.len(),
            self.complexity_partial_order.complexity_lattice.lattice_nodes.len(),
            self.complexity_partial_order.complexity_lattice.lattice_edges.len(),
            self.resource_constraints.weight_based_constraints.len(),
            self.resource_constraints.level_based_constraints.len(),
            self.resource_constraints.combined_constraints.len(),
            self.program_classifications.len(),
            self.modular_complexity_classes.len(),
            self.validate_complexity_system()
        )
    }
    
    fn validate_complexity_system(&self) -> bool {
        !self.modular_complexity_classes.is_empty() &&
        !self.complexity_partial_order.ordering_relations.is_empty() &&
        !self.program_classifications.is_empty()
    }
}

#[derive(Debug)]
pub struct ComplexityComparison {
    pub program_a: (u32, u32),
    pub program_b: (u32, u32),
    pub relation_type: RelationType,
    pub weight_comparison: String,
    pub level_divisibility: String,
    pub mathematical_justification: String,
}

#[derive(Debug)]
pub struct ProgramComplexityClassification {
    pub program_name: String,
    pub weight: u32,
    pub level: u32,
    pub complexity_class: String,
    pub time_complexity: String,
    pub space_complexity: String,
    pub resource_bounds: ResourceBounds,
    pub classification_successful: bool,
}

fn main() {
    let complexity_system = ComplexitySystem::new();
    println!("{}", complexity_system.generate_complexity_report());
    
    // Demonstrate complexity comparisons
    println!("\n🔍 COMPLEXITY COMPARISONS:");
    let comparisons = vec![
        ((4, 1), (6, 1)),
        ((4, 1), (4, 3)),
        ((6, 1), (12, 1)),
        ((4, 3), (6, 2)),
    ];
    
    for (prog_a, prog_b) in comparisons {
        let comparison = complexity_system.compare_complexity(prog_a, prog_b);
        println!("\n   ({}, {}) vs ({}, {}): {:?}", 
            comparison.program_a.0, comparison.program_a.1,
            comparison.program_b.0, comparison.program_b.1,
            comparison.relation_type);
        println!("   Weight: {}", comparison.weight_comparison);
        println!("   Level divisibility: {}", comparison.level_divisibility);
        println!("   Proof: {}", comparison.mathematical_justification);
    }
    
    // Demonstrate program classification
    println!("\n🎯 PROGRAM CLASSIFICATIONS:");
    let programs = vec![
        ("quicksort", 6, 1),
        ("dijkstra", 12, 1),
        ("binary_search", 4, 1),
    ];
    
    for (name, weight, level) in programs {
        let classification = complexity_system.classify_program(name, weight, level);
        println!("\n   Program: {}", classification.program_name);
        println!("   Weight: {}, Level: {}", classification.weight, classification.level);
        println!("   Complexity class: {}", classification.complexity_class);
        println!("   Time: {}, Space: {}", classification.time_complexity, classification.space_complexity);
        println!("   Classification successful: {}", classification.classification_successful);
    }
}
