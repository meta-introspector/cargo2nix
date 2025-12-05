# Tool: `replace`

## Category: File System Management

## Purpose
Replaces specific text within a file. By default, it replaces a single occurrence, but can replace multiple occurrences when `expected_replacements` is specified. This tool requires providing significant context around the change to ensure precise targeting.

## Use Cases
*   Making precise, targeted modifications to code, configuration, or documentation.
*   Fixing specific, localized bugs (e.g., correcting a typo in a single variable name).
*   Small-scale refactoring where an exact string needs to be changed across a known number of occurrences.

## Key Parameters

*   **`file_path`** (string, **required**):
    *   The path to the file to modify.
*   **`instruction`** (string, **required**):
    *   A clear, semantic instruction for the code change, explaining the WHY, WHERE, WHAT, and desired outcome of the change.
*   **`old_string`** (string, **required**):
    *   The exact literal text to replace. For single replacements (default), this *must* include at least 3 lines of context BEFORE and AFTER the target text, matching whitespace and indentation precisely. If this string is not an exact literal match or matches multiple locations, the tool will fail.
*   **`new_string`** (string, **required**):
    *   The exact literal text to replace `old_string` with.
*   **`expected_replacements`** (number, optional, defaults to `1`):
    *   The number of replacements expected. Use when you want to replace multiple occurrences. The tool will replace ALL occurrences that match `old_string` exactly.

## Important Considerations

*   **Precision is Critical**: `old_string` must be an *exact literal* match, including all whitespace, indentation, and newlines. Any discrepancy will lead to failure.
*   **Context for Uniqueness**: For single replacements, the `old_string` must be unique within the file when combined with its surrounding context.
*   **User Preference**: Due to its sensitivity and potential for unintended changes, the user prefers refactoring and rewriting over direct edits using this tool. It should only be used if it is confirmed to work and is absolutely necessary for targeted, atomic changes. Always use the `read_file` tool to examine the file's current content before attempting a text replacement.
*   **Breaking Down Changes**: Prefer to break down complex and long changes into multiple smaller, atomic calls to this tool. Always check the content of the file after changes or if a string was not found to match.

## Examples

**1. Correct a typo in a function name:**
```
replace(
    file_path="src/utils.rs",
    instruction="Correct the typo from 'intialise' to 'initialise' in the `initialise_config` function.",
    old_string="""
    fn intialise_config() -> Result<Config, Error> {
        // ...
    }
    """,
    new_string="""
    fn initialise_config() -> Result<Config, Error> {
        // ...
    }
    """
)
```

**2. Update a version number in a configuration file (assuming a unique context):**
```
replace(
    file_path="Cargo.toml",
    instruction="Update the crate version from 0.1.0 to 0.2.0.",
    old_string="""
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
""",
    new_string="""
[package]
name = "my-crate"
version = "0.2.0"
authors = ["Your Name <you@example.com>"]
"""
)
```