macro_wrapper_lib::MODULE_HEADER!();

macro_wrapper_lib::wrap_use! {
    use crate::gemini_compiler_host;
    use rustc_driver;
    use trait_fixer_compiler_host_trait::CompilerHost;
    use crate::actual_compiler_host::ActualCompilerHost; // Import ActualCompilerHost
}

macro_wrapper_lib::wrap_impl! {
    impl<C: rustc_driver::Callbacks> CompilerHost<C> for ActualCompilerHost {
        macro_wrapper_lib::wrap_fn! {
            fn run_compiler_callbacks(&self, args: Vec<String>, _callbacks: &mut C) {
                gemini_compiler_host::run_compiler_with_gemini_emitter(args);
            }
        }
    }
}

macro_wrapper_lib::MODULE_FOOTER!();
