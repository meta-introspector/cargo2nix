// tools/trait-fixer/src/main.rs

use rustc_driver; // Always needed for the Callbacks trait

// Import common trait definitions
use trait_fixer_compiler_host_trait::CompilerHost;
use trait_fixer_core_trait::{CoreFixer, Fix};
use trait_fixer_rules_trait::ConfigTrait;

// Conditionally import the concrete implementations and rustc types
#[cfg(feature = "use_real_impls")]
mod real_impls {
    pub use rustc_interface::interface::Compiler;
    pub use rustc_middle::ty::TyCtxt;
    pub use rustc_driver::Compilation;

    pub use crate::trait_fixer_compiler_host_real::ActualCompilerHost;
    pub use crate::trait_fixer_core_real::TraitFixer as CoreFixerImpl;
    pub use crate::trait_fixer_rules_real::Config as RulesConfigImpl;
}

#[cfg(feature = "use_mock_impls")]
mod mock_impls {
    pub use trait_fixer_rustc_mock::{MockCompiler as Compiler, TyCtxt, MockCompilation as Compilation};
    pub use crate::trait_fixer_compiler_host_mock::MockCompilerHost as ActualCompilerHost;
    pub use crate::trait_fixer_core_mock::MockTraitFixer as CoreFixerImpl;
    pub use crate::trait_fixer_rules_mock::MockConfig as RulesConfigImpl;
}

// Re-export chosen implementations and types
#[cfg(feature = "use_real_impls")]
use real_impls::*;
#[cfg(feature = "use_mock_impls")]
use mock_impls::*;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: trait-fixer <crate> [--fix]");
        return;
    }

    let fix_mode = args.contains(&"--fix".to_string());
    if fix_mode {
        args.retain(|a| a != "--fix");
    }

    let mut callbacks = TraitFixerCallbacks { fix_mode };

    let mut rustc_args = vec!["rustc".to_string()];
    rustc_args.extend(args.into_iter().skip(1));

    // Instantiate the compiler host based on feature
    #[cfg(feature = "use_real_impls")]
    let compiler_host = ActualCompilerHost;
    #[cfg(feature = "use_mock_impls")]
    let compiler_host = ActualCompilerHost;

    compiler_host.run_compiler_callbacks(rustc_args, &mut callbacks);
}

struct TraitFixerCallbacks {
    fix_mode: bool,
}

impl rustc_driver::Callbacks for TraitFixerCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler, // This Compiler type is now resolved by the feature flag (real/mock)
        tcx: TyCtxt<'tcx>, // This TyCtxt type is now resolved by the feature flag (real/mock)
    ) -> Compilation { // This Compilation type is now resolved by the feature flag (real/mock)
        // Instantiate the CoreFixerImpl, which uses the correct TyCtxt from the feature flag
        let mut fixer = CoreFixerImpl::new(tcx);

        // This is where the HIR walking and item checking happens
        // This part needs to be made generic or conditional as well,
        // as tcx.hir().walk_tops might have different mock implementations.
        // For now, let's make a temporary call to process_hir which will be implemented later.
        fixer.process_hir();

        if self.fix_mode {
            apply_fixes(fixer.get_fixes(), _compiler);
        } else {
            report_fixes(fixer.get_fixes());
        }

        Compilation::Continue
    }
}

fn report_fixes(fixes: &[Fix]) {
    for fix in fixes {
        match fix {
            Fix::AddDerive { span: _, trait_name } => {
                println!("Consider adding #[derive({})]", trait_name);
            }
            _ => println!("Fix available"),
        }
    }
}

fn apply_fixes(fixes: &[Fix], _compiler: &Compiler) { // Compiler type is conditional
    println!("--fix mode: {} fixes would be applied", fixes.len());
}
