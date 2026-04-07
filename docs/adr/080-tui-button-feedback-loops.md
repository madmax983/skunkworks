# 080. TUI Button Feedback Loops

## Status
Accepted

## Context
Various TUI applications require a way to indicate the progress or result of asynchronous operations to the user (such as a network request or heavy processing). The shared `Button` widget in `crates/tui-shared/src/button.rs` previously only supported interactive states like `Hovered` and `Clicked`. This lacked a standard mechanism to communicate system-level feedback, leading to inconsistent user experiences across experiments.

## Decision
We decided to implement visual feedback loops directly into the shared TUI `Button` component state by adding `is_loading` and `is_success` properties.

When rendered:
- The `is_success` state takes the highest precedence, displaying a green background and a '✅' icon.
- The `is_loading` state takes secondary precedence, displaying a yellow background and a '⏳' icon.
- Builder methods (`.loading()` and `.success()`) have been added to allow developers to declaratively change these states.

## Consequences
### Positive
*   **Standardization:** Experiments now have a uniform, expected visual language for asynchronous button actions.
*   **Discoverability:** The new properties clearly communicate component capabilities and simplify state management on the caller's side.

### Negative
*   **Complexity:** The `Button` struct now has multiple boolean flags, which requires strict precedence logic during rendering.
*   **Visual Coupling:** The hardcoded icons ('⏳', '✅') and colors might conflict with the themes of some specific, heavily stylized experiments.
