# Tool: `codebase_investigator`

## Category: Codebase Search & Analysis

## Purpose
The `codebase_investigator` is a specialized tool for comprehensive codebase analysis, architectural mapping, and understanding system-wide dependencies. It is designed to provide a structured report with key file paths, symbols, and actionable architectural insights, especially for complex or unfamiliar codebases.

## Use Cases
*   **Vague Requests**: When a user's request is broad or lacks specific details, this tool can be used to gather initial context and understanding.
*   **Bug Root-Cause Analysis**: To delve deep into the codebase to identify the underlying causes of defects.
*   **System Refactoring**: Before undertaking significant changes, use this tool to map out the current architecture, identify dependencies, and understand potential impacts.
*   **Comprehensive Feature Implementation**: To gain a holistic view of the system when implementing new, complex features that might touch multiple modules.
*   **Answering Questions About the Codebase**: For queries that require a deep understanding of how different parts of the project interact.
*   **Onboarding**: To quickly get an overview of a new project or a new part of an existing project.

## Key Parameters

*   **`objective`** (string, **required**):
    *   A comprehensive and detailed description of the user's ultimate goal. This must include the original user's objective, as well as any questions and extra context the agent may have, to guide the investigation effectively.

## Examples

**1. Understand the architecture of the authentication module:**
```
codebase_investigator(
    objective="Understand the architecture of the authentication module, including its main components, data flow, and dependencies on other parts of the system. Specifically, I want to know how user login, session management, and authorization checks are performed."
)
```

**2. Analyze a performance bottleneck in the data processing pipeline:**
```
codebase_investigator(
    objective="Investigate the data processing pipeline to identify the root cause of a recent performance bottleneck. I need to understand the sequence of operations, where data is transformed, and any I/O operations that might be slowing down the process. Focus on the `process_large_dataset` function and its callees."
)
```

**3. Prepare for refactoring the database interaction layer:**
```
codebase_investigator(
    objective="Prepare for a major refactoring of the database interaction layer. Provide an overview of all database access patterns, ORM usage (if any), and how data models are defined and used throughout the application. Highlight areas of tight coupling or potential for improvement."
)
```