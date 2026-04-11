# 078. TUI Views Facade Extraction

Date: 2026-04-01

## Status
Accepted

## Context
The module `experiments/chimera-lang/src/tui/views/mod.rs` previously utilized leaky wildcard exports (`pub(crate) use module::*;`). This practice violated encapsulation by indiscriminately polluting the parent namespace with internal details of the view submodules. Furthermore, the views are intricately tied to their respective compilation flags (`#[cfg(feature = "...")]`), making the implicit exports fragile.

## Decision
Refactored `experiments/chimera-lang/src/tui/views/mod.rs` into a strict Facade pattern. The wildcard exports were removed. Instead, the module explicitly imports and re-exports only the defined `render_*` functions, carefully guarded by their original `#[cfg(feature = "...")]` compilation flags.

## Consequences

### Positive
*   **Encapsulation:** Strongly enforces modular boundaries by exposing only intended API surfaces.
*   **Safety:** Explicit feature gating prevents accidental exposure or usage of view components when their required features are disabled.
*   **Maintainability:** Makes dependencies and available exports immediately visible within the facade module.

### Negative
*   **Verbosity:** Requires maintaining an explicit list of exports, which demands slight additional overhead when introducing new views.
