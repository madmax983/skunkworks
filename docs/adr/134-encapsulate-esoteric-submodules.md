# 134. Encapsulate Esoteric Submodules via Facade

Date: 2026-06-27

## Status
Proposed

## Context
During an architectural review by the Atlas persona, it was discovered that several experimental crates (`verge-computer`, `chaos-hologram`, and `hologram-text`) leaked their internal submodules directly via `pub mod`. This breaks the intended Facade pattern and exposes implementation details that should remain internal to the crates.

## Decision
We replaced `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` in the `lib.rs` files of these experimental crates.

## Consequences
*   **Positive:** Enforces a strict structural boundary and encapsulation, hiding internal implementation details from external consumers.
*   **Positive:** Preserves the external API by explicitly re-exporting the required types/structs using `pub use`.
