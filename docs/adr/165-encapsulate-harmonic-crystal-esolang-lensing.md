# 165. Encapsulate harmonic-engine, crystal-fs, chimera-esolang, and lensing-poetry submodules via Facade

Date: 2026-07-23

## Status

Accepted

## Context

Several experimental crates (`harmonic-engine`, `crystal-fs`, `chimera-esolang`, and `lensing-poetry`) leaked their internal submodules directly via `pub mod`. This broke the intended Facade architectural pattern by exposing internal implementation details to external consumers, creating high coupling and violating module boundary rules.

## Decision

We enforced strict module boundaries by replacing `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` in library `lib.rs` files.

## Consequences

External consumers of these crates must now interact strictly with the root facade, which improves encapsulation and reduces coupling to the crate's internal directory structure. This ensures compliance with the system's architectural guidelines.
