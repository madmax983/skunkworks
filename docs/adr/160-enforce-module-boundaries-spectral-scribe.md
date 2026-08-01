# 160. Enforce Module Boundaries via Facade in spectral-scribe

Date: 2026-07-19

## Status

Accepted

## Context

The `experiments/spectral-scribe` crate leaked its internal modules (`decoder` and `encoder`) directly via `pub mod` declarations in `lib.rs`. This broke the intended Facade architectural pattern by exposing internal implementation details to external consumers, creating high coupling and violating module boundary rules.

## Decision

We have enforced strict module boundaries by modifying `experiments/spectral-scribe/src/lib.rs` to declare internal modules as `pub(crate) mod` and explicitly exposing their public API using `pub use <module>::*;`.

## Consequences

External consumers of `spectral-scribe` must now interact strictly with the root facade, which improves encapsulation and reduces coupling to the crate's internal directory structure. This ensures compliance with the system's architectural guidelines.
