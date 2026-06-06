# 109. Encapsulate System Attractor Audio

Date: 2026-06-06

## Status
Proposed

## Context
The internal audio implementation in the `system-attractor` crate (`audio_impl`) was exposed, violating module boundaries and leaking internal implementation details to consumers.

## Decision
Demoted `pub mod audio_impl` to `pub(crate) mod audio_impl` within `experiments/system-attractor/src/audio.rs` to establish a proper Facade pattern. The required types (like `Synth`) are selectively re-exported via `pub use audio_impl::Synth`.

## Consequences
* **Positive:** Consumers can no longer depend on the internal `audio_impl` module directly. Enforces encapsulation and prevents leaky abstractions.
* **Negative:** Slightly increases boilerplate for re-exporting internal types.
