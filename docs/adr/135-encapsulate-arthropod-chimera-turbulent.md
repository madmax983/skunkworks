# 135. Encapsulate Arthropod, Chimera-Lang, and Turbulent-Rhythms via Facade

Date: 2026-06-27

## Status
Accepted

## Context
During an architectural review by the Atlas persona, it was discovered that internal modules within `crates/arthropod/src/lib.rs`, `experiments/chimera-lang/src/lib.rs`, and `experiments/turbulent-rhythms/tests/havoc_contention.rs` leaked via `pub mod`. This breaks the Facade pattern and exposes implementation details.

## Decision
We replaced `pub mod` with `pub(crate) mod` in `crates/arthropod/src/lib.rs` and `experiments/turbulent-rhythms/tests/havoc_contention.rs`. In `experiments/chimera-lang/src/lib.rs`, we replaced `pub mod` with `pub(crate) mod` and restored the `pub mod` visibility for `compiler` as it was explicitly re-exported and needed elsewhere.

## Consequences
*   **Positive:** Enforces a strict structural boundary and encapsulation, hiding internal implementation details from external consumers.
*   **Positive:** Preserves the required external API functionality by retaining necessary `pub mod` visibility where explicitly needed.
