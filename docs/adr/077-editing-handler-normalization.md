# 077. Editing Handler Normalization

Date: 2026-04-04

## Status
Accepted

## Context
The "Blob" anti-pattern had taken hold in `experiments/chimera-lang/src/tui/app/handlers/editing/enter.rs`. The file contained a bloated `match` block spanning hundreds of lines where over 30 different `ViewMode` variants duplicated identical fallback logic (`InputMode::Normal` and `input_buffer.clear()`). The logic for calling `apply_grid_edit` was also loosely repeated across multiple variants. This made the file overly large, difficult to read, and violated DRY principles.

## Decision
We normalized the editing handler by removing explicitly duplicated no-op variants, allowing them to cleanly fall through to a default `_ =>` handler. We consolidated variants that shared buffer-preserving logic, such as `Babel` and `Weaver`. Furthermore, we grouped the `apply_grid_edit` logic into consolidated arms guarded by their appropriate `#[cfg]` feature flags.

## Consequences

### Positive
*   **Maintainability:** Significantly reduced the line count and enhanced readability.
*   **Cohesion:** Business logic is more concentrated and easier to manage.

### Negative
*   **Complexity:** Grouping match arms with complex `#[cfg]` feature flags can slightly increase cognitive load when reading the matching conditions.
