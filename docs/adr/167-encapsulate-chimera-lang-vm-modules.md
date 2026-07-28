# ADR 167: Enforce Module Boundaries via Facade in chimera-lang VM

## Status
Proposed

## Context
The `experiments/chimera-lang/src/vm/mod.rs` leaked numerous internal modules directly via `pub mod`, breaking the Facade pattern and exposing all implementation details.

## Decision
Replaced `pub mod` with `pub(crate) mod` across internal VM modules to enforce strict boundaries. Re-exported needed modules using `pub use` to maintain the API surface required by integration tests and binaries, preserving structural integrity.

## Consequences
Implementation details of the chimera-lang VM are successfully encapsulated. External consumers use the curated API, which ensures high cohesion and strict encapsulation. Structural integrity for tests and binaries is preserved.
