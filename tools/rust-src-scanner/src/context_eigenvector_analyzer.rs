/// Bidirectional Context Tagging and Eigenvector Analysis
/// Contents influence container, container influences declarations
/// Module usage functions as eigenvectors
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualDeclaration {
    pub decl_name: String,
    pub decl_type: String,
    pub content_tags: Vec<String>,
    pub container_context: ContainerContext,
    pub influenced_by_container: f64, // Weight 0.0-1.0
    pub influences_container: f64,    // Weight 0.0-1.0
    pub usage_vector: Vec<f64>,       // Eigenvector representation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerContext {
    pub file_path: String,
    pub module_path: String,
    pub sibling_decls: Vec<String>,
    pub parent_context: Option<String>,
    pub semantic_domain: String, // "error_handling", "type_system", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUsageMatrix {
    pub modules: Vec<String>,
    pub usage_matrix: Vec<Vec<f64>>, // Module x Module usage relationships
    pub eigenvalues: Vec<f64>,
    pub eigenvectors: Vec<Vec<f64>>,
}

pub struct ContextEigenvectorAnalyzer {
    pub declarations: HashMap<String, ContextualDeclaration>,
    pub containers: HashMap<String, ContainerContext>,
    pub usage_matrix: ModuleUsageMatrix,
}

impl ContextEigenvectorAnalyzer {
    pub fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            containers: HashMap::new(),
            usage_matrix: ModuleUsageMatrix {
                modules: Vec::new(),
                usage_matrix: Vec::new(),
                eigenvalues: Vec::new(),
                eigenvectors: Vec::new(),
            },
        }
    }

    /// Bidirectional context tagging: content ↔ container influence
    pub fn analyze_bidirectional_context(&mut self, file_path: &str, syntax_tree: &syn::File) {
        let container_context = self.extract_container_context(file_path, syntax_tree);
        
        for item in &syntax_tree.items {
            let mut decl = self.extract_declaration_context(item);
            
            // Container influences declaration
            decl.influenced_by_container = self.calculate_container_influence(&decl, &container_context);
            decl.container_context = container_context.clone();
            
            // Declaration influences container (feedback)
            let container_influence = self.calculate_declaration_influence(&decl);
            self.update_container_influence(file_path, container_influence);
            
            // Generate usage vector for this declaration
            decl.usage_vector = self.generate_usage_vector(&decl);
            
            self.declarations.insert(decl.decl_name.clone(), decl);
        }
        
        self.containers.insert(file_path.to_string(), container_context);
    }

    fn extract_container_context(&self, file_path: &str, syntax_tree: &syn::File) -> ContainerContext {
        let semantic_domain = self.infer_semantic_domain(file_path, syntax_tree);
        let sibling_decls = syntax_tree.items.iter()
            .filter_map(|item| self.extract_item_name(item))
            .collect();

        ContainerContext {
            file_path: file_path.to_string(),
            module_path: self.extract_module_path(file_path),
            sibling_decls,
            parent_context: self.extract_parent_context(file_path),
            semantic_domain,
        }
    }

    fn extract_declaration_context(&self, item: &syn::Item) -> ContextualDeclaration {
        let (decl_name, decl_type) = self.extract_decl_info(item);
        let content_tags = self.extract_content_tags(item);

        ContextualDeclaration {
            decl_name,
            decl_type,
            content_tags,
            container_context: ContainerContext {
                file_path: String::new(),
                module_path: String::new(),
                sibling_decls: Vec::new(),
                parent_context: None,
                semantic_domain: String::new(),
            },
            influenced_by_container: 0.0,
            influences_container: 0.0,
            usage_vector: Vec::new(),
        }
    }

    fn calculate_container_influence(&self, decl: &ContextualDeclaration, container: &ContainerContext) -> f64 {
        let mut influence = 0.0;
        
        // Semantic domain alignment
        if decl.content_tags.iter().any(|tag| container.semantic_domain.contains(tag)) {
            influence += 0.4;
        }
        
        // Sibling declaration influence
        let sibling_overlap = decl.content_tags.iter()
            .filter(|tag| container.sibling_decls.iter().any(|sibling| sibling.contains(*tag)))
            .count() as f64;
        influence += (sibling_overlap / decl.content_tags.len().max(1) as f64) * 0.3;
        
        // Module path influence
        if container.module_path.contains(&decl.decl_type) {
            influence += 0.3;
        }
        
        influence.min(1.0)
    }

    fn calculate_declaration_influence(&self, decl: &ContextualDeclaration) -> f64 {
        let mut influence = 0.0;
        
        // High-impact declarations influence container more
        match decl.decl_type.as_str() {
            "trait" => influence += 0.8,  // Traits define interfaces
            "enum" => influence += 0.6,   // Enums define data structures
            "struct" => influence += 0.5, // Structs define data
            "fn" => influence += 0.3,     // Functions implement behavior
            _ => influence += 0.1,
        }
        
        // Complex content increases influence
        influence += (decl.content_tags.len() as f64 * 0.05).min(0.4);
        
        influence.min(1.0)
    }

    fn generate_usage_vector(&self, decl: &ContextualDeclaration) -> Vec<f64> {
        // Generate eigenvector representation based on:
        // 1. Content complexity
        // 2. Container influence
        // 3. Semantic domain
        // 4. Declaration type
        
        let mut vector = vec![0.0; 15]; // 15 Monster Group primes
        
        // Map declaration type to vector components
        match decl.decl_type.as_str() {
            "fn" => vector[14] = 1.0,      // Prime 71
            "struct" => vector[13] = 1.0,  // Prime 59
            "enum" => vector[12] = 1.0,    // Prime 47
            "trait" => vector[11] = 1.0,   // Prime 41
            "impl" => vector[10] = 1.0,    // Prime 31
            _ => vector[0] = 1.0,          // Prime 2
        }
        
        // Weight by container influence
        for i in 0..vector.len() {
            vector[i] *= decl.influenced_by_container;
        }
        
        // Add content complexity
        let complexity = decl.content_tags.len() as f64 / 10.0;
        vector[1] = complexity.min(1.0); // Prime 3 for complexity
        
        vector
    }

    /// Build module usage matrix for eigenvector analysis
    pub fn build_usage_matrix(&mut self) {
        let modules: Vec<String> = self.containers.keys().cloned().collect();
        let n = modules.len();
        let mut matrix = vec![vec![0.0; n]; n];
        
        // Calculate module-to-module usage relationships
        for (i, module_a) in modules.iter().enumerate() {
            for (j, module_b) in modules.iter().enumerate() {
                if i != j {
                    matrix[i][j] = self.calculate_module_usage(module_a, module_b);
                }
            }
        }
        
        self.usage_matrix = ModuleUsageMatrix {
            modules,
            usage_matrix: matrix,
            eigenvalues: Vec::new(),
            eigenvectors: Vec::new(),
        };
    }

    fn calculate_module_usage(&self, module_a: &str, module_b: &str) -> f64 {
        // Calculate usage relationship between modules based on:
        // 1. Shared semantic domains
        // 2. Declaration cross-references
        // 3. Container influence patterns
        
        let container_a = self.containers.get(module_a);
        let container_b = self.containers.get(module_b);
        
        if let (Some(a), Some(b)) = (container_a, container_b) {
            let mut usage = 0.0;
            
            // Semantic domain similarity
            if a.semantic_domain == b.semantic_domain {
                usage += 0.5;
            }
            
            // Declaration name overlap
            let overlap = a.sibling_decls.iter()
                .filter(|decl| b.sibling_decls.contains(decl))
                .count() as f64;
            usage += overlap / (a.sibling_decls.len().max(1) as f64) * 0.3;
            
            // Module path similarity
            let path_similarity = self.calculate_path_similarity(&a.module_path, &b.module_path);
            usage += path_similarity * 0.2;
            
            usage.min(1.0)
        } else {
            0.0
        }
    }

    /// Compute eigenvalues and eigenvectors (simplified power iteration)
    pub fn compute_eigenvectors(&mut self) {
        let matrix = &self.usage_matrix.usage_matrix;
        let n = matrix.len();
        
        if n == 0 {
            return;
        }
        
        // Power iteration for dominant eigenvector
        let mut eigenvector = vec![1.0; n];
        let mut eigenvalue = 0.0;
        
        for _ in 0..100 { // 100 iterations
            let mut new_vector = vec![0.0; n];
            
            // Matrix-vector multiplication
            for i in 0..n {
                for j in 0..n {
                    new_vector[i] += matrix[i][j] * eigenvector[j];
                }
            }
            
            // Normalize
            let norm = new_vector.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                for i in 0..n {
                    new_vector[i] /= norm;
                }
                eigenvalue = norm;
            }
            
            eigenvector = new_vector;
        }
        
        self.usage_matrix.eigenvalues = vec![eigenvalue];
        self.usage_matrix.eigenvectors = vec![eigenvector];
    }

    pub fn print_eigenvector_analysis(&self) {
        println!("🔢 Module Usage Eigenvector Analysis");
        println!("=" .repeat(50));
        
        if let Some(dominant_eigenvector) = self.usage_matrix.eigenvectors.first() {
            println!("Dominant eigenvector (module importance):");
            for (i, &value) in dominant_eigenvector.iter().enumerate() {
                if let Some(module) = self.usage_matrix.modules.get(i) {
                    let bar = "█".repeat((value * 50.0) as usize);
                    println!("  {:30} : {:.3} {}", module, value, bar);
                }
            }
        }
        
        if let Some(&eigenvalue) = self.usage_matrix.eigenvalues.first() {
            println!("\nDominant eigenvalue: {:.3}", eigenvalue);
        }
        
        println!("\n📊 Bidirectional Context Summary:");
        let mut high_influence_decls = 0;
        let mut container_influenced_decls = 0;
        
        for decl in self.declarations.values() {
            if decl.influences_container > 0.5 {
                high_influence_decls += 1;
            }
            if decl.influenced_by_container > 0.5 {
                container_influenced_decls += 1;
            }
        }
        
        println!("  High-influence declarations: {}", high_influence_decls);
        println!("  Container-influenced declarations: {}", container_influenced_decls);
        println!("  Total declarations analyzed: {}", self.declarations.len());
    }

    // Helper methods
    fn infer_semantic_domain(&self, file_path: &str, _syntax_tree: &syn::File) -> String {
        if file_path.contains("error") { "error_handling".to_string() }
        else if file_path.contains("ty") || file_path.contains("type") { "type_system".to_string() }
        else if file_path.contains("mir") { "mir_analysis".to_string() }
        else if file_path.contains("hir") { "hir_analysis".to_string() }
        else if file_path.contains("ast") { "ast_processing".to_string() }
        else if file_path.contains("codegen") { "code_generation".to_string() }
        else { "general".to_string() }
    }

    fn extract_module_path(&self, file_path: &str) -> String {
        file_path.replace("/", "::").replace(".rs", "")
    }

    fn extract_parent_context(&self, file_path: &str) -> Option<String> {
        std::path::Path::new(file_path).parent()?.to_str().map(|s| s.to_string())
    }

    fn extract_item_name(&self, item: &syn::Item) -> Option<String> {
        match item {
            syn::Item::Fn(f) => Some(f.sig.ident.to_string()),
            syn::Item::Struct(s) => Some(s.ident.to_string()),
            syn::Item::Enum(e) => Some(e.ident.to_string()),
            syn::Item::Trait(t) => Some(t.ident.to_string()),
            _ => None,
        }
    }

    fn extract_decl_info(&self, item: &syn::Item) -> (String, String) {
        match item {
            syn::Item::Fn(f) => (f.sig.ident.to_string(), "fn".to_string()),
            syn::Item::Struct(s) => (s.ident.to_string(), "struct".to_string()),
            syn::Item::Enum(e) => (e.ident.to_string(), "enum".to_string()),
            syn::Item::Trait(t) => (t.ident.to_string(), "trait".to_string()),
            syn::Item::Impl(_) => ("impl".to_string(), "impl".to_string()),
            _ => ("unknown".to_string(), "unknown".to_string()),
        }
    }

    fn extract_content_tags(&self, item: &syn::Item) -> Vec<String> {
        let mut tags = Vec::new();
        
        match item {
            syn::Item::Fn(f) => {
                tags.push("function".to_string());
                if f.sig.asyncness.is_some() { tags.push("async".to_string()); }
                if f.sig.unsafety.is_some() { tags.push("unsafe".to_string()); }
            }
            syn::Item::Struct(_) => tags.push("data_structure".to_string()),
            syn::Item::Enum(_) => tags.push("variant_type".to_string()),
            syn::Item::Trait(_) => tags.push("interface".to_string()),
            syn::Item::Impl(_) => tags.push("implementation".to_string()),
            _ => {}
        }
        
        tags
    }

    fn update_container_influence(&mut self, _file_path: &str, _influence: f64) {
        // Update container context based on declaration influence
        // This creates the bidirectional feedback loop
    }

    fn calculate_path_similarity(&self, path_a: &str, path_b: &str) -> f64 {
        let parts_a: Vec<&str> = path_a.split("::").collect();
        let parts_b: Vec<&str> = path_b.split("::").collect();
        
        let common_parts = parts_a.iter()
            .zip(parts_b.iter())
            .take_while(|(a, b)| a == b)
            .count();
        
        common_parts as f64 / parts_a.len().max(parts_b.len()) as f64
    }
}
