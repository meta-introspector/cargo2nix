// crates/tool-refactorings/src/movie_toolkits.rs

use crate::movie_toolkit_config::MovieToolkitConfigProvider;
use gemini_utils::gemini_eprintln;

pub struct MovieToolkits<T: MovieToolkitConfigProvider> {
    config_provider: T,
}

impl<T: MovieToolkitConfigProvider> MovieToolkits<T> {
    pub fn new(config_provider: T) -> Self {
        gemini_eprintln!("Initializing MovieToolkits with config:");
        gemini_eprintln!("  Include adult: :adult:", adult = config_provider.get_include_adult());
        gemini_eprintln!("  Language: :lang:", lang = config_provider.get_language());
        gemini_eprintln!("  Page: :page:", page = config_provider.get_page());
        MovieToolkits { config_provider }
    }

    // Simulate movie search and detail fetching
    pub fn get_movie_details(&self, movie_name: &str) -> Option<Movie> {
        gemini_eprintln!("Searching for movie: :name:", name = movie_name);
        gemini_eprintln!("  With language: :lang:", lang = self.config_provider.get_language());
        gemini_eprintln!("  Include adult: :adult:", adult = self.config_provider.get_include_adult());

        // Simulate API call result
        if movie_name.to_lowercase() == "the matrix" {
            Some(Movie {
                id: "1".to_string(),
                title: "The Matrix".to_string(),
                release_date: "1999-03-31".to_string(),
                crew: vec![
                    CrewMember { name: "Lana Wachowski".to_string(), job: "Director".to_string() },
                    CrewMember { name: "Lilly Wachowski".to_string(), job: "Director".to_string() },
                    CrewMember { name: "Joel Silver".to_string(), job: "Producer".to_string() },
                    CrewMember { name: "Keanu Reeves".to_string(), job: "Actor".to_string() },
                    CrewMember { name: "Carrie-Anne Moss".to_string(), job: "Actor".to_string() },
                ],
                // Other fields would go here
            })
        } else {
            None
        }
    }

    pub fn get_crew_by_job<'a>(&self, movie: &'a Movie) -> Vec<&'a CrewMember> {
        let mut filtered_crew = Vec::new();
        let must_contain_job = self.config_provider.get_must_contain_job();
        let crew_limit = self.config_provider.get_crew_limit();

        gemini_eprintln!("Filtering crew for movie :title: with jobs :jobs:", title = &movie.title, jobs = format!("{:?}", must_contain_job));

        for member in &movie.crew {
            if must_contain_job.contains(&member.job.as_str()) {
                filtered_crew.push(member);
            }
        }
        filtered_crew.truncate(crew_limit as usize);
        filtered_crew
    }
}

pub struct Movie {
    pub id: String,
    pub title: String,
    pub release_date: String,
    pub crew: Vec<CrewMember>,
    // Add other fields as needed
}

#[derive(Debug, PartialEq)]
pub struct CrewMember {
    pub name: String,
    pub job: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movie_toolkit_config::DefaultMovieToolkitConfig;

    #[test]
    fn test_movie_toolkits_new() {
        let config = DefaultMovieToolkitConfig;
        let toolkits = MovieToolkits::new(config);
        assert_eq!(toolkits.config_provider.get_language(), "en-US");
        assert_eq!(toolkits.config_provider.get_crew_limit(), 10);
    }

    #[test]
    fn test_get_movie_details() {
        let config = DefaultMovieToolkitConfig;
        let toolkits = MovieToolkits::new(config);
        let movie = toolkits.get_movie_details("The Matrix");
        assert!(movie.is_some());
        assert_eq!(movie.unwrap().title, "The Matrix");

        let no_movie = toolkits.get_movie_details("NonExistent Movie");
        assert!(no_movie.is_none());
    }

    #[test]
    fn test_get_crew_by_job() {
        let config = DefaultMovieToolkitConfig;
        let toolkits = MovieToolkits::new(config);
        let movie = toolkits.get_movie_details("The Matrix").unwrap();

        let directors = toolkits.get_crew_by_job(&movie);
        assert_eq!(directors.len(), 3); // Lana, Lilly, Joel (Producer)
        assert!(directors.iter().any(|c| c.name == "Lana Wachowski"));
        assert!(directors.iter().any(|c| c.name == "Joel Silver"));

        // Test with different jobs
        struct CustomConfig;
        impl MovieToolkitConfigProvider for CustomConfig {
            fn get_include_adult(&self) -> bool { false }
            fn get_language(&self) -> &str { "en-US" }
            fn get_page(&self) -> u32 { 1 }
            fn get_must_contain_job(&self) -> &[&str] { &["Actor"] } // Only actors
            fn get_crew_limit(&self) -> u32 { 10 }
            fn get_target_languages(&self) -> &[&str] { &[] }
        }
        let custom_toolkits = MovieToolkits::new(CustomConfig);
        let actors = custom_toolkits.get_crew_by_job(&movie);
        assert_eq!(actors.len(), 2);
        assert!(actors.iter().any(|c| c.name == "Keanu Reeves"));
    }

    #[test]
    fn test_get_crew_by_job_limit() {
        struct LimitedConfig;
        impl MovieToolkitConfigProvider for LimitedConfig {
            fn get_include_adult(&self) -> bool { false }
            fn get_language(&self) -> &str { "en-US" }
            fn get_page(&self) -> u32 { 1 }
            fn get_must_contain_job(&self) -> &[&str] { &["Director", "Producer", "Actor"] }
            fn get_crew_limit(&self) -> u32 { 2 } // Limit to 2
            fn get_target_languages(&self) -> &[&str] { &[] }
        }
        let toolkits = MovieToolkits::new(LimitedConfig);
        let movie = toolkits.get_movie_details("The Matrix").unwrap();
        let crew = toolkits.get_crew_by_job(&movie);
        assert_eq!(crew.len(), 2);
    }
}
