use crate::cli::Args;
use crate::app_context::AppContext;
use crate::layer_manager::{LayerManager, FileSystemLayerManager, apply_limit};
use crate::compilation_orchestrator::{CompilationOrchestrator, DefaultCompilationOrchestrator, handle_compilation_result};
use crate::compiler::RustcCompilerImpl;
use crate::error::AppError;

pub struct AppRunner;

impl AppRunner {
    pub fn run(context: AppContext, args: Args) -> Result<(), AppError> {
        let compiler = RustcCompilerImpl;
        let orchestrator = DefaultCompilationOrchestrator::new();
        let layer_manager = FileSystemLayerManager;
        
        let max_layer = context.get_max_layer();
        println!("Max layer found: {}", max_layer);
        
        let mut total_crates_processed_overall = 0;
        let mut context = context; // Make mutable
        
        for current_layer in 0..=max_layer {
            println!("\n--- Processing Layer {} ---", current_layer);
            
            let crates_in_layer = layer_manager.get_crates_for_layer(
                current_layer,
                &context.layered_crates,
                &context.crate_name_to_root_map,
            );
            
            let total_crates_in_layer = crates_in_layer.len();
            if total_crates_in_layer == 0 {
                println!("No crates to process in Layer {}.", current_layer);
                continue;
            }
            
            let limited_crates = apply_limit(crates_in_layer, args.limit);
            
            let mut crates_processed_in_layer = 0;
            for crate_root_path in limited_crates {
                crates_processed_in_layer += 1;
                total_crates_processed_overall += 1;
                
                let progress_percent = (crates_processed_in_layer as f64 / total_crates_in_layer as f64) * 100.0;
                println!(
                    "Compiling crate {} of {} in Layer {} ({:.2}%): {:?}",
                    crates_processed_in_layer,
                    total_crates_in_layer,
                    current_layer,
                    progress_percent,
                    crate_root_path.display()
                );
                
                let compilation_result = orchestrator.compile_crate_with_cache(
                    &crate_root_path,
                    &compiler,
                    &context.config,
                    &context.crate_name_to_root_map,
                    &mut context.compiled_artifacts_map,
                    &context.state,
                    args.dry_run,
                )?;
                
                handle_compilation_result(&compilation_result, &crate_root_path)?;
            }
        }
        
        println!("Total crates processed: {}", total_crates_processed_overall);
        Ok(())
    }
}
