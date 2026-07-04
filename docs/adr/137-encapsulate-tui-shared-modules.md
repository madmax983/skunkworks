# 137. Encapsulate tui-shared Modules via Facade

Date: 2026-06-27

## Status
Accepted

## Context
During an architectural review by the Atlas persona, it was discovered that `crates/tui-shared/src/lib.rs` leaked internal modules via `pub mod`, breaking the intended Facade pattern.

## Decision
We changed `pub mod action`, `entity`, `region`, and `snapshot` to `pub(crate) mod` to enforce strict boundaries while preserving re-exports.

## Consequences
*   **Positive:** Enforces strict structural boundaries, hiding internal implementation details from external consumers.
*   **Positive:** Preserves the required external API functionality by retaining necessary re-exports.
