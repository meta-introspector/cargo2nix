```python
class CodeExecutorContext:
    """
    Manages the context for a code executor, including the current working directory
    and a history of executed commands and their outputs.
    """

    def __init__(self, working_dir: Path):
        self.working_dir = working_dir
        self.history: List[Tuple[str, str]] = []  # (command, output)

    def add_to_history(self, command: str, output: str):
        self.history.append((command, output))

    def get_history(self) -> List[Tuple[str, str]]:
        return self.history

    def get_working_dir(self) -> Path:
        return self.working_dir
```

```rust
// Rust translation for the CodeExecutorContext class

use std::path::{Path, PathBuf};
use std::vec::Vec;

/// Manages the context for a code executor, including the current working directory
/// and a history of executed commands and their outputs.
#[derive(Debug, Clone)]
pub struct CodeExecutorContext {
    working_dir: PathBuf,
    history: Vec<(String, String)>, // (command, output)
}

impl CodeExecutorContext {
    /// Creates a new `CodeExecutorContext` with the specified working directory.
    pub fn new(working_dir: PathBuf) -> Self {
        CodeExecutorContext {
            working_dir,
            history: Vec::new(),
        }
    }

    /// Adds a command and its output to the history.
    pub fn add_to_history(&mut self, command: String, output: String) {
        self.history.push((command, output));
    }

    /// Returns a reference to the history of executed commands and their outputs.
    pub fn get_history(&self) -> &Vec<(String, String)> {
        &self.history
    }

    /// Returns a reference to the current working directory.
    pub fn get_working_dir(&self) -> &Path {
        &self.working_dir
    }

    /// Returns a mutable reference to the current working directory.
    pub fn get_working_dir_mut(&mut self) -> &mut PathBuf {
        &mut self.working_dir
    }
}

// Example usage
/*
fn main() {
    let mut context = CodeExecutorContext::new(PathBuf::from("/tmp/my_project"));
    context.add_to_history("ls -l".to_string(), "total 0\n-rw-r--r-- 1 user user 0 Jan 1 00:00 file.txt".to_string());
    context.add_to_history("pwd".to_string(), "/tmp/my_project".to_string());

    println!("Working directory: {}", context.get_working_dir().display());
    for (cmd, output) in context.get_history() {
        println!("Command: {}\nOutput: {}", cmd, output);
    }
}
*/
```