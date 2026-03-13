# 071. Chimera TUI Blob Extraction

## Status
Accepted

## Context
The TUI code for `chimera-lang` within `experiments/chimera-lang/src/tui/mod.rs` had grown into a monolithic ~4800-line "Blob". A significant portion of this file consisted of the `run_app` event loop and thousands of lines of `match` blocks handling input for over 40 distinct `ViewMode` variants within a single `event::read()?` branch. This centralized logic became highly tangled and difficult to navigate.

## Decision
Extracted the `run_app` functionality into a new dedicated application module `experiments/chimera-lang/src/tui/app/mod.rs`. The three primary input handling branches (Editing, View Selector, Normal) were separated into dedicated files within the `experiments/chimera-lang/src/tui/app/handlers/` module (`editing.rs`, `normal.rs`, `selector.rs`).

## Consequences

### Positive
*   **Decoupled Architecture:** Drastically simplifies the main event loop by delegating input handling responsibilities to specific handler modules.
*   **Scalability:** Paves the way for further extracting domain-specific input handlers as the experiment grows.
*   **Modularity:** Enhances code isolation and readability, making the system architecture more navigable.

### Negative
*   **Redirection:** Locating the input handling logic requires navigating through the `handlers` module directory hierarchy instead of referencing a single file.
