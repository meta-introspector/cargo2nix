# Tool: `write_todos`

## Category: Workflow & Task Management

## Purpose
This tool helps in managing complex tasks by breaking them down into smaller, manageable subtasks. It allows the agent to track progress, organize queries, and ensures that no steps are missed. It also provides transparency to the user about the agent's current progress and plan.

## Use Cases
*   Dividing a complex user request into distinct and manageable subtasks.
*   Keeping track of the current task and its progress.
*   Organizing complex queries that require multiple steps.
*   Ensuring all components of a request are addressed.
*   Providing the user with an overview of the execution plan and status.

## Task State Definitions

*   **`pending`**: Work has not yet begun on this subtask.
*   **`in_progress`**: Work on this subtask has started. Only one subtask should be `in_progress` at a time.
*   **`completed`**: The subtask was successfully finished without errors or issues.
*   **`cancelled`**: The subtask is no longer required due to changes in the overall task or new information.

## Key Parameters

*   **`todos`** (array of objects, **required**):
    *   The complete list of todo items. This will replace the existing list. Each item is an object with the following properties:
        *   **`description`** (string, **required**): A clear description of the task.
        *   **`status`** (string, **required**): The current status of the task (`pending`, `in_progress`, `completed`, `cancelled`).

## Methodology for Using This Tool

1.  **Initialize**: Use this tool as soon as a user request is received, if the task complexity warrants it (more than 2 steps, or a need for planning).
2.  **Track**: Keep track of every subtask added to the list.
3.  **Mark `in_progress`**: Mark a subtask as `in_progress` just before beginning work on it.
4.  **Update Progress**: The subtask list is dynamic and should reflect current plans and progress, which may evolve.
5.  **Mark `completed`**: Mark a subtask as `completed` when it has been successfully finished.
6.  **Mark `cancelled`**: Mark a subtask as `cancelled` if it is no longer needed.
7.  **Immediate Updates**: Update the todo list immediately when a subtask starts, stops, or is cancelled. Avoid batching updates.

## Examples

**1. Initializing a new list of tasks:**
```
write_todos(
    todos=[
        {"description": "Analyze existing codebase for dependencies.", "status": "pending"},
        {"description": "Formulate a refactoring plan for module X.", "status": "pending"}
    ]
)
```

**2. Updating the status of a task and adding a new one:**
```
write_todos(
    todos=[
        {"description": "Analyze existing codebase for dependencies.", "status": "completed"},
        {"description": "Formulate a refactoring plan for module X.", "status": "in_progress"},
        {"description": "Implement refactored module X with new test cases.", "status": "pending"}
    ]
)
```