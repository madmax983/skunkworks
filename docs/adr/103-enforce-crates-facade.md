# 103. Enforce Module Boundaries via Facade

Date: 2026-05-27

## Status
Proposed

## Context
Internal modules within `crates/git-associates`, `crates/locus`, `crates/resonance-audio`, and `crates/hyper-system` were exposed as `pub mod`. This breaks the Facade pattern by leaking internal implementation details to consumers, leading to tight coupling.

## Decision
Modified the module definitions to use `pub(crate) mod` and explicitly `pub use` only the intended types/structs.

## Consequences
*   **Positive:** Ensures high cohesion and strict encapsulation. Consumers are forced to use the intended public API.
*   **Negative:** Requires more verbose `pub use` declarations in the library root.
