# 164. Enforce Module Boundaries via Facade in arthropod, chimera-lang, and turbulent-rhythms

Date: 2026-07-23

## Status

Proposed

## Context

Internal modules within `crates/arthropod/src/lib.rs`, `experiments/chimera-lang/src/lib.rs`, and `experiments/turbulent-rhythms/tests/havoc_contention.rs` leaked via `pub mod`, breaking the Facade pattern and exposing implementation details.

## Decision

We enforced strict module boundaries by replacing `pub mod` with `pub(crate) mod` in `crates/arthropod/src/lib.rs` and `experiments/turbulent-rhythms/tests/havoc_contention.rs`. In `experiments/chimera-lang/src/lib.rs`, we replaced `pub mod` with `pub(crate) mod` and restored the `pub mod` visibility for `compiler` as it was explicitly re-exported and needed elsewhere.

## Consequences

External consumers must now interact strictly with the root facade (or explicitly intended modules), which improves encapsulation and reduces coupling to the crate's internal directory structure.
