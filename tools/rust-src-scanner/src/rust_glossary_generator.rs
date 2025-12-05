/// Rust Glossary Generator with Topological Ordering
/// Term, value, usage, eigenvector ordered for building rustc from scratch
use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustTerm {
    pub name: String,
    pub value: String,                    // Definition/implementation
    pub usage: Vec<String>,               // How it's used in rustc
    pub eigenvector: Vec<f64>,            // 15-dimensional Monster Group vector
    pub dependencies: Vec<String>,        // What this term depends on
    pub dependents: Vec<String>,          // What depends on this term
    pub build_order: usize,               // Topological order (0 = build first)
    pub monster_factor: (u64, u32),       // Monster Group assignment
    pub semantic_category: String,        // "lexer", "parser", "ast", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustGlossary {
    pub terms: Vec<RustTerm>,
    pub build_layers: Vec<Vec<String>>,   // Topologically ordered layers
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub total_terms: usize,
}

pub struct RustGlossaryGenerator {
    pub glossary: RustGlossary,
    pub term_definitions: HashMap<String, String>,
    pub usage_patterns: HashMap<String, Vec<String>>,
    pub eigenvectors: HashMap<String, Vec<f64>>,
    pub monster_mapping: HashMap<String, (u64, u32)>,
}

impl RustGlossaryGenerator {
    pub fn new() -> Self {
        Self {
            glossary: RustGlossary {
                terms: Vec::new(),
                build_layers: Vec::new(),
                dependency_graph: HashMap::new(),
                total_terms: 0,
            },
            term_definitions: HashMap::new(),
            usage_patterns: HashMap::new(),
            eigenvectors: HashMap::new(),
            monster_mapping: HashMap::new(),
        }
    }

    /// Generate complete Rust glossary from rustc analysis
    pub fn generate_glossary(&mut self, 
        perfect_hash: &crate::perfect_monster_hash::PerfectMonsterHash,
        eigenvector_analysis: &crate::context_eigenvector_analyzer::ContextEigenvectorAnalyzer
    ) -> Result<(), String> {
        println!("📚 Generating Rust Glossary with Topological Ordering");
        
        // Extract core rustc terms
        self.extract_core_terms()?;
        
        // Build dependency graph
        self.build_dependency_graph()?;
        
        // Compute topological ordering
        self.compute_topological_order()?;
        
        // Assign eigenvectors and Monster factors
        self.assign_mathematical_properties(perfect_hash, eigenvector_analysis)?;
        
        // Generate build layers
        self.generate_build_layers()?;
        
        println!("✅ Glossary generated with {} terms in {} layers", 
                self.glossary.total_terms, self.glossary.build_layers.len());
        
        Ok(())
    }

    fn extract_core_terms(&mut self) -> Result<(), String> {
        // Core rustc terms in semantic order
        let core_terms = [
            // Lexical Analysis
            ("Token", "Basic lexical unit", vec!["lexer", "parser"]),
            ("Span", "Source location", vec!["error_reporting", "diagnostics"]),
            ("Symbol", "Interned string", vec!["identifiers", "names"]),
            
            // Parsing
            ("TokenStream", "Stream of tokens", vec!["macro_expansion", "parsing"]),
            ("ParseSess", "Parse session", vec!["parser_state", "error_handling"]),
            
            // AST
            ("Expr", "Expression node", vec!["ast", "type_checking"]),
            ("Stmt", "Statement node", vec!["ast", "control_flow"]),
            ("Item", "Top-level item", vec!["modules", "declarations"]),
            ("Pat", "Pattern", vec!["pattern_matching", "destructuring"]),
            ("Ty", "Type representation", vec!["type_system", "inference"]),
            
            // HIR
            ("HirId", "HIR node identifier", vec!["hir", "resolution"]),
            ("Body", "Function body", vec!["hir", "mir_building"]),
            ("Def", "Definition", vec!["name_resolution", "paths"]),
            
            // Types
            ("TyCtxt", "Type context", vec!["type_checking", "inference"]),
            ("Substs", "Type substitutions", vec!["generics", "monomorphization"]),
            ("Region", "Lifetime region", vec!["borrow_checking", "lifetimes"]),
            
            // MIR
            ("BasicBlock", "MIR basic block", vec!["mir", "control_flow"]),
            ("Place", "Memory location", vec!["mir", "borrow_checking"]),
            ("Operand", "MIR operand", vec!["mir", "code_generation"]),
            
            // Codegen
            ("CodegenCx", "Codegen context", vec!["llvm", "code_generation"]),
            ("Value", "LLVM value", vec!["llvm", "backend"]),
        ];

        for (name, definition, usage) in core_terms {
            self.term_definitions.insert(name.to_string(), definition.to_string());
            self.usage_patterns.insert(name.to_string(), usage.iter().map(|s| s.to_string()).collect());
        }

        Ok(())
    }

    fn build_dependency_graph(&mut self) -> Result<(), String> {
        // Build dependency relationships for topological sorting
        let dependencies = [
            ("Token", vec![]),
            ("Span", vec![]),
            ("Symbol", vec![]),
            ("TokenStream", vec!["Token"]),
            ("ParseSess", vec!["Span", "Symbol"]),
            ("Expr", vec!["TokenStream", "Span"]),
            ("Stmt", vec!["Expr"]),
            ("Item", vec!["Expr", "Stmt"]),
            ("Pat", vec!["Expr"]),
            ("Ty", vec!["Symbol", "Span"]),
            ("HirId", vec!["Symbol"]),
            ("Body", vec!["Expr", "Stmt"]),
            ("Def", vec!["Symbol", "Span"]),
            ("TyCtxt", vec!["Ty", "Def"]),
            ("Substs", vec!["Ty", "TyCtxt"]),
            ("Region", vec!["TyCtxt"]),
            ("BasicBlock", vec!["Body"]),
            ("Place", vec!["Ty", "HirId"]),
            ("Operand", vec!["Place"]),
            ("CodegenCx", vec!["TyCtxt"]),
            ("Value", vec!["CodegenCx", "Operand"]),
        ];

        for (term, deps) in dependencies {
            self.glossary.dependency_graph.insert(
                term.to_string(), 
                deps.iter().map(|s| s.to_string()).collect()
            );
        }

        Ok(())
    }

    fn compute_topological_order(&mut self) -> Result<(), String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize in-degrees and reverse graph
        for (term, deps) in &self.glossary.dependency_graph {
            in_degree.insert(term.clone(), deps.len());
            
            for dep in deps {
                graph.entry(dep.clone()).or_insert_with(Vec::new).push(term.clone());
            }
        }
        
        // Kahn's algorithm for topological sorting
        let mut queue = VecDeque::new();
        let mut order = Vec::new();
        
        // Find nodes with no incoming edges
        for (term, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(term.clone());
            }
        }
        
        while let Some(term) = queue.pop_front() {
            order.push(term.clone());
            
            if let Some(dependents) = graph.get(&term) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
        
        // Assign build order
        for (build_order, term) in order.iter().enumerate() {
            if let Some(definition) = self.term_definitions.get(term) {
                let usage = self.usage_patterns.get(term).cloned().unwrap_or_default();
                
                let rust_term = RustTerm {
                    name: term.clone(),
                    value: definition.clone(),
                    usage,
                    eigenvector: vec![0.0; 15], // Will be filled later
                    dependencies: self.glossary.dependency_graph.get(term).cloned().unwrap_or_default(),
                    dependents: graph.get(term).cloned().unwrap_or_default(),
                    build_order,
                    monster_factor: (2, 1), // Default, will be updated
                    semantic_category: self.infer_semantic_category(term),
                };
                
                self.glossary.terms.push(rust_term);
            }
        }
        
        self.glossary.total_terms = self.glossary.terms.len();
        Ok(())
    }

    fn assign_mathematical_properties(&mut self,
        perfect_hash: &crate::perfect_monster_hash::PerfectMonsterHash,
        _eigenvector_analysis: &crate::context_eigenvector_analyzer::ContextEigenvectorAnalyzer
    ) -> Result<(), String> {
        
        for term in &mut self.glossary.terms {
            // Assign Monster Group factor from perfect hash
            if let Some(&factor) = perfect_hash.term_to_factor.get(&term.name) {
                term.monster_factor = factor;
            }
            
            // Generate eigenvector based on semantic category and dependencies
            term.eigenvector = self.generate_term_eigenvector(&term.name, &term.semantic_category, term.build_order);
        }
        
        Ok(())
    }

    fn generate_term_eigenvector(&self, term_name: &str, category: &str, build_order: usize) -> Vec<f64> {
        let mut vector = vec![0.0; 15];
        
        // Base vector from semantic category
        match category {
            "lexer" => vector[0] = 1.0,      // Prime 2
            "parser" => vector[1] = 1.0,     // Prime 3
            "ast" => vector[2] = 1.0,        // Prime 5
            "hir" => vector[3] = 1.0,        // Prime 7
            "type_system" => vector[4] = 1.0, // Prime 11
            "mir" => vector[5] = 1.0,        // Prime 13
            "codegen" => vector[6] = 1.0,    // Prime 17
            _ => vector[0] = 0.5,            // Default
        }
        
        // Weight by build order (earlier = more fundamental)
        let order_weight = 1.0 / (build_order + 1) as f64;
        for i in 0..vector.len() {
            vector[i] *= order_weight;
        }
        
        // Add term-specific adjustments
        if term_name.contains("Ctxt") || term_name.contains("Context") {
            vector[14] = 0.8; // Prime 71 for core contexts
        }
        
        vector
    }

    fn generate_build_layers(&mut self) -> Result<(), String> {
        // Group terms by build order into layers
        let mut layers: HashMap<usize, Vec<String>> = HashMap::new();
        
        for term in &self.glossary.terms {
            layers.entry(term.build_order).or_insert_with(Vec::new).push(term.name.clone());
        }
        
        // Convert to ordered layers
        let mut layer_keys: Vec<usize> = layers.keys().copied().collect();
        layer_keys.sort();
        
        for layer_idx in layer_keys {
            if let Some(layer_terms) = layers.get(&layer_idx) {
                self.glossary.build_layers.push(layer_terms.clone());
            }
        }
        
        Ok(())
    }

    fn infer_semantic_category(&self, term: &str) -> String {
        match term {
            "Token" | "Span" | "Symbol" => "lexer".to_string(),
            "TokenStream" | "ParseSess" => "parser".to_string(),
            "Expr" | "Stmt" | "Item" | "Pat" => "ast".to_string(),
            "HirId" | "Body" | "Def" => "hir".to_string(),
            "Ty" | "TyCtxt" | "Substs" | "Region" => "type_system".to_string(),
            "BasicBlock" | "Place" | "Operand" => "mir".to_string(),
            "CodegenCx" | "Value" => "codegen".to_string(),
            _ => "general".to_string(),
        }
    }

    /// Print formatted glossary
    pub fn print_glossary(&self) {
        println!("\n📚 RUST COMPILER GLOSSARY");
        println!("Topologically Ordered for Building rustc from Scratch");
        println!("=" .repeat(80));
        
        for (layer_idx, layer) in self.glossary.build_layers.iter().enumerate() {
            println!("\n🏗️  BUILD LAYER {} (Build these first)", layer_idx);
            println!("-" .repeat(50));
            
            for term_name in layer {
                if let Some(term) = self.glossary.terms.iter().find(|t| &t.name == term_name) {
                    self.print_term_entry(term);
                }
            }
        }
        
        println!("\n📊 GLOSSARY SUMMARY:");
        println!("Total terms: {}", self.glossary.total_terms);
        println!("Build layers: {}", self.glossary.build_layers.len());
        println!("Dependency edges: {}", self.glossary.dependency_graph.values().map(|v| v.len()).sum::<usize>());
    }

    fn print_term_entry(&self, term: &RustTerm) {
        println!("\n📖 {}", term.name);
        println!("   Definition: {}", term.value);
        println!("   Usage: {}", term.usage.join(", "));
        println!("   Dependencies: {}", term.dependencies.join(" → "));
        println!("   Monster Factor: {}^{}", term.monster_factor.0, term.monster_factor.1);
        println!("   Category: {}", term.semantic_category);
        
        // Print top 3 eigenvector components
        let mut indexed_vector: Vec<(usize, f64)> = term.eigenvector.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed_vector.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let top_components: Vec<String> = indexed_vector.iter().take(3)
            .filter(|(_, v)| *v > 0.01)
            .map(|(i, v)| format!("p{}:{:.2}", [2,3,5,7,11,13,17,19,23,29,31,41,47,59,71][*i], v))
            .collect();
        
        if !top_components.is_empty() {
            println!("   Eigenvector: [{}]", top_components.join(", "));
        }
    }

    pub fn export_glossary(&self, output_path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.glossary)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| format!("Write failed: {}", e))?;
        
        println!("📄 Rust glossary exported to: {}", output_path);
        Ok(())
    }
}
