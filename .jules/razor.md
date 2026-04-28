## [Reduction]
**Bloat:** 1-variant enums `AudioCommand`, `VisualEffect`, `MidiEvent` across multiple packages.
**Cut:** Flattened single-variant enums into simple structs.
**Saved:** Multiple unnecessary abstract wrappers and lines of pattern-matching boilerplate / Reduced cognitive load of maintaining enums with only one possible state.

## [Reduction]
**Bloat:** Using `Vec::new()` and multiple `.push()` calls sequentially instead of using the `vec![]` macro.
**Cut:** Refactored `spawn_gear` and `spawn_anchor` in `experiments/clockwork-concerto/src/mechanism.rs` to use `vec![...]`.
**Saved:** Multiple lines of redundant `.push()` calls and resolved a `clippy::vec_init_then_push` warning.
