# 073. Normal Handler Extraction

## Status
Accepted

## Context
The Blob - `experiments/chimera-lang/src/tui/app/handlers/normal.rs` was ~2300 lines long, containing a monolithic event loop `match` block. This centralized logic became highly tangled and difficult to navigate.

## Decision
Extracted the main input branches into a new `normal/` module, separating character inputs (`chars.rs`), directional navigation (`navigation.rs`), and other actions (`actions.rs`). The `normal/mod.rs` acts as a facade delegator for `handle_normal_input`.

## Consequences

### Positive
*   **High Cohesion:** Input handling responsibilities are cleanly separated.
*   **Maintainability:** Significantly reduces file length while maintaining the TUI input handling domain logic boundary, making the codebase easier to read and modify.
*   **Build Times:** Faster compilation due to smaller, decoupled files.

### Negative
*   **Indirection:** Requires developers to trace input routing through the facade module to the specific handler file instead of reading a single monolithic function.
