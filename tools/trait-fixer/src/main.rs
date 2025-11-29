use rustc_interface::{Queries, interface::Compiler};
use rustc_session::EarlyErrorHandler;
// The clippytask content had `rustc_tools_util::*` which is too broad and causes issues.
// We only need `run_rustc_with_callbacks`.
use rustc_tools_util::run_rustc_with_callbacks;

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

    // For `rustc_driver` to find the sysroot, we often need to set RUSTC_SYSROOT.
    // In a Nix environment, this is usually handled by `rustc_tools_util::run_rustc_with_callbacks`
    // or by wrapping the binary. For development, we might need a fallback.
    // if std::env::var("RUSTC_SYSROOT").is_err() {
    //     eprintln!("Warning: RUSTC_SYSROOT not set. May encounter issues.");
    // }

    run_rustc_with_callbacks(rustc_args, &mut callbacks);
}

struct TraitFixerCallbacks {
    fix_mode: bool,
}

impl rustc_driver::Callbacks for TraitFixerCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        let tcx = queries.global_ctxt().unwrap().peek_mut();
        let mut fixer = crate::visitor::TraitFixer::new(tcx);

        tcx.hir().walk_tops(|item| {
            fixer.check_item(item);
        });

        if self.fix_mode {
            apply_fixes(&fixer.fixes, compiler);
        } else {
            report_fixes(&fixer.fixes);
        }

        rustc_driver::Compilation::Continue
    }
}

fn report_fixes(fixes: &[crate::visitor::Fix]) {
    for fix in fixes {
        match fix {
            crate::visitor::Fix::AddDerive { span: _, trait_name } => {
                println!("Consider adding #[derive({})]", trait_name);
            }
            _ => println!("Fix available"),
        }
    }
}

fn apply_fixes(fixes: &[crate::visitor::Fix], compiler: &Compiler) {
    // This is simplified — real version would use `rustc_ast_pretty` + rewrite
    // Or integrate with `rustfmt` + source rewriting
    println!("--fix mode: {} fixes would be applied", fixes.len());
    // Real implementation: parse source, apply edits, write back with `cargo fix`-like logic
}
