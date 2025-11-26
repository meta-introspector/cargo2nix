// crates/tool-refactorings/src/code_executor_file_browser.rs

use crate::file_browser_config::CodeExecutorFileBrowserConfigProvider;
use gemini_utils::gemini_eprintln;
use std::fs; // Add fs for file reading
use std::path::{Path, PathBuf};

// Mocking some external dependency
// In a real scenario, this would be a trait for CodeExecutor and MarkdownConverter
// For now, simple mock functions
fn mock_execute_code_blocks(path: &Path) -> String {
    gemini_eprintln!("Mocking code execution for file: :path:", path = path.display());
    if path.to_string_lossy().contains("mock_image.png") {
        "TITLE: Image Conversion Result\nCONTENT: ![](/path/to/mock_image.png)".to_string()
    } else {
        // Read file content for simulation
        match fs::read_to_string(path) {
            Ok(content) => format!("TITLE: Content of {}\nCONTENT:\n{}", path.display(), content),
            Err(_) => format!("TITLE: Error reading file {}\nCONTENT: Could not read file.", path.display()),
        }
    }
}

pub struct CodeExecutorMarkdownFileBrowser<T: CodeExecutorFileBrowserConfigProvider> {
    config_provider: T,
    current_path: PathBuf,
    page_title: String,
    page_content: String,
    viewport_current_page: usize,
    viewport_pages: Vec<(usize, usize)>, // (start_char_idx, end_char_idx)
}

impl<T: CodeExecutorFileBrowserConfigProvider> CodeExecutorMarkdownFileBrowser<T> {
    pub fn new(config_provider: T) -> Self {
        gemini_eprintln!("Initializing CodeExecutorMarkdownFileBrowser with config:");
        gemini_eprintln!("  Viewport size: :size:", size = config_provider.get_viewport_size());
        gemini_eprintln!("  Save converted files: :save:", save = config_provider.get_save_converted_files());
        CodeExecutorMarkdownFileBrowser {
            config_provider,
            current_path: PathBuf::from("."),
            page_title: String::new(),
            page_content: String::new(),
            viewport_current_page: 0,
            viewport_pages: Vec::new(),
        }
    }

    pub async fn set_path(&mut self, path: &str) {
        let new_path = PathBuf::from(path);
        gemini_eprintln!("Setting path to: :path:", path = new_path.display());

        if new_path.exists() {
            self.current_path = new_path.clone();
            if new_path.is_dir() {
                self.page_title = format!("Directory: {}", new_path.display());
                self.page_content = self.list_directory_content(&new_path);
            } else {
                // Simulate file conversion or display
                let conversion_result = mock_execute_code_blocks(&new_path);
                self.parse_conversion_result(&conversion_result);
                // Save converted file if configured
                if self.config_provider.get_save_converted_files() {
                    self.save_converted_file(&new_path, &self.page_content);
                }
            }
            self.update_viewport_pages();
        } else {
            self.page_title = "FileNotFoundError".to_string();
            self.page_content = format!("# FileNotFoundError\n\nFile not found: {}", new_path.display());
            self.viewport_pages = vec![(0, self.page_content.len())];
        }
    }

    fn list_directory_content(&self, path: &Path) -> String {
        let mut content = format!("# Contents of {}\n\n", path.display());
        // Simulate directory listing
        content.push_str("## Directories\n");
        content.push_str("- dir_a/\n- dir_b/\n\n");
        content.push_str("## Files\n");
        content.push_str("- file1.txt\n- file2.md\n- mock_image.png\n");
        content
    }

    fn parse_conversion_result(&mut self, result_string: &str) {
        let mut title_line = "";
        let mut content_start_idx = 0;

        for (i, line) in result_string.lines().enumerate() {
            if line.starts_with("TITLE:") {
                title_line = line;
            } else if line.starts_with("CONTENT:") {
                content_start_idx = i;
                break;
            }
        }

        self.page_title = title_line.strip_prefix("TITLE:").unwrap_or("").trim().to_string();
        self.page_content = result_string.lines()
                                        .skip(content_start_idx + 1)
                                        .collect::<Vec<&str>>()
                                        .join("\n");
    }

    fn save_converted_file(&self, original_path: &Path, content: &str) {
        let work_dir = PathBuf::from("."); // Mock work_dir
        let converted_dir = work_dir.join(self.config_provider.get_converted_files_dir_name());
        let md_filename = original_path.file_stem().unwrap_or_default().to_string_lossy().to_string() + self.config_provider.get_converted_file_extension();
        let full_path = converted_dir.join(md_filename);

        gemini_eprintln!("Simulating saving converted file to: :path:", path = full_path.display());
        // In a real implementation, this would write to file.
    }

    fn update_viewport_pages(&mut self) {
        self.viewport_pages.clear();
        let viewport_size = self.config_provider.get_viewport_size();
        let mut start = 0;
        let content_len = self.page_content.len();

        while start < content_len {
            let end = (start + viewport_size).min(content_len);
            self.viewport_pages.push((start, end));
            start = end;
        }
        if self.viewport_pages.is_empty() {
            self.viewport_pages.push((0, 0));
        }
        self.viewport_current_page = 0;
    }

    pub fn get_current_page_content(&self) -> &str {
        let (start, end) = self.viewport_pages[self.viewport_current_page];
        &self.page_content[start..end]
    }

    pub fn next_page(&mut self) {
        if self.viewport_current_page + 1 < self.viewport_pages.len() {
            self.viewport_current_page += 1;
        }
    }

    pub fn previous_page(&mut self) {
        if self.viewport_current_page > 0 {
            self.viewport_current_page -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_browser_config::DefaultFileBrowserConfig;

    #[tokio::test] // Assuming tokio for async functions
    async fn test_file_browser_new() {
        let config = DefaultFileBrowserConfig;
        let browser = CodeExecutorMarkdownFileBrowser::new(config);
        assert_eq!(browser.config_provider.get_viewport_size(), 1024 * 8);
        assert!(!browser.config_provider.get_save_converted_files());
    }

    #[tokio::test]
    async fn test_set_path_dir() {
        let config = DefaultFileBrowserConfig;
        let mut browser = CodeExecutorMarkdownFileBrowser::new(config);
        
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let path_str = temp_dir.path().to_str().unwrap();

        browser.set_path(path_str).await;
        assert!(browser.page_title.contains("Directory:"));
        assert!(browser.page_content.contains("## Files"));
        assert_eq!(browser.current_path.to_str().unwrap(), path_str);
    }

    #[tokio::test]
    async fn test_set_path_file() {
        let config = DefaultFileBrowserConfig;
        let mut browser = CodeExecutorMarkdownFileBrowser::new(config);

        // Create a temporary file for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.rs");
        std::fs::write(&file_path, "fn main() { println!(\"Hello\"); }").unwrap();
        let path_str = file_path.to_str().unwrap();

        browser.set_path(path_str).await;
        assert!(browser.page_title.contains("Content of"));
        assert!(browser.page_content.contains("Hello"));
    }

    #[tokio::test]
    async fn test_set_path_file_not_found() {
        let config = DefaultFileBrowserConfig;
        let mut browser = CodeExecutorMarkdownFileBrowser::new(config);
        browser.set_path("non_existent_file.txt").await;
        assert_eq!(browser.page_title, "FileNotFoundError");
        assert!(browser.page_content.contains("File not found: non_existent_file.txt"));
    }

    #[tokio::test]
    async fn test_pagination() {
        let mut config = DefaultFileBrowserConfig;
        // Override viewport size for testing pagination
        // This is not ideal as DefaultFileBrowserConfig returns fixed values.
        // A better approach would be a configurable config for tests or a test-specific config provider.
        // For now, we'll manually ensure content is larger than default viewport.

        let mut browser = CodeExecutorMarkdownFileBrowser::new(config);
        
        let mut long_content = String::new();
        for i in 0..2000 { // Make content larger than default 8KB viewport
            long_content.push_str(&format!("Line {}. This is a long line of content to test pagination.\n", i));
        }
        
        // Directly set internal state for testing pagination (not ideal but works for this mock)
        browser.page_content = long_content;
        browser.page_title = "Long Content".to_string();
        browser.update_viewport_pages();

        assert!(browser.viewport_pages.len() > 1);
        assert_eq!(browser.viewport_current_page, 0);

        let initial_page = browser.get_current_page_content().to_string(); // Make a copy
        browser.next_page();
        assert_eq!(browser.viewport_current_page, 1);
        let next_page = browser.get_current_page_content(); // This is fine, immutable borrow
        assert_ne!(&initial_page, next_page); // Compare with reference to copied string

        browser.previous_page();
        assert_eq!(browser.viewport_current_page, 0);
        assert_eq!(browser.get_current_page_content(), initial_page); // Compare with copied string
    }
}
