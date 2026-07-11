# 143. Enforce Module Boundaries in Graveyard Crates

Date: 2026-07-02

## Status
Accepted

## Context
During an architectural review by the Atlas persona, it was discovered that several graveyard crates (`git_galaxy`, `git_rhythm`, `thread-symphony`) and experimental crates like `chimera-lang` were leaking their internal submodules directly via `pub mod`. A previous automated attempt to fix this issue inadvertently broke compilation by stripping or misplacing the `#[cfg(feature = "...")]` conditionals on module exports.

## Decision
We enforced strict module boundaries by replacing the exposed `pub mod` declarations with `pub(crate) mod` across the affected crates. To maintain functionality for external consumers, we explicitly re-exported the necessary APIs using `pub use module::*;`. Crucially, we preserved all `#[cfg(feature = "...")]` flags, ensuring they were correctly applied above both the `pub(crate) mod` declarations and their corresponding `pub use` re-exports.

## Consequences
*   **Positive:** Strengthens the encapsulation of internal implementation details across the graveyard and experimental crates without breaking conditional compilation rules. Enforces consistency with the system-wide Facade pattern architecture.
*   **Negative:** Increases the verbosity of the module entry points, requiring careful maintenance of `#[cfg(feature = "...")]` flags on both the module definition and its facade re-export.
