// crates/tool-refactorings/src/movie_toolkit_config.rs

/// Trait for providing Movie Toolkit configuration values.
pub trait MovieToolkitConfigProvider {
    fn get_include_adult(&self) -> bool;
    fn get_language(&self) -> &str;
    fn get_page(&self) -> u32;
    fn get_must_contain_job(&self) -> &[&str];
    fn get_crew_limit(&self) -> u32;
    fn get_target_languages(&self) -> &[&str];
}

/// Dummy implementation of MovieToolkitConfigProvider returning hardcoded values.
pub struct DefaultMovieToolkitConfig;

impl MovieToolkitConfigProvider for DefaultMovieToolkitConfig {
    fn get_include_adult(&self) -> bool {
        false
    }

    fn get_language(&self) -> &str {
        "en-US"
    }

    fn get_page(&self) -> u32 {
        1
    }

    fn get_must_contain_job(&self) -> &[&str] {
        &["Director", "Producer", "Writer"]
    }

    fn get_crew_limit(&self) -> u32 {
        10
    }

    fn get_target_languages(&self) -> &[&str] {
        &["NL", "CN", "US", "DE", "RU", "JP"]
    }
}
