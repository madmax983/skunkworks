# 108. Encapsulate Chimera Lang VM

Date: 2026-06-04

## Status
Proposed

## Context
The internal modules within `experiments/chimera-lang/src/lib.rs` (such as `acoustic_compiler`, `audio_source`, `constants`, `matrix_rain`) and the internal submodules of the `vm` (such as `organelles`, `systems`, `alchemy`, `babel_chaos`) were exposed as `pub mod`. This breaks the intended Facade pattern by leaking internal implementation details to downstream consumers. This creates tight coupling, where consumers might rely on deep structural elements rather than the carefully curated external API.

## Decision
Modified the module definitions to use `pub(crate) mod` across internal `chimera-lang` components (both in `lib.rs` and `vm/mod.rs`), replacing `pub mod` with `pub(crate) mod` while explicitly `pub use`ing the intended types in the `prelude` module and other designated public boundaries. This encapsulates the Virtual Machine internals.

## Consequences
*   **Positive:** Ensures high cohesion and strict encapsulation. Consumers are forced to use the intended public API via the `prelude` and explicitly exported paths.
*   **Negative:** Requires slightly more verbosity when internally routing types and enforcing boundary definitions.
