// crates/trait-fixer-compiler-host-mock/src/lib.rs

use rustc_driver;
use trait_fixer_compiler_host_trait::CompilerHost; // Import the trait

// A concrete mock implementation
pub struct MockCompilerHost;

impl CompilerHost for MockCompilerHost {
    fn run_compiler_callbacks<C: rustc_driver::Callbacks>(
        &self,
        _args: Vec<String>,
        _callbacks: &mut C,
    ) {
        println!("MockCompilerHost::run_compiler_callbacks called");
    }
}
