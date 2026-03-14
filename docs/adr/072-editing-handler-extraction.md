# 072. Editing Handler Extraction

## Status
Accepted

## Context
The Blob - `experiments/chimera-lang/src/tui/app/handlers/editing.rs` was almost 50k lines of tangled code handling all variants of editing mode input parsing inside a single event loop. This monolithic structure made it difficult to maintain, navigate, and compile efficiently.

## Decision
Converted the monolithic `editing.rs` file into an `editing/` module with dedicated submodules: `enter.rs`, `chars.rs`, and `actions.rs`. The input logic is cleanly delegated to these submodules, with `mod.rs` acting as a facade to route the events.

## Consequences

### Positive
*   **Reduced Coupling:** Input handling responsibilities are cleanly separated.
*   **Maintainability:** Drastically shortened file length, reducing structural bloat and making the code easier to navigate.
*   **Build Times:** Faster compilation due to smaller, decoupled files.

### Negative
*   **Indirection:** Requires developers to trace input routing through the facade module to the specific handler file instead of reading a single monolithic function.
