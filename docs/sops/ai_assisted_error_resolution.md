# SOP: AI-Assisted Error Resolution using Structured Build Output

## 1. Purpose

This Standard Operating Procedure (SOP) outlines the methodology for utilizing AI agents, such as Gemini CLI, to automate or assist in the resolution of compiler errors and warnings. By leveraging structured build output, AI agents can intelligently analyze detected issues, generate targeted semantic patches, and verify their effectiveness, thereby streamlining the software development and maintenance process.

## 2. Key Concepts

### 2.1 Structured Build Output (`cargo build --message-format=json`)
As detailed in "3.4 Leveraging Structured Build Output" of the [SOP: Structured Editing and Change Management](structured_editing_and_change_management.md), the `cargo build --message-format=json` command provides a machine-readable stream of build events, including detailed compiler messages (warnings and errors). This output is the primary source of information for AI-assisted analysis.

### 2.2 Ticket Data Files
For each identified error or warning group (a "ticket"), the `ticket_data_generator` tool creates two files:
-   **`ticket_<id>.json`**: A comprehensive JSON file containing structured information about the error group, including its summary, all occurrences, code snippets, heuristically identified referenced files, and relevant project documentation. This file serves as the rich context for the AI.
-   **`task_<id>.md`**: A Markdown file derived from the JSON data, providing a human-readable summary of the error and a list of `ripgrep` commands to aid in manual investigation.

### 2.3 Semantic Patches (`rustc_expand_patches` crate)
Semantic patches, defined in TOML files and processed by the `rustc_expand_patches` `build.rs` script, are the mechanism for applying structured code modifications. AI agents will formulate specific `EditJob` types (e.g., `RemoveUse`, `ReplaceExpression`) to address the errors identified in the ticket data.

## 3. Workflow for AI Agent (e.g., Gemini CLI)

This section describes the step-by-step process an AI agent would follow to resolve an error using the generated ticket data.

### 3.1 Ticket Selection and Initialization

The AI agent is initialized with a specific task: to resolve an error associated with a given `ticket_id`. It has access to the generated `ticket_<id>.json` and `task_<id>.md` files.

**Agent's Initial Actions:**
1.  **Load Ticket Data:** Read the `ticket_<id>.json` file to ingest all available structured information about the error.
    *   **Tool Call (Simulated):** `read_file("ticket_data_output/ticket_<id>.json")`

### 3.2 Contextual Analysis

The AI agent parses the loaded `TicketData` to understand the nature and location of the error. It focuses on:
-   `target_name`: The crate or target where the error occurred.
-   `summary_message`: A high-level description of the error group.
-   `occurrences`: Detailed information about each instance of the error, including:
    -   `rendered_message`: The exact compiler message.
    -   `file_path`: The source file where the error is located.
    -   `line_start`, `line_end`: The lines affected.
    -   `code_snippet`: The relevant code context.
-   `search_commands`: Pre-generated `ripgrep` commands for investigation.

### 3.3 Investigation and Confirmation

To deepen its understanding and confirm the context, the AI agent executes the provided `search_commands` or generates new ones based on its analysis. This helps to pinpoint the exact code causing the issue and its surrounding environment.

**Agent's Actions:**
1.  **Execute Search Commands:** Run one or more `ripgrep` commands.
    *   **Tool Call (Simulated Example):** `run_shell_command("rg --context 5 \"use std::str::FromStr;\" submodules/rust/compiler/rustc_target/src/lib.rs", "Confirm the presence and context of the unused import.")`
2.  **Analyze Search Output:** Interpret the output to gather further insights.

### 3.4 Semantic Patch Formulation

Based on the error type and contextual information, the AI agent determines the most appropriate `EditJob` to resolve the issue.

**Agent's Actions (Example for "unused import: `std::str::FromStr`"):**
1.  **Identify Error Type:** Recognize it as an "unused import" warning.
2.  **Select `EditJob` Type:** Choose `RemoveUse`.
3.  **Extract Parameters:**
    *   `target_file`: From `ticket.occurrences[0].file_path`.
    *   `use_path`: From the parsed error message (`std::str::FromStr;`).

### 3.5 Patch Application

The AI agent constructs the TOML snippet for the chosen `EditJob` and integrates it into a semantic patch file. For a new error, a new patch file (e.g., `patches/fix_<ticket_id>.toml`) would be created. If the issue is related to an existing patch (e.g., adding another `RemoveUse` to an existing patch that cleans up imports), it would append to that file.

**Agent's Actions:**
1.  **Construct TOML Snippet:**
    ```toml
    [[edits]]
    type = "RemoveUse"
    target_file = "../../..../submodules/rust/compiler/rustc_target/src/lib.rs" # Relative path as per SOP
    use_path = "std::str::FromStr;"
    ```
2.  **Create/Append Patch File:** Write or append the snippet to the designated patch file in `submodules/rust/compiler/rustc_expand_patches/patches/`.
    *   **Tool Call (Simulated Example):** `write_file("submodules/rust/compiler/rustc_expand_patches/patches/fix_ac3c99c721bf086f4eb48d162d33498c988d2a3f3781f6be5e37e8f1772e025b.toml", <TOML_CONTENT>)

### 3.6 Verification

After applying the patch, the AI agent triggers a build to confirm that the error is resolved and no new issues are introduced.

**Agent's Actions:**
1.  **Trigger Build:** Execute `cargo check` (or `cargo build`).
    *   **Tool Call (Simulated):** `run_shell_command("cargo check", "Verify the semantic patch is applied and resolves the issue.")`
2.  **Analyze Build Output:** Inspect the output for the disappearance of the target warning/error and the absence of new issues. This would involve re-running `cargo build --message-format=json` and analyzing its output, or checking the `err.txt` for plain text warnings.

### 3.7 Iteration and Completion

-   **Success:** If the error is resolved and no new issues arise, the AI marks the ticket as resolved.
-   **Failure/New Issues:** If the error persists or new issues appear, the AI would re-evaluate the problem, potentially generate new search commands, or modify the existing semantic patch in an iterative loop until a resolution is found or human intervention is required.

## 4. Example Scenario: Resolving "Unused Import" in `rustc_target`

Consider ticket `ac3c99c721bf086f4eb48d162d33498c988d2a3f3781f6be5e37e8f1772e025b`, which highlights an "unused import: `std::str::FromStr`" warning in `submodules/rust/compiler/rustc_target/src/lib.rs`.

**AI Agent's Simulated Steps:**
1.  **Loads `ticket_ac3c99c721bf086f4eb48d162d33498c988d2a3f3781f6be5e37e8f1772e025b.json`**.
2.  **Identifies** the `target_name` as "rustc_target", the `file_path` as "submodules/rust/compiler/rustc_target/src/lib.rs", and the `rendered_message` as "warning: unused import: `std::str::FromStr`".
3.  **Executes** `rg --context 5 "use std::str::FromStr;" submodules/rust/compiler/rustc_target/src/lib.rs` to confirm the import statement's presence.
4.  **Formulates** an `EditJob` of type `RemoveUse` with `target_file = "../../..../submodules/rust/compiler/rustc_target/src/lib.rs"` and `use_path = "std::str::FromStr;"`.
5.  **Creates** a new patch file `submodules/rust/compiler/rustc_expand_patches/patches/fix_ac3c99c721bf086f4eb48d162d33498c988d2a3f3781f6be5e37e8f1772e025b.toml` with the generated TOML content.
6.  **Runs `cargo check`**.
7.  **Analyzes the output of `cargo check`**. If the warning is gone, the task is considered complete for this ticket.

This SOP provides a structured approach for AI agents to participate in the automated maintenance and quality assurance of Rust projects.
