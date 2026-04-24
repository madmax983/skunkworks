# 087. TUI Navigation Handler Refactoring

Date: 2026-04-24

## Status
Accepted

## Context
The navigation handlers for directional keys (`Up`, `Down`, `Left`, `Right`) in the TUI application contained massive `match` blocks evaluating `app_state.view_mode`. This caused a pyramid-of-doom nesting structure and degraded code readability, making it hard to find and modify navigation logic for specific views. The logic was also mixed with general normal mode handler functions.

## Decision
Extracted the large `match app_state.view_mode` blocks into domain-specific helper functions (e.g., `handle_genome_down`, `handle_alchemy_up`) within `experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs`. Existing `#[cfg(feature = "nova")]` and other feature flags were preserved on both the match arms and newly extracted functions to maintain correctness.

## Consequences

### Positive
*   **Readability**: The flat structure of the helper functions removes deep nesting.
*   **Maintainability**: View-specific navigation logic is now isolated and easily locatable.
*   **Separation of Concerns**: The core navigation handlers simply route actions to the appropriate domain function.

### Negative
*   **Number of Functions**: Increases the total number of functions in the `navigation.rs` file.
