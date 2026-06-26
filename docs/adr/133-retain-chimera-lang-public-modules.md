# 133. Retain chimera-lang Public Modules

Date: 2026-06-26

## Status
Accepted

## Context
During an architectural review, the Atlas persona investigated the module boundaries in `chimera-lang/src/lib.rs`. It was discovered that internal modules such as `ast`, `vm`, `opcode`, `tui`, and `value` were intentionally exposed using `pub mod`. Typically, this breaks the intended Facade pattern by leaking implementation details. However, in this specific case, numerous integration tests and the `main.rs` binary are heavily coupled to this exact public structure and rely on it to function.

## Decision
We decided to retain the `pub mod` visibility for `ast`, `vm`, `opcode`, `tui`, and `value` within `chimera-lang/src/lib.rs`, intentionally deciding against enforcing a strict Facade pattern (`pub(crate) mod`).

## Consequences
*   **Positive:** Preserves the functionality of hundreds of integration tests and the primary binary without requiring a massive, potentially destructive refactoring. The architecture is deemed sound for its specific testing and execution requirements.
*   **Negative:** Internal modules remain technically exposed as part of the public API, which is a conscious trade-off prioritized for testing capabilities over strict encapsulation.
