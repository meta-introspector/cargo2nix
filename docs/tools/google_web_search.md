# Tool: `google_web_search`

## Category: External Information & Web Interaction

## Purpose
Performs a web search using Google Search (via the Gemini API) and returns the results. This tool is useful for finding information on the internet based on a query when project-specific or local documentation is insufficient.

## Use Cases
*   Researching external libraries, frameworks, or APIs.
*   Finding solutions to general programming problems or error messages.
*   Gathering information about best practices or common design patterns.
*   Staying updated with recent technological advancements or news.

## Key Parameters

*   **`query`** (string, **required**):
    *   The search query to find information on the web.

## Examples

**1. Search for documentation on a Rust crate:**
```
google_web_search(query="rust serde crate documentation")
```

**2. Find solutions for a specific error message:**
```
google_web_search(query="cargo build error E0277 trait not implemented")
```

**3. Research best practices for secure coding:**
```
google_web_search(query="secure coding practices rust web applications")
```