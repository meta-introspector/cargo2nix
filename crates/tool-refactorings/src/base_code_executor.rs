// crates/tool-refactorings/src/base_code_executor.rs

use crate::base_code_executor_config::BaseCodeExecutorConfigProvider;
use gemini_utils::gemini_eprintln;

pub struct BaseCodeExecutor<T: BaseCodeExecutorConfigProvider> {
    config_provider: T,
}

impl<T: BaseCodeExecutorConfigProvider> BaseCodeExecutor<T> {
    pub fn new(config_provider: T) -> Self {
        gemini_eprintln!("Initializing BaseCodeExecutor with config:");
        gemini_eprintln!("  Error retry attempts: :retries:", retries = config_provider.get_error_retry_attempts());
        gemini_eprintln!("  Code block delimiters (count: :count:)", count = config_provider.get_code_block_delimiters().len());
        gemini_eprintln!("  Execution result delimiters (found: :found:)", found = !config_provider.get_execution_result_delimiters().0.is_empty());
        BaseCodeExecutor { config_provider }
    }

    // This is a highly simplified simulation of code execution.
    // The actual Python class had much more complex logic for running code.
    pub fn execute_code_blocks(&self, code: &str) -> String {
        gemini_eprintln!("Simulating execution of code block.");
        gemini_eprintln!("Code: :code:", code = code);
        
        // Simulate retries
        for attempt in 0..self.config_provider.get_error_retry_attempts() {
            gemini_eprintln!("  Attempt :attempt:/:max_attempts:", attempt = attempt + 1, max_attempts = self.config_provider.get_error_retry_attempts());
            // Simulate success on first attempt, or always success for simplicity
            if code.contains("error!") && attempt == 0 {
                gemini_eprintln!("Simulating an error on first attempt for code containing 'error!'.");
                // Simulate error output
                return format!(
                    "{delim_start}{lang}\nError: Simulated execution failure. Attempt {attempt_num}\n{delim_end}",
                    delim_start = self.config_provider.get_execution_result_delimiters().0,
                    lang = "python", // Simulate language
                    attempt_num = attempt + 1,
                    delim_end = self.config_provider.get_execution_result_delimiters().1,
                );
            } else if code.contains("print(") {
                gemini_eprintln!("Simulating print output.");
                return format!(
                    "{delim_start}{lang}\nSimulated output for print statement.\n{delim_end}",
                    delim_start = self.config_provider.get_execution_result_delimiters().0,
                    lang = "python", // Simulate language
                    delim_end = self.config_provider.get_execution_result_delimiters().1,
                );
            }
        }

        // Default successful execution
        format!(
            "{delim_start}{lang}\nSimulated successful execution.\n{delim_end}",
            delim_start = self.config_provider.get_execution_result_delimiters().0,
            lang = "python", // Simulate language
            delim_end = self.config_provider.get_execution_result_delimiters().1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_code_executor_config::DefaultBaseCodeExecutorConfig;

    #[test]
    fn test_base_code_executor_new() {
        let config = DefaultBaseCodeExecutorConfig;
        let executor = BaseCodeExecutor::new(config);
        assert_eq!(executor.config_provider.get_error_retry_attempts(), 2);
        assert_eq!(executor.config_provider.get_code_block_delimiters().len(), 2);
    }

    #[test]
    fn test_execute_code_block_success() {
        let config = DefaultBaseCodeExecutorConfig;
        let executor = BaseCodeExecutor::new(config);
        let code = "print('Hello, world!')";
        let output = executor.execute_code_blocks(code);
        let (start_delim, end_delim) = DefaultBaseCodeExecutorConfig.get_execution_result_delimiters();
        assert!(output.starts_with(start_delim));
        assert!(output.contains("Simulated output for print statement."));
        assert!(output.ends_with(end_delim));
    }

    #[test]
    fn test_execute_code_block_error() {
        let config = DefaultBaseCodeExecutorConfig;
        let executor = BaseCodeExecutor::new(config);
        let code = "raise error!";
        let output = executor.execute_code_blocks(code);
        let (start_delim, end_delim) = DefaultBaseCodeExecutorConfig.get_execution_result_delimiters();
        assert!(output.starts_with(start_delim));
        assert!(output.contains("Simulated execution failure. Attempt 1"));
        assert!(output.ends_with(end_delim));
    }
}
