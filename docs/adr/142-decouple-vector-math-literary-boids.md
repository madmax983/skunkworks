# 142. Decouple Vector Math and Enforce Facade in literary-boids

Date: 2026-07-02

## Status
Proposed

## Context
The experimental crate `alleles/literary-boids` (along with several graveyard crates) failed to compile due to a structural tangle: it was incorrectly referencing `tui_shared::math::Vec2`, a module that had been previously extracted out to `locus::Vec2` during a prior architectural refactoring. Additionally, it was discovered that `literary-boids/src/main.rs` was leaking its internal submodules (`boid`, `critic`, `syntax_physics`, `traces`, `world`) directly via `pub mod`, breaking the intended Facade pattern.

## Decision
We decoupled the vector math logic by redirecting the imports from `tui_shared::math::Vec2` to `locus::Vec2` and updating `alleles/literary-boids/Cargo.toml` to depend directly on the `locus` crate. Furthermore, to enforce the Facade pattern, we replaced the exposed `pub mod` statements with `pub(crate) mod` in `literary-boids/src/main.rs`, explicitly encapsulating the internal domain models. Unused distance wrapping functions were removed in favor of direct `Vec2` method calls.

## Consequences
*   **Positive:** Resolves the build failure and permanently untangles the vector math dependency. Enforcing `pub(crate) mod` strengthens the encapsulation and aligns the crate with the system-wide Facade pattern architecture.
*   **Negative:** Adds a minor amount of boilerplate for any explicit re-exports (via `pub use`) that might be required in the future for external consumers.
