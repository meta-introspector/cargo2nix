#[cfg(feature = "with-anyhow")]
use anyhow::Result;
#[cfg(not(feature = "with-anyhow"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use git_wrapper_lib::real_git_wrapper_lib::RealGitWrapperLib;
use git_wrapper_lib::mock_git_wrapper_lib::MockGitWrapperLib;
use git_wrapper_lib::GitWrapperLibTrait;

fn main() -> Result<()> {
    println!("Running git-wrapper-lib-test-git2-only");

    // Test RealGitWrapperLib
    let real_wrapper = RealGitWrapperLib::new();
    println!("RealGitWrapperLib instantiated.");
    let _ = real_wrapper.git_executor().status(&std::path::PathBuf::from("."));
    println!("RealGitWrapperLib basic method called.");

    // Test MockGitWrapperLib
    let mock_wrapper = MockGitWrapperLib::new();
    println!("MockGitWrapperLib instantiated.");
    let _ = mock_wrapper.git_executor().status(&std::path::PathBuf::from("."));
    println!("MockGitWrapperLib basic method called.");

    Ok(())
}