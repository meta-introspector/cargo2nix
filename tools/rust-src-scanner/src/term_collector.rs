/// Comprehensive term collection with 4K page optimization
/// Collects all terms, names, constants with Monster Group factor assignment
use syn::{visit::Visit, *};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const PAGE_SIZE: usize = 4096; // 4K pages
const MONSTER_FACTORS: [(u64, u32); 15] = [
    (2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3),
    (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1),
    (47, 1), (59, 1), (71, 1)
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermCounts {
    pub identifiers: HashMap<String, usize>,
    pub constants: HashMap<String, usize>,
    pub keywords: HashMap<String, usize>,
    pub types: HashMap<String, usize>,
    pub total_terms: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclSummary {
    pub decl_name: String,
    pub decl_type: String, // fn, struct, enum, trait, impl, mod
    pub terms: TermCounts,
    pub assigned_factors: Vec<(u64, u32)>, // prime^exponent
    pub semantic_chunk_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSummary {
    pub file_path: String,
    pub declarations: Vec<DeclSummary>,
    pub file_terms: TermCounts,
    pub file_factors: Vec<(u64, u32)>,
    pub chunks: Vec<SemanticChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChunk {
    pub chunk_id: String,
    pub size_bytes: usize,
    pub terms: TermCounts,
    pub assigned_prime: u64,
    pub semantic_meaning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorySummary {
    pub dir_path: String,
    pub files: Vec<FileSummary>,
    pub dir_terms: TermCounts,
    pub dir_factors: Vec<(u64, u32)>,
    pub total_chunks: usize,
}

pub struct ComprehensiveTermCollector {
    current_terms: TermCounts,
    current_decl: Option<String>,
    factor_allocator: MonsterFactorAllocator,
}

pub struct MonsterFactorAllocator {
    available_factors: HashMap<u64, u32>, // prime -> remaining exponent
    used_factors: HashMap<u64, u32>,      // prime -> used exponent
}

impl MonsterFactorAllocator {
    pub fn new() -> Self {
        let mut available = HashMap::new();
        for (prime, exp) in MONSTER_FACTORS {
            available.insert(prime, exp);
        }
        
        Self {
            available_factors: available,
            used_factors: HashMap::new(),
        }
    }
    
    pub fn allocate_factor(&mut self, term_count: usize, semantic_type: &str) -> (u64, u32) {
        let prime = self.select_prime_for_semantic(semantic_type);
        let needed_exp = self.count_to_exponent(term_count);
        
        let available = self.available_factors.get(&prime).copied().unwrap_or(0);
        let used = self.used_factors.get(&prime).copied().unwrap_or(0);
        
        let actual_exp = needed_exp.min(available);
        
        if actual_exp > 0 {
            self.available_factors.insert(prime, available - actual_exp);
            self.used_factors.insert(prime, used + actual_exp);
        }
        
        (prime, actual_exp)
    }
    
    fn select_prime_for_semantic(&self, semantic_type: &str) -> u64 {
        match semantic_type {
            "rustc_main" => 71,
            "function" => 59,
            "struct" => 47,
            "enum" => 41,
            "trait" => 31,
            "impl" => 29,
            "module" => 23,
            "type" => 19,
            "const" => 17,
            "identifier" => 13,
            "keyword" => 11,
            "literal" => 7,
            "operator" => 5,
            "punctuation" => 3,
            _ => 2, // Default to prime 2 with 46 available factors
        }
    }
    
    fn count_to_exponent(&self, count: usize) -> u32 {
        match count {
            0 => 0,
            1..=4 => 1,
            5..=16 => 2,
            17..=64 => 3,
            65..=256 => 4,
            257..=1024 => 5,
            _ => 6,
        }
    }
}

impl ComprehensiveTermCollector {
    pub fn new() -> Self {
        Self {
            current_terms: TermCounts::new(),
            current_decl: None,
            factor_allocator: MonsterFactorAllocator::new(),
        }
    }
    
    pub fn collect_file_terms(&mut self, file_path: &str, syntax_tree: &syn::File) -> FileSummary {
        let mut file_summary = FileSummary {
            file_path: file_path.to_string(),
            declarations: Vec::new(),
            file_terms: TermCounts::new(),
            file_factors: Vec::new(),
            chunks: Vec::new(),
        };
        
        // Collect terms for each top-level declaration
        for item in &syntax_tree.items {
            let decl_summary = self.collect_declaration_terms(item);
            file_summary.file_terms.merge(&decl_summary.terms);
            file_summary.declarations.push(decl_summary);
        }
        
        // Assign file-level factors
        file_summary.file_factors = self.assign_file_factors(&file_summary.file_terms);
        
        // Create semantic chunks (4K pages)
        file_summary.chunks = self.create_semantic_chunks(&file_summary);
        
        file_summary
    }
    
    fn collect_declaration_terms(&mut self, item: &Item) -> DeclSummary {
        self.current_terms = TermCounts::new();
        
        let (decl_name, decl_type) = self.extract_decl_info(item);
        self.current_decl = Some(decl_name.clone());
        
        // Visit the declaration to collect terms
        self.visit_item(item);
        
        let assigned_factors = self.assign_declaration_factors(&self.current_terms, &decl_type);
        let semantic_chunk_id = self.generate_chunk_id(&decl_name, &assigned_factors);
        
        DeclSummary {
            decl_name,
            decl_type,
            terms: self.current_terms.clone(),
            assigned_factors,
            semantic_chunk_id,
        }
    }
    
    fn extract_decl_info(&self, item: &Item) -> (String, String) {
        match item {
            Item::Fn(f) => (f.sig.ident.to_string(), "fn".to_string()),
            Item::Struct(s) => (s.ident.to_string(), "struct".to_string()),
            Item::Enum(e) => (e.ident.to_string(), "enum".to_string()),
            Item::Trait(t) => (t.ident.to_string(), "trait".to_string()),
            Item::Impl(i) => ("impl".to_string(), "impl".to_string()),
            Item::Mod(m) => (m.ident.as_ref().map(|i| i.to_string()).unwrap_or_else(|| "mod".to_string()), "mod".to_string()),
            Item::Type(t) => (t.ident.to_string(), "type".to_string()),
            Item::Const(c) => (c.ident.to_string(), "const".to_string()),
            Item::Static(s) => (s.ident.to_string(), "static".to_string()),
            _ => ("unknown".to_string(), "unknown".to_string()),
        }
    }
    
    fn assign_declaration_factors(&mut self, terms: &TermCounts, decl_type: &str) -> Vec<(u64, u32)> {
        let mut factors = Vec::new();
        
        // Clone terms to avoid borrow checker issues
        let terms_clone = terms.clone();
        
        // Assign factors based on term counts and declaration type
        if terms_clone.identifiers.len() > 0 {
            factors.push(self.factor_allocator.allocate_factor(terms_clone.identifiers.len(), "identifier"));
        }
        if terms_clone.constants.len() > 0 {
            factors.push(self.factor_allocator.allocate_factor(terms_clone.constants.len(), "const"));
        }
        if terms_clone.keywords.len() > 0 {
            factors.push(self.factor_allocator.allocate_factor(terms_clone.keywords.len(), "keyword"));
        }
        if terms_clone.types.len() > 0 {
            factors.push(self.factor_allocator.allocate_factor(terms_clone.types.len(), "type"));
        }
        
        // Add semantic factor for declaration type
        factors.push(self.factor_allocator.allocate_factor(1, decl_type));
        
        factors
    }
    
    fn assign_file_factors(&mut self, file_terms: &TermCounts) -> Vec<(u64, u32)> {
        vec![
            self.factor_allocator.allocate_factor(file_terms.total_terms, "file"),
        ]
    }
    
    fn create_semantic_chunks(&mut self, file_summary: &FileSummary) -> Vec<SemanticChunk> {
        let mut chunks = Vec::new();
        let mut current_size = 0;
        let mut chunk_terms = TermCounts::new();
        let mut chunk_counter = 0;
        
        for decl in &file_summary.declarations {
            let decl_size = self.estimate_decl_size(&decl.terms);
            
            if current_size + decl_size > PAGE_SIZE && current_size > 0 {
                // Create chunk
                let chunk = self.finalize_chunk(chunk_counter, current_size, chunk_terms);
                chunks.push(chunk);
                
                // Reset for next chunk
                chunk_counter += 1;
                current_size = 0;
                chunk_terms = TermCounts::new();
            }
            
            current_size += decl_size;
            chunk_terms.merge(&decl.terms);
        }
        
        // Final chunk
        if current_size > 0 {
            chunks.push(self.finalize_chunk(chunk_counter, current_size, chunk_terms));
        }
        
        chunks
    }
    
    fn finalize_chunk(&mut self, chunk_id: usize, size: usize, terms: TermCounts) -> SemanticChunk {
        let (prime, _) = self.factor_allocator.allocate_factor(terms.total_terms, "chunk");
        let semantic_meaning = self.derive_semantic_meaning(&terms, prime);
        
        SemanticChunk {
            chunk_id: format!("chunk_{}", chunk_id),
            size_bytes: size,
            terms,
            assigned_prime: prime,
            semantic_meaning,
        }
    }
    
    fn derive_semantic_meaning(&self, terms: &TermCounts, prime: u64) -> String {
        let dominant_category = if terms.keywords.len() > terms.identifiers.len() {
            "control_flow"
        } else if terms.types.len() > terms.constants.len() {
            "type_definitions"
        } else if terms.constants.len() > 0 {
            "data_constants"
        } else {
            "general_code"
        };
        
        format!("{}_{}", dominant_category, prime)
    }
    
    fn estimate_decl_size(&self, terms: &TermCounts) -> usize {
        // Rough estimate: 10 bytes per term on average
        terms.total_terms * 10
    }
    
    fn generate_chunk_id(&self, decl_name: &str, factors: &[(u64, u32)]) -> String {
        let factor_sum: u64 = factors.iter().map(|(p, e)| p * (*e as u64)).sum();
        format!("{}_{}", decl_name, factor_sum % 1000)
    }
}

impl TermCounts {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            constants: HashMap::new(),
            keywords: HashMap::new(),
            types: HashMap::new(),
            total_terms: 0,
        }
    }
    
    pub fn merge(&mut self, other: &TermCounts) {
        for (k, v) in &other.identifiers {
            *self.identifiers.entry(k.clone()).or_insert(0) += v;
        }
        for (k, v) in &other.constants {
            *self.constants.entry(k.clone()).or_insert(0) += v;
        }
        for (k, v) in &other.keywords {
            *self.keywords.entry(k.clone()).or_insert(0) += v;
        }
        for (k, v) in &other.types {
            *self.types.entry(k.clone()).or_insert(0) += v;
        }
        self.total_terms += other.total_terms;
    }
}

impl<'ast> Visit<'ast> for ComprehensiveTermCollector {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        let name = ident.to_string();
        *self.current_terms.identifiers.entry(name).or_insert(0) += 1;
        self.current_terms.total_terms += 1;
    }
    
    fn visit_lit(&mut self, lit: &'ast Lit) {
        let value = match lit {
            Lit::Str(s) => format!("\"{}\"", s.value()),
            Lit::Int(i) => i.base10_digits().to_string(),
            Lit::Float(f) => f.base10_digits().to_string(),
            Lit::Bool(b) => b.value.to_string(),
            _ => "literal".to_string(),
        };
        *self.current_terms.constants.entry(value).or_insert(0) += 1;
        self.current_terms.total_terms += 1;
        syn::visit::visit_lit(self, lit);
    }
    
    fn visit_type(&mut self, ty: &'ast Type) {
        let type_name = match ty {
            Type::Path(p) => p.path.segments.last().map_or("path".to_string(), |s| s.ident.to_string()),
            Type::Reference(_) => "ref".to_string(),
            Type::Ptr(_) => "ptr".to_string(),
            Type::Array(_) => "array".to_string(),
            Type::Slice(_) => "slice".to_string(),
            Type::Tuple(_) => "tuple".to_string(),
            _ => "type".to_string(),
        };
        *self.current_terms.types.entry(type_name).or_insert(0) += 1;
        self.current_terms.total_terms += 1;
        syn::visit::visit_type(self, ty);
    }
}
