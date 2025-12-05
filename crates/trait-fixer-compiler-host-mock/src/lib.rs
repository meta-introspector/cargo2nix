// crates/trait-fixer-compiler-host-mock/src/lib.rs

use trait_fixer_compiler_host_trait::CompilerHost; // Import the trait

// A concrete mock implementation
pub struct MockCompilerHost;

impl<C> CompilerHost<C> for MockCompilerHost {
    // Generic over C, no Callbacks constraint here
    fn run_compiler_callbacks(&self, _args: Vec<String>, _callbacks: &mut C) {
        println!("MockCompilerHost::run_compiler_callbacks called");
    }
}
