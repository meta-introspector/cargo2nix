```python
class SystemPromptStrategy(ABC):
    """
    Abstract base class for strategies that build system prompts.
    """

    @abstractmethod
    def build_prompt(self, context: CodeExecutorContext) -> str:
        pass
```

```rust
// Rust translation for the SystemPromptStrategy abstract base class

use crate::code_executor_context_class::CodeExecutorContext; // Assuming CodeExecutorContext is in the same crate

/// Trait for strategies that build system prompts.
pub trait SystemPromptStrategy {
    /// Builds a system prompt based on the provided context.
    fn build_prompt(&self, context: &CodeExecutorContext) -> String;
}

// Example of a concrete implementation
/*
pub struct SimplePromptStrategy;

impl SystemPromptStrategy for SimplePromptStrategy {
    fn build_prompt(&self, context: &CodeExecutorContext) -> String {
        format!(
            "You are an AI assistant. Current working directory: {}. History: {:?}",
            context.get_working_dir().display(),
            context.get_history()
        )
    }
}

// Example usage
fn main() {
    let mut context = CodeExecutorContext::new(PathBuf::from("/app"));
    context.add_to_history("ls".to_string(), "file1.txt".to_string());

    let strategy = SimplePromptStrategy;
    let prompt = strategy.build_prompt(&context);
    println!("{}", prompt);
}
*/
```