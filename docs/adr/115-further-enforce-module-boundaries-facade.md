# 115. Further Enforce Module Boundaries via Facade

Date: 2026-06-14

## Status
Accepted

## Context
Various experimental crates (such as `spqr-rsa`, `lensing-poetry`, `colony-concerto`, `system-attractor`, `clockwork-concerto`, `dependency-karst`, `cargo-rocket`, `phonetic-flock`, `synaptic-pachinko`, `hyperbolic-dungeon`, `heap-arena`, `thermo-market`, `verge-computer`, `glossolalia`, `harmonic-engine`, `crystal-fs`, `spectral-scribe`, `neuro-terminal`, `quipu-serializer`, `mnem-bridge`, `memetic-market`, `git-cantata`, `syncopated-threads`) were leaking their internal submodules directly via `pub mod`. This broke the intended Facade pattern and exposed implementation details to downstream consumers.
Additionally, the `crates/hyper-system` workspace crate leaked its internal `monitor` and `physics` modules in the same way.

## Decision
We applied the Facade pattern across these affected modules to hide internal structure. In library crates (`lib.rs`), we replaced `pub mod` with `pub(crate) mod` and selectively re-exported the intended external API via `pub use <mod>::*;`. In binary crates (`main.rs`), we demoted leaked modules to private `mod` declarations.

## Consequences
*   **Positive:** Strengthens encapsulation, preventing external consumers from depending on internal directory and module structures. Reduces coupling and improves the maintainability of the affected crates.
*   **Negative:** Adds a small amount of boilerplate by requiring explicit `pub use` statements to form the boundary APIs.
