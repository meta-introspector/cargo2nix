macro_wrapper_lib::MODULE_HEADER!();

macro_wrapper_lib::wrap_use! {
    use rustc_driver;
    use trait_fixer_compiler_host_trait::CompilerHost;
}

macro_wrapper_lib::wrap_struct! {
    pub struct ActualCompilerHost;
}

macro_wrapper_lib::MODULE_FOOTER!();
