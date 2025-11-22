```python
class SparqlExecuter:
    """
    Executes SPARQL queries against a given endpoint.
    """

    def __init__(self, endpoint_url: str):
        self.endpoint_url = endpoint_url

    def execute_query(self, query: str) -> Dict:
        """
        Executes a SPARQL query and returns the JSON result.
        """
        headers = {"Accept": "application/sparql-results+json"}
        response = requests.post(self.endpoint_url, data={"query": query}, headers=headers)
        response.raise_for_status()
        return response.json()
```

```rust
// Conceptual Rust translation for the SparqlExecuter class

use reqwest::{Client, Error as ReqwestError};
use serde_json::{Value, Error as SerdeJsonError};
use std::collections::HashMap;
use anyhow::Result; // Using anyhow for simplified error handling

/// Errors that can occur during SPARQL execution.
#[derive(Debug, thiserror::Error)]
enum SparqlError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] ReqwestError),
    #[error("JSON deserialization failed: {0}")]
    JsonDeserialization(#[from] SerdeJsonError),
    #[error("SPARQL query failed: {0}")]
    QueryFailed(String),
}

/// Executes SPARQL queries against a given endpoint.
pub struct SparqlExecutor {
    endpoint_url: String,
    http_client: Client,
}

impl SparqlExecutor {
    /// Creates a new `SparqlExecutor`.
    pub fn new(endpoint_url: String) -> Self {
        SparqlExecutor {
            endpoint_url,
            http_client: Client::new(),
        }
    }

    /// Executes a SPARQL query and returns the JSON result.
    pub async fn execute_query(&self, query: &str) -> Result<Value, SparqlError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            "application/sparql-results+json".parse().unwrap(),
        );

        let mut form_data = HashMap::new();
        form_data.insert("query", query);

        let response = self.http_client
            .post(&self.endpoint_url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?
            .error_for_status()?; // Automatically handles 4xx/5xx HTTP errors

        let json_result: Value = response.json().await?;
        Ok(json_result)
    }
}

// Example usage (requires an async runtime like tokio)
/*
#[tokio::main]
async fn main() -> Result<()> {
    let executor = SparqlExecutor::new("http://localhost:8080/sparql".to_string());
    let query = r#"
        SELECT ?s ?p ?o WHERE {
            ?s ?p ?o .
        } LIMIT 10
    "#;

    match executor.execute_query(query).await {
        Ok(results) => {
            println!("SPARQL Results: {}", serde_json::to_string_pretty(&results)?);
        }
        Err(e) => {
            eprintln!("Error executing SPARQL query: {}", e);
        }
    }
    Ok(())
}
*/
```