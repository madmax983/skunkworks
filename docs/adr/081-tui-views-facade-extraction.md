# 081. TUI Views Facade Extraction

Date: 2026-04-01

## Status
Accepted

## Context
The module `experiments/chimera-lang/src/tui/views/mod.rs` relied on leaky wildcard exports (`pub(crate) use module::*;`). This pattern polluted the namespace, made it difficult to track dependencies, and obfuscated which specific views were being exposed to the rest of the application, leading to a tangled architecture.

## Decision
We replaced the wildcard exports with a strict Facade pattern. The `tui/views/mod.rs` module now explicitly exports only the defined `render_*` functions. These exports are carefully guarded by their original `#[cfg(feature = "...")]` compilation flags.

## Consequences

### Positive
*   **Encapsulation:** Enforces strict domain boundaries by explicitly stating what is exposed.
*   **Maintainability:** Easier to trace the origins of `render_*` functions.
*   **Build Optimization:** Feature-gating explicit exports reduces unnecessary compilation in specific feature contexts.

### Negative
*   **Boilerplate:** Requires adding an explicit `pub(crate) use ...` statement for each new view created in the future.
