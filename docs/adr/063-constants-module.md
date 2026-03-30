# 063. Constants Module

Date: 2025-06-12

## Status
Accepted

## Context
Across the ChimeraVM and various experiments, numerical values and parameters were scattered throughout the codebase. This made it difficult to adjust these values globally or maintain consistency.
For example, the `GOLDEN_FREQUENCIES` array used by the TUI audio views and the Nova signals processing logic were duplicated in multiple places.

## Decision
Create a central `Constants` module to store all shared numerical values and parameters.

## Consequences

### Positive
*   **Consistency:** All shared constants are now defined in a single location, reducing the risk of duplication and inconsistencies.
*   **Maintainability:** Adjusting a shared parameter only requires modifying the `Constants` module, making it easier to manage and update.
*   **Clarity:** The `Constants` module provides a clear and centralized view of all shared parameters used across the project.

### Negative
*   **Coupling:** The `Constants` module introduces coupling between modules that depend on it, potentially increasing the complexity of the codebase.
