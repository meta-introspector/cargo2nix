// crates/tool-refactorings/src/base_code_executor_config.rs

/// Trait for providing BaseCodeExecutor configuration values.
pub trait BaseCodeExecutorConfigProvider {
    fn get_error_retry_attempts(&self) -> u32;
    fn get_code_block_delimiters(&self) -> &[(&str, &str)]; // Changed to slices of &str
    fn get_execution_result_delimiters(&self) -> (&str, &str); // Changed to tuple of &str
}

/// Dummy implementation of BaseCodeExecutorConfigProvider returning hardcoded values.
pub struct DefaultBaseCodeExecutorConfig;

impl BaseCodeExecutorConfigProvider for DefaultBaseCodeExecutorConfig {
    fn get_error_retry_attempts(&self) -> u32 {
        2
    }

    fn get_code_block_delimiters(&self) -> &[(&str, &str)] {
        &[
            ("\n", "\n```"),
            ("```python\n", "\n```"),
        ]
    }

    fn get_execution_result_delimiters(&self) -> (&str, &str) {
        ("```tool_output\n", "\n```")
    }
}