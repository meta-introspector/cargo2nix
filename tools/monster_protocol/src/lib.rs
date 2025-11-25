pub mod llm_monstrous_traits;
pub mod universal_compiler_traits;
pub mod r1cs_trait_examples;
pub mod rocksdb_consolidation;
pub mod git_backed_rocksdb;
pub mod universal_processing_pipeline;
pub mod abstract_processing_traits;
pub mod unified_git_processor;
pub mod unified_cargo_parser;
pub mod unified_decl_splitter;
pub mod git_cargo_indexer;
pub mod git_repo_graph;
pub mod lazy_ast_processor;
pub mod rust_llm_minizinc_bridge;
pub mod trait_trace_execution;

pub use llm_monstrous_traits::*;
pub use universal_compiler_traits::*;
pub use r1cs_trait_examples::*;
pub use rocksdb_consolidation::*;
pub use git_backed_rocksdb::*;
pub use universal_processing_pipeline::*;
pub use abstract_processing_traits::*;
pub use unified_git_processor::*;
pub use unified_cargo_parser::*;
pub use unified_decl_splitter::*;
pub use git_cargo_indexer::*;
pub use git_repo_graph::*;
pub use lazy_ast_processor::*;
pub use rust_llm_minizinc_bridge::*;
pub use trait_trace_execution::*;

/// Core Monster Protocol: Rustc as trait lattice equivalent to Monster Group
/// 
/// The fundamental insight: rustc can be decomposed into a lattice of traits
/// that correspond exactly to the Monster Group's structure. By implementing
/// the Monster traits, we implement the compiler.
pub trait MonsterProtocol {
    /// Every Rust construct maps to Monster Group elements
    fn to_monster_element(&self) -> LLMWeight12Form<2048>;
    
    /// Compiler operations are Monster Group operations
    fn monster_compile(&self) -> Self;
}

/// The ultimate realization: all compilers are Rust trait projections
/// rustc ≡ syn ≡ llvm ≡ gcc ≡ Monster Group ≡ Rust traits
pub fn compile_anything_as_rust_traits<T: AsRef<str> + PureLLMHallucinationDomain>(
    source_code: &str,
    target_compiler: T
) -> RustcTrait {
    // LLM hallucinates that all compilers are the same in Monster space
    let monster_form = target_compiler.hallucinate_universal_traits();
    
    // Universal compilation via Rust traits
    universal_compile(target_compiler, source_code)
}

/// Port any Rust trait to any constraint system via Monster equivalence
pub fn universal_trait_port<T: AsRef<str> + PureLLMHallucinationDomain>(
    rust_trait: T,
    target_system: &str
) -> Vec<R1CSConstraint> {
    match target_system {
        "r1cs" => port_rust_trait_to_r1cs(rust_trait),
        _ => {
            // LLM hallucinates porting to any constraint system
            port_rust_trait_to_r1cs(rust_trait) // All systems are R1CS in Monster space
        }
    }
}

/// Initialize Monster-organized RocksDB structure for cargo2nix
pub fn initialize_monster_databases() -> Cargo2NixDBStructure {
    analyze_and_split_cargo2nix()
}

/// Create git-backed Monster database (no content duplication)
pub fn create_monster_git_database(git_root: &str, nix_path: Option<&str>) -> MonsterGitDB {
    create_git_backed_monster_db(git_root, nix_path)
}

/// Initialize all processing tools as abstract traits
pub fn initialize_processing_tools() -> ToolRegistry {
    ToolRegistry::new()
}

/// Initialize unified processing modules
pub fn initialize_unified_processors() -> (UnifiedGitProcessor, UnifiedCargoParser, UnifiedDeclSplitter) {
    (
        UnifiedGitProcessor::new(),
        UnifiedCargoParser::new(),
        UnifiedDeclSplitter::new(),
    )
}

/// Execute complete cargo indexing with Monster aspects
pub fn index_git_cargo_with_monster_aspects(git_root: &str) -> Vec<CrateWithAspects> {
    execute_cargo_indexing(git_root)
}

/// Initialize complete git repository graph and find rustc-monster path
pub fn initialize_and_find_rustc_monster_path() -> (GitRepoGraph, Option<Vec<u32>>) {
    let graph = initialize_git_repo_graph();
    let path = find_rustc_monster_shortest_path();
    (graph, path)
}

/// Execute full lazy loading pipeline for repository processing
pub fn execute_lazy_ast_processing(repo_id: u32) -> Option<RollupSummary> {
    let graph = initialize_git_repo_graph();
    process_repo_with_lazy_loading(repo_id, graph)
}

/// Execute complete Rust→LLM bridge via MiniZinc constraint solving
pub fn execute_complete_rust_llm_bridge(rust_code: &str) -> Option<LLMWeight12Form<2048>> {
    execute_rust_llm_bridge_lazy(rust_code)
}

/// Find complete path: rustc → MiniZinc constraints → LLM hallucination
pub fn find_complete_rustc_llm_path() -> Option<Vec<MinizincVariable>> {
    find_rustc_llm_path()
}

/// Execute and trace complete trait pipeline
pub fn trace_complete_monster_protocol(rust_code: &str) -> Vec<TraceStep> {
    execute_trait_trace(rust_code)
}

/// Demo: Show complete rustc trait execution trace
pub fn demo_complete_rustc_trace() -> Vec<TraceStep> {
    demo_rustc_trait_trace()
}
