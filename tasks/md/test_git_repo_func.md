```python
def test_git_repo(tmp_path: Path, repo_name: str = "test_repo") -> Path:
    """
    Creates a dummy Git repository for testing purposes.
    """
    repo_path = tmp_path / repo_name
    repo_path.mkdir()
    subprocess.run(["git", "init"], cwd=repo_path, check=True)
    (repo_path / "README.md").write_text("# Test Repo")
    subprocess.run(["git", "add", "."], cwd=repo_path, check=True)
    subprocess.run(["git", "commit", "-m", "Initial commit"], cwd=repo_path, check=True)
    return repo_path
```

```rust
// Rust translation for the test_git_repo function

use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use anyhow::{Result, anyhow};

/// Creates a dummy Git repository for testing purposes.
fn create_dummy_git_repo(tmp_path: &Path, repo_name: &str) -> Result<PathBuf> {
    let repo_path = tmp_path.join(repo_name);
    fs::create_dir_all(&repo_path)?;

    // git init
    Command::new("git")
        .arg("init")
        .current_dir(&repo_path)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow!("git init failed"))?;

    // Create README.md
    fs::write(repo_path.join("README.md"), "# Test Repo")?;

    // git add .
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&repo_path)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow!("git add failed"))?;

    // git commit -m "Initial commit"
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .current_dir(&repo_path)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow!("git commit failed"))?;

    Ok(repo_path)
}

// Example usage (would be in a test function)
/*
#[test]
fn test_create_dummy_repo() -> Result<()> {
    let tmp_dir = tempfile::tempdir()?; // Requires `tempfile` crate
    let repo_path = create_dummy_git_repo(tmp_dir.path(), "my_test_repo")?;
    println!("Created dummy repo at: {}", repo_path.display());

    // Verify it's a git repo
    assert!(repo_path.join(".git").exists());
    assert!(repo_path.join("README.md").exists());

    Ok(())
}
*/
```