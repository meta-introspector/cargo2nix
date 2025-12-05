# Tool: `web_fetch`

## Category: External Information & Web Interaction

## Purpose
Processes content from URL(s), including local and private network addresses (e.g., localhost), embedded in a prompt. This tool allows for gathering information directly from web resources or internal services.

## Use Cases
*   Summarizing articles or documentation from external websites.
*   Extracting specific data from web pages or API responses.
*   Fetching content from local development servers for testing or analysis.
*   Analyzing public data available via URLs.

## Key Parameters

*   **`prompt`** (string, **required**):
    *   A comprehensive prompt that includes the URL(s) (up to 20) to fetch and specific instructions on how to process their content. All URLs to be fetched must be valid and complete, starting with "http://" or "https://", and be fully-formed with a valid hostname (e.g., a domain name like "example.com" or an IP address).

## Examples

**1. Summarize an article from a website:**
```
web_fetch(prompt="Summarize the key points from https://example.com/blog/article-on-ai-ethics")
```

**2. Extract specific data from an API endpoint:**
```
web_fetch(prompt="Extract the 'name' and 'version' fields from the JSON response of http://localhost:8080/api/status")
```

**3. Fetch and analyze content from multiple URLs:**
```
web_fetch(prompt="Compare the software requirements listed on https://project1.com/requirements and https://project2.org/specs. Highlight any discrepancies.")
```