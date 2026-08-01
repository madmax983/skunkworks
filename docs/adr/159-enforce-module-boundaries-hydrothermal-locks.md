# 159. Enforce Module Boundaries via Facade in hydrothermal-locks

Date: 2026-07-19

## Status

Accepted

## Context

The `experiments/hydrothermal-locks` crate leaked its internal modules (`agents`, `biology`, `fluid`, `grid`, and `particles`) directly via `pub mod` declarations in `lib.rs`. This broke the intended Facade architectural pattern by exposing internal implementation details to external consumers, creating high coupling and violating module boundary rules.

## Decision

We have enforced strict module boundaries by modifying `experiments/hydrothermal-locks/src/lib.rs` to declare internal modules as `pub(crate) mod` and explicitly exposing their public API using `pub use <module>::*;`.

## Consequences

External consumers of `hydrothermal-locks` must now interact strictly with the root facade, which improves encapsulation and reduces coupling to the crate's internal directory structure. This ensures compliance with the system's architectural guidelines.
