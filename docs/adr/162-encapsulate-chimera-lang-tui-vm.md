# 162. Encapsulate chimera-lang TUI, VM ops, VM systems, and Prologue submodules via Facade

Date: 2026-07-23

## Status

Accepted

## Context

Internal modules within `experiments/chimera-lang/src/tui/views/mod.rs`, `experiments/chimera-lang/src/tui/mod.rs`, `experiments/chimera-lang/src/vm/ops/mod.rs`, `experiments/chimera-lang/src/vm/systems/mod.rs`, and `experiments/chimera-lang/src/vm/prologue/mod.rs` were exposed as `pub mod`. This breaks the intended Facade pattern by leaking implementation details to external consumers, creating high coupling and violating module boundary rules.

## Decision

We modified the module definitions to use `pub(crate) mod` and explicitly `pub use` only the intended types/structs to ensure high cohesion and strict encapsulation. Fixed broken integration tests by re-exporting `RealityMode`, `AstralState`, `GrammarRule`, `LogosEngine`, and `SirenState`.

## Consequences

External consumers of these submodules must now interact strictly with the explicitly exported API, which improves encapsulation and reduces coupling to the crate's internal directory structure. This ensures compliance with the system's architectural guidelines.
