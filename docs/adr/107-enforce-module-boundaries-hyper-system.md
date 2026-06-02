# 107. Enforce Module Boundaries via Facade in hyper-system

Date: 2026-06-02

## Status
Proposed

## Context
The internal `math` module within `crates/hyper-system` was leaked as a public module (`pub mod math`), violating the Facade pattern and exposing implementation boundaries. This created tight coupling where consumers could rely on internal structures rather than the intended public API.

## Decision
Changed `pub mod math` to `pub(crate) mod math` and explicitly re-exported its contents via `pub use math::*;`. Updated internal references to use the clean crate-level path.

## Consequences
* **Positive:** Tightens module encapsulation while maintaining the expected public API. Ensures high cohesion and strict encapsulation.
* **Negative:** Requires more verbose `pub use` declarations in the library root.
