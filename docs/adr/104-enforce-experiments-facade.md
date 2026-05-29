# 104. Enforce Module Boundaries in Experiments via Facade

Date: 2026-05-27

## Status
Accepted

## Context
Several experimental crates (e.g., `spqr-rsa`, `lensing-poetry`, `colony-concerto`, `system-attractor`, `clockwork-concerto`, `dependency-karst`, `cargo-rocket`, `phonetic-flock`, `synaptic-pachinko`, `hyperbolic-dungeon`, `heap-arena`) leaked their internal submodules directly via `pub mod`, breaking the Facade pattern.

## Decision
Replaced `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` in library `lib.rs` files, and demoted to `mod` in binary `main.rs` files.

## Consequences
*   **Positive:** Enforces a strict structural boundary while preserving the external API.
*   **Negative:** None.
