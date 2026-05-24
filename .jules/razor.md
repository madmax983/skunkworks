## [Reduction]
**Bloat:** 1-variant enums `AudioCommand`, `VisualEffect`, `MidiEvent` across multiple packages.
**Cut:** Flattened single-variant enums into simple structs.
**Saved:** Multiple unnecessary abstract wrappers and lines of pattern-matching boilerplate / Reduced cognitive load of maintaining enums with only one possible state.

## [Reduction]
**Bloat:** Using `Vec::new()` and multiple `.push()` calls sequentially instead of using the `vec![]` macro.
**Cut:** Refactored `spawn_gear` and `spawn_anchor` in `experiments/clockwork-concerto/src/mechanism.rs` to use `vec![...]`.
**Saved:** Multiple lines of redundant `.push()` calls and resolved a `clippy::vec_init_then_push` warning.
## [Reduction]
**Bloat:** Layer Lasagna (`crates/tui-shared/src/semantic/` contained a nested folder hierarchy for only four very simple modules, requiring a separate `mod.rs` to wire them up).
**Cut:** Flattened the module hierarchy by moving `action.rs`, `entity.rs`, `region.rs`, and `snapshot.rs` up to the `src/` root directory, and exported them directly in `lib.rs` under a `pub mod semantic { ... }` namespace proxy to avoid breaking public API while removing the directory clutter.
**Saved:** One unnecessary `mod.rs` file, one nested directory level, and reduced cognitive load required to navigate the crate's internal structure.

## [Reduction]
**Bloat:** Layer Lasagna (`crates/poincare-disk/src/` contained `math.rs`, `geometry.rs`, `transform.rs`, `tiling.rs`, and `types.rs`, taking up 5 internal modules for a library that could easily be contained in one file).
**Cut:** Flattened the module hierarchy by moving `math.rs`, `geometry.rs`, `transform.rs`, `tiling.rs`, and `types.rs` into `lib.rs` inside `poincare-disk`.
**Saved:** Five internal files, multiple module definitions, reduced module boundaries, and lowered cognitive load when navigating the `poincare-disk` crate.

## [Reduction]
**Bloat:** Layer Lasagna (`crates/git-associates/src/` contained `model.rs` just for some structs, spreading a small crate across multiple files).
**Cut:** Flattened the module hierarchy by moving `model.rs` into `lib.rs` inside `git-associates`.
**Saved:** One file (`model.rs`), multiple lines of module exports, reduced module boundaries, and lowered cognitive load when navigating the `git-associates` crate.
