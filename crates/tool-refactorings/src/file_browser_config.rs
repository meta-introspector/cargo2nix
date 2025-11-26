// crates/tool-refactorings/src/file_browser_config.rs

/// Trait for providing CodeExecutorMarkdownFileBrowser configuration values.
pub trait CodeExecutorFileBrowserConfigProvider {
    fn get_viewport_size(&self) -> usize;
    fn get_save_converted_files(&self) -> bool;
    fn get_converted_files_dir_name(&self) -> &str;
    fn get_converted_file_extension(&self) -> &str;
}

/// Dummy implementation of CodeExecutorFileBrowserConfigProvider returning hardcoded values.
pub struct DefaultFileBrowserConfig;

impl CodeExecutorFileBrowserConfigProvider for DefaultFileBrowserConfig {
    fn get_viewport_size(&self) -> usize {
        1024 * 8
    }

    fn get_save_converted_files(&self) -> bool {
        false
    }

    fn get_converted_files_dir_name(&self) -> &str {
        "converted_files"
    }

    fn get_converted_file_extension(&self) -> &str {
        ".converted.md"
    }
}
