type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use git_wrapper_lib::real_git_wrapper_lib::RealGitWrapperLib;
use git_wrapper_lib::mock_git_wrapper_lib::MockGitWrapperLib;
use git_wrapper_lib::GitWrapperLibTrait;

fn main() -> Result<()> {
    println!("Running git-wrapper-lib-test-no-features");

    // Test RealGitWrapperLib
    let real_wrapper = RealGitWrapperLib::new();
    println!("RealGitWrapperLib instantiated.");
    // Call a simple method to ensure it compiles
    let _ = real_wrapper.git_executor().status(&std::path::PathBuf::from("."));
    println!("RealGitWrapperLib basic method called.");

    // Test MockGitWrapperLib
    let mock_wrapper = MockGitWrapperLib::new();
    println!("MockGitWrapperLib instantiated.");
    // Call a simple method to ensure it compiles
    let _ = mock_wrapper.git_executor().status(&std::path::PathBuf::from("."));
    println!("MockGitWrapperLib basic method called.");

    Ok(())
}