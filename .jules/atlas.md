## YYYY-MM-DD - [Semantic Bridge Extraction]
**Tangle:** The Blob - `crates/tui-shared/src/semantic.rs` was ~565 lines long, mixing several domain distinct concepts (`Entity`, `Region`, `Snapshot`, `Action`, and `PropValue`) into a single file.
**Blueprint:** Extracted the types into separate modules (`entity.rs`, `region.rs`, `snapshot.rs`, `action.rs`) under `crates/tui-shared/src/semantic/`, and converted `crates/tui-shared/src/semantic.rs` (now `semantic/mod.rs`) into a facade that `pub use`s these types. This improves cohesion, limits file length, and cleanly bounds the domain logic without breaking consumer imports.

## YYYY-MM-DD - [Chimera TUI Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` was ~4800 lines long. Most of this was a monolithic `run_app` event loop with thousands of lines of `match` blocks handling input for over 40 different `ViewMode` variants inside a single `event::read()?` branch.
**Blueprint:** Extracted `run_app` into a new module `app/mod.rs`. Extracted the three main input handling branches (Editing, View Selector, Normal) into a new `app/handlers.rs` file. This drastically simplifies the event loop and paves the way for further domain-specific input handler files.

## YYYY-MM-DD - [Normal Handler Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/app/handlers/normal.rs` was ~2300 lines long, containing a monolithic event loop `match` block.
**Blueprint:** Extracted the main input branches into a new `normal/` module, separating character inputs (`chars.rs`), directional navigation (`navigation.rs`), and other actions (`actions.rs`). The `normal/mod.rs` acts as a facade delegator for `handle_normal_input`. This improves cohesion and significantly reduces file length while maintaining the TUI input handling domain logic boundary.
## 2026-03-20 - [Extracting Execution Ops]
**Tangle:** The Blob - `experiments/chimera-lang/src/vm/mod.rs` was over 5600 lines long, with many opcode execution methods implemented directly inside it.
**Blueprint:** Created `experiments/chimera-lang/src/vm/ops/` directory, moved execution methods to submodules (`math.rs`, `stack.rs`, `flow.rs`, `grid.rs`, `io.rs`, `bio.rs`), reducing `mod.rs` by over 600 lines and improving structural cohesion.
## 2024-05-18 - [Math/Bitwise Operator Untangling]
**Tangle:** The Blob - `experiments/chimera-lang/src/vm/mod.rs` had multiple fundamental core math and bitwise operations (`Mod`, `BitAnd`, `BitOr`, `BitXor`, `BitNot`, `Shl`, `Shr`) improperly placed inside `exec_havoc_op` rather than their correct domain context inside `exec_math_op`.
**Blueprint:** Extracted these mathematical and bitwise execution blocks from `exec_havoc_op` and relocated them to `exec_math_op` within the dedicated `experiments/chimera-lang/src/vm/ops/math.rs` module. This enforces strict domain boundaries, reduces the size of `mod.rs`, and ensures that `exec_havoc_op` is solely responsible for its designated mutation domain (`HavocRate` and `HavocScope`).

## 2024-05-20 - [Extracted Flocking from Locus]
**Tangle:** The Blob/Leak - `crates/locus/src/flocking.rs` embedded Craig Reynolds' "Boids" algorithm logic into `locus`, which is supposed to be a fundamental, lightweight 2D geometry library. This couples AI simulation domain logic directly into pure math primitives.
**Blueprint:** Extracted `flocking.rs` into its own dedicated workspace crate `crates/flocking` that depends on `locus`. Updated all workspace dependents that previously imported `locus::flocking` to depend on the new `flocking` crate and use `flocking::...` imports. This strictly isolates AI algorithms from geometry math boundaries.
## 2024-06-15 - [Misc Ops Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/vm/mod.rs` still contained loosely coupled execution logic and operations for miscellaneous and esoteric behavior (`exec_prion_op`, `exec_transposon`, `exec_scavenge_op`, `exec_digest_op`, `exec_havoc_op`, `exec_char_op`, `exec_mutagen_op`, `exec_findall_op`, `handle_unknown_opcode`), representing roughly 500 lines of unrelated domain code.
**Blueprint:** Extracted these remaining miscellaneous operations into a newly created `experiments/chimera-lang/src/vm/ops/misc.rs` file. Added `misc` to `experiments/chimera-lang/src/vm/ops/mod.rs`. This cleanly delegates the remaining execution methods into a separate file, resolving the final remnants of the 'Blob' within the main `mod.rs`.
## $(date +%Y-%m-%d) - [Editing Handler Normalization]
**Tangle:** The Sprawl / The Blob - `experiments/chimera-lang/src/tui/app/handlers/editing/enter.rs` had a bloated `match` block spanning hundreds of lines. Over 30 different `ViewMode` variants duplicated identical fallback logic (`InputMode::Normal` and `input_buffer.clear()`). The matching logic for `apply_grid_edit` was also loosely repeated.
**Blueprint:** Removed the explicitly duplicated no-op variants, allowing them to cleanly fall through to the default `_ =>` handler. Consolidated `Babel` and `Weaver` which shared buffer-preserving logic. Grouped the `apply_grid_edit` logic variants into grouped arms guarded by their appropriate `#[cfg]` feature flags. This significantly reduces line count, enhances readability, and concentrates specific business logic.

## $(date +%Y-%m-%d) - [Chimera TUI Blob Elimination]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` retained a ~4,500 line duplication of the `run_app` event loop logic despite previous extractions into `tui/app/mod.rs`. This meant the application was maintaining massive duplicated match blocks inside `tui/mod.rs` while the refactored handlers in `tui/app/handlers/` remained disconnected.
**Blueprint:** Eliminated the monolithic 4,500+ line `run_app` function and all of its duplicated `match` blocks and view rendering routing from `tui/mod.rs`. Redirected the `run_tui` loop to properly consume the cleanly modularized `app::run_app` execution logic. `tui/mod.rs` now correctly acts as a small facade (reduced to ~200 lines) maintaining shared UI utilities (`apply_glitch_fx`, `parse_grid_value`).
## 2026-04-19 - [Extracted Value struct in penrose-genes]
**Tangle:** Circular dependency found in `penrose-genes` between `vm.rs` and `penrose.rs` via `Value` enum. `vm.rs` imported `PenroseTiling` from `penrose.rs`, and `penrose.rs` imported `Value` from `vm.rs`.
**Blueprint:** Extracted the `Value` enum into a new `value.rs` module, breaking the cyclic dependency. `vm.rs` and `penrose.rs` now both rely on `value.rs`.
## $(date +%Y-%m-%d) - [Chimera TUI View Router Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/app/mod.rs` had a massive `run_app` event loop where `terminal.draw` was rendering over 40 individual `ViewMode` enumerations using large `match` and `if let` blocks, tightly coupling application routing to rendering.
**Blueprint:** Extracted the massive view routing block from `terminal.draw` inside `run_app` into a new `router.rs` module (`route_view`). This keeps `mod.rs` lean and dedicated to high-level loop execution while centralizing the UI view resolution.
