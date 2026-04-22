# 086. Chimera TUI Blob Elimination

Date: 2026-04-10

## Status
Accepted

## Context
The application maintained a massive duplicated `match` block inside `experiments/chimera-lang/src/tui/mod.rs` while the refactored handlers in `experiments/chimera-lang/src/tui/app/handlers/` remained disconnected. Specifically, `tui/mod.rs` retained a ~4,500 line duplication of the `run_app` event loop logic despite previous extractions into `tui/app/mod.rs`. This was a severe architectural "Blob" that hindered maintainability and resulted in code sprawl.

## Decision
We eliminated the monolithic 4,500+ line `run_app` function and all of its duplicated `match` blocks and view rendering routing from `tui/mod.rs`. The `run_tui` loop was redirected to properly consume the cleanly modularized `app::run_app` execution logic.

## Consequences

### Positive
*   **Decoupling and Simplification**: `tui/mod.rs` has been drastically reduced to a small facade (approximately 200 lines).
*   **Single Source of Truth**: The TUI event loop is now correctly executed via `app::run_app`, relying on the decoupled handlers, eliminating massive duplication.
*   **Modularity**: Shared UI utilities (`apply_glitch_fx`, `parse_grid_value`) are now cleanly maintained in the lightweight `tui/mod.rs` facade.

### Negative
*   **Refactor Risk**: Rerouting the core event loop away from the main module could introduce regressions if any subtle view logic was missed during the extraction, though previous test coverage mitigates this risk.
