// crates/trait-fixer-compiler-host-trait/src/lib.rs

use rustc_driver;

pub trait CompilerHost {
    fn run_compiler_callbacks<C: rustc_driver::Callbacks>(
        &self,
        args: Vec<String>,
        callbacks: &mut C,
    );
}
