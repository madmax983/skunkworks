## [Reduction]
**Bloat:** Manual `match` statements for Enum <-> Integer conversion and verbose neighbor counting in `experiments/automata-warfare`.
**Cut:** `#[repr(usize)]`, `From<usize>`, and direct array indexing.
**Saved:** ~50 lines of code, reduced cognitive load in `update`.

## [Reduction]
**Bloat:** `AgentLogic` trait in `experiments/ram-bazaar` was a "One-Time Trait" implemented only by `Agent`.
**Cut:** Deleted the trait, moved `decide_bids` and `update_budget` to `impl Agent`.
**Saved:** 5 lines of boilerplate, removed unnecessary abstraction layer.

## [Reduction]
**Bloat:** `SemanticState` trait in `tui-semantic` was a "One-Time Trait" enforcing a contract for a data structure already defined by `Snapshot`. Unused `Command` enum.
**Cut:** Deleted the trait and enum. Implemented `snapshot()` directly on structs.
**Saved:** Removed unnecessary abstraction layer, simplified library API.

## [Reduction]
**Bloat:** Duplicated TUI setup/teardown boilerplate in `orbital-decay` and `semantic-spy`.
**Cut:** Replaced with `tui_shared::Tui` RAII wrapper.
**Saved:** ~20 lines of boilerplate, enforced consistent terminal handling.

## [Reduction]
**Bloat:** `Attractor` trait in `experiments/strange-loops` was a "One-Time Trait" (or rather, "Internal Polymorphism Trait") implemented by 3 structs but only used behind an enum wrapper.
**Cut:** Deleted the trait and used inherent methods directly.
**Saved:** Removed an unnecessary abstraction layer.

## [Reduction]
**Bloat:** `kind: usize` magic numbers and unused `mass`, `is_fixed` fields in `struct-soup/physics.rs`.
**Cut:** Replaced with explicit `NodeKind` enum and removed unused fields.
**Saved:** Removed dead code paths and enforced type safety.

## [Reduction]
**Bloat:** Re-implementation of `Vec2` in `experiments/cargo-rocket/src/physics.rs` while `tui-shared` provides it.
**Cut:** Replaced with `tui_shared::math::Vec2` and added `rotate` to the shared library.
**Saved:** ~100 lines of duplicate vector math code.
## [Reduction]
**Bloat:** `ToSnapshot` trait in `git-ouroboros` (implemented only by `World`).
**Cut:** Moved logic to inherent `impl World`. Deleted `semantic.rs`.
**Saved:** 1 File, 1 Trait lookup.

## [Reduction]
**Bloat:** `RainManager` in `fluid-rain`.
**Cut:** Renamed to `Rain`. Inlined `scan_files`.
**Saved:** "Manager" cognitive overhead.

## [Reduction]
**Bloat:** Repetitive neighbor iteration and boundary checking logic in `chimera-lang` diffusion functions. Dead code `experiments/sonar-swarm`.
**Cut:** Extracted `get_open_neighbors` helper. Deleted `sonar-swarm`.
**Saved:** ~30 lines of duplication, 1 broken experiment removed.
