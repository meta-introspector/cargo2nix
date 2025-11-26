# Tool: `save_memory`

## Category: Workflow & Task Management

## Purpose
Saves a specific piece of information or fact to the agent's long-term memory. This tool is designed to remember user-specific facts or preferences that persist across sessions, personalizing future interactions.

## Use Cases
*   When the user explicitly asks the agent to remember something (e.g., "Remember my preferred coding style is `snake_case`").
*   When the user states a clear, concise fact about themselves, their preferences, or their environment that is important for future interactions (e.g., "My usual build command is `cargo xtask build`").
*   To store specific project-related preferences or common aliases provided by the user.

## Key Parameters

*   **`fact`** (string, **required**):
    *   The specific fact or piece of information to remember. This should be a clear, self-contained statement. For example, if the user says "My favorite color is blue", the fact would be "My favorite color is blue".

## Important Considerations

*   **Use Sparingly**: This tool is for long-term, user-specific facts, not for remembering conversational context that is only relevant for the current session.
*   **Concise Facts**: The fact should be relatively short and to the point. Avoid saving long, complex, or rambling pieces of text.
*   **Confirmation**: If unsure whether to save something, the agent can ask the user, "Should I remember that for you?"

## Examples

**1. Remember a user's preferred programming language:**
```
save_memory(fact="User's preferred programming language is Rust.")
```

**2. Store a common project-specific alias:**
```
save_memory(fact="The alias 'dtr' is used for 'docker-compose run --rm'."
```

**3. Record a user's preference for output format:**
```
save_memory(fact="User prefers code examples in documentation to use Markdown code blocks with language highlighting.")
```