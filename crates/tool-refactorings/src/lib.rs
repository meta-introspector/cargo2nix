pub mod sparql_executer;
pub mod movie_toolkit_config;
pub mod movie_toolkits;
pub mod file_browser_config;
pub mod code_executor_file_browser;
pub mod base_code_executor_config;
pub mod base_code_executor;
pub mod repository_indexer_config;
pub mod repository_indexer;
pub mod non_vendored_workspace_enforcer;
pub mod gh_plan_executor;
pub mod package_extractor;

/// Trait for providing SPARQL configuration values.
pub trait SparqlConfigProvider {
    fn get_sparql_endpoint_url(&self) -> &str;
    fn get_freebase_uri_prefix(&self) -> &str;
    fn get_time_offset_string(&self) -> &str;
}

/// Dummy implementation of SparqlConfigProvider returning hardcoded values.
pub struct DefaultSparqlConfig;

impl SparqlConfigProvider for DefaultSparqlConfig {
    fn get_sparql_endpoint_url(&self) -> &str {
        "http://164.107.116.56:3093/sparql"
    }

    fn get_freebase_uri_prefix(&self) -> &str {
        "http://rdf.freebase.com/ns/"
    }

    fn get_time_offset_string(&self) -> &str {
        "-08:00"
    }
}