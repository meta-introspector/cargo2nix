use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::error::AppError;
use crate::results::CompilationResult;
use crate::traits::Compiler;
use crate::config::CompilerConfig;
use crate::state_manager::State;
use crate::hasher::calculate_file_hash;
use crate::semantic_constraints::{GödelNumber, SemanticSolver};
use crate::goal_state::{GoalState, CompilerState};
use crate::monster_group::{MonsterRustc, RustcComponent};
use crate::modular_forms::{RamanujanTau, HeckeAlgebra};

pub trait CompilationOrchestrator {
    fn compile_crate_with_cache(
        &self,
        crate_root_path: &Path,
        compiler: &dyn Compiler,
        config: &CompilerConfig,
        crate_name_to_root_map: &HashMap<String, PathBuf>,
        compiled_artifacts_map: &mut HashMap<String, PathBuf>,
        state_arc: &Arc<Mutex<State>>,
        dry_run: bool,
    ) -> Result<CompilationResult, AppError>;
}

pub struct DefaultCompilationOrchestrator {
    semantic_solver: SemanticSolver,
    goal_state: GoalState,
    compiler_state: CompilerState,
    monster_rustc: MonsterRustc,
    hecke_algebra: HeckeAlgebra,
}

impl DefaultCompilationOrchestrator {
    pub fn new() -> Self {
        Self {
            semantic_solver: SemanticSolver::new(),
            goal_state: GoalState::rust_compiler_bootstrap(),
            compiler_state: CompilerState::new(),
            monster_rustc: MonsterRustc::new(),
            hecke_algebra: HeckeAlgebra::new(),
        }
    }
}

impl CompilationOrchestrator for DefaultCompilationOrchestrator {
    fn compile_crate_with_cache(
        &self,
        crate_root_path: &Path,
        compiler: &dyn Compiler,
        config: &CompilerConfig,
        crate_name_to_root_map: &HashMap<String, PathBuf>,
        compiled_artifacts_map: &mut HashMap<String, PathBuf>,
        state_arc: &Arc<Mutex<State>>,
        dry_run: bool,
    ) -> Result<CompilationResult, AppError> {
        let file_hash = calculate_file_hash(&crate_root_path.join("Cargo.toml"))?;

        // Generate Monster Group factor ID for this crate
        let crate_index = crate_root_path.to_string_lossy().len() as u64;
        let factor_id = (crate_index % 108) as u32 + 1; // Map to 1-108 range
        let semantic_hash = GödelNumber::from_index(crate_index);

        // Determine rustc component type from crate name
        let crate_name = crate_root_path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let component = classify_rustc_component(&crate_name);

        // Apply Ramanujan τ(n) for structural invariants
        let tau_value = RamanujanTau::tau((factor_id % 20) as usize);
        println!("Crate {} -> Factor {} -> τ({}) = {}", 
                crate_name, factor_id, factor_id % 20, tau_value);

        // Check cache
        {
            let current_state = state_arc.lock().unwrap();
            if let Some(cached_result) = current_state.cache.get(crate_root_path) {
                if cached_result.source_checksum == file_hash {
                    println!("Cache hit for {:?} (Monster factor {})", 
                            crate_root_path.display(), factor_id);
                    
                    if let Some(ref rlib_path_str) = cached_result.compiled_checksum {
                        let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                        compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
                    }
                    
                    return Ok(cached_result.clone());
                }
            }
        }

        if dry_run {
            println!("Dry run: Monster factor {} ({})", factor_id, component_name(&component));
            return Ok(create_dry_run_result(crate_root_path, semantic_hash, factor_id));
        }

        let compilation_result = compiler.compile_crate(
            crate_root_path, 
            config, 
            crate_name_to_root_map, 
            compiled_artifacts_map
        )?;

        // Update Monster Group state on successful compilation
        if compilation_result.exit_code == 0 {
            // Note: In real implementation, we'd need mutable access to monster_rustc
            println!("Successfully compiled Monster factor {} ({})", factor_id, component_name(&component));
            
            if let Some(ref rlib_path_str) = compilation_result.compiled_checksum {
                let crate_name = crate_root_path.file_name().unwrap().to_string_lossy().to_string();
                compiled_artifacts_map.insert(crate_name, PathBuf::from(rlib_path_str));
            }
        }

        // Update cache
        {
            let mut current_state = state_arc.lock().unwrap();
            current_state.cache.insert(crate_root_path.to_path_buf(), compilation_result.clone());
        }

        Ok(compilation_result)
    }
}

fn classify_rustc_component(crate_name: &str) -> RustcComponent {
    match crate_name {
        name if name.contains("rustc_driver") => RustcComponent::BuildSystem,
        name if name.contains("rustc_resolve") => RustcComponent::DependencyResolver,
        name if name.contains("rustc_ast") => RustcComponent::ASTParser,
        name if name.contains("rustc_hir") => RustcComponent::TypeChecker,
        name if name.contains("rustc_borrowck") => RustcComponent::BorrowChecker,
        name if name.contains("rustc_mir") => RustcComponent::MIRBuilder,
        name if name.contains("rustc_codegen") => RustcComponent::CodeGen,
        name if name.contains("rustc_link") => RustcComponent::Linker,
        name if name.contains("rustc") => RustcComponent::BootstrapCore,
        _ => RustcComponent::Optimizer,
    }
}

fn component_name(component: &RustcComponent) -> &'static str {
    match component {
        RustcComponent::BuildSystem => "Build System",
        RustcComponent::DependencyResolver => "Dependency Resolver", 
        RustcComponent::ASTParser => "AST Parser",
        RustcComponent::TypeChecker => "Type Checker",
        RustcComponent::BorrowChecker => "Borrow Checker",
        RustcComponent::MIRBuilder => "MIR Builder",
        RustcComponent::Optimizer => "Optimizer",
        RustcComponent::CodeGen => "Code Generator",
        RustcComponent::Linker => "Linker",
        RustcComponent::BootstrapCore => "Bootstrap Core",
    }
}

fn create_dry_run_result(crate_root_path: &Path, semantic_hash: GödelNumber, factor_id: u32) -> CompilationResult {
    CompilationResult {
        success: true,
        output: format!("Dry run - Monster factor {} with semantic hash: {:?}", factor_id, semantic_hash),
        error: None,
        file_path: crate_root_path.to_path_buf(),
        rustc_version: "monster-group-dry-run".to_string(),
        rustc_flags: vec![format!("--monster-factor={}", factor_id)],
        stdout: format!("Monster Group factor {} compiled", factor_id),
        stderr: String::new(),
        exit_code: 0,
        duration_ms: 0,
        source_checksum: String::new(),
        compiled_checksum: Some(format!("monster-factor-{}.rlib", factor_id)),
        timestamp: chrono::Utc::now(),
    }
}

pub fn handle_compilation_result(
    compilation_result: &CompilationResult,
    crate_root_path: &Path,
) -> Result<(), AppError> {
    let exit_code = compilation_result.exit_code;
    
    if exit_code != 0 {
        println!("Monster Group compilation failed for {:?}", crate_root_path.display());
        println!("--- STDOUT ---");
        println!("{}", compilation_result.stdout);
        println!("--- STDERR ---");
        println!("{}", compilation_result.stderr);
        return Err(AppError::Custom(format!("Monster compilation failed for {:?}", crate_root_path.display())));
    } else {
        println!("Monster Group compilation successful for {:?}", crate_root_path.display());
    }
    
    Ok(())
}
