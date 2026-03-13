## YYYY-MM-DD - [Semantic Bridge Extraction]
**Tangle:** The Blob - `crates/tui-shared/src/semantic.rs` was ~565 lines long, mixing several domain distinct concepts (`Entity`, `Region`, `Snapshot`, `Action`, and `PropValue`) into a single file.
**Blueprint:** Extracted the types into separate modules (`entity.rs`, `region.rs`, `snapshot.rs`, `action.rs`) under `crates/tui-shared/src/semantic/`, and converted `crates/tui-shared/src/semantic.rs` (now `semantic/mod.rs`) into a facade that `pub use`s these types. This improves cohesion, limits file length, and cleanly bounds the domain logic without breaking consumer imports.

## YYYY-MM-DD - [Chimera TUI Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` was ~4800 lines long. Most of this was a monolithic `run_app` event loop with thousands of lines of `match` blocks handling input for over 40 different `ViewMode` variants inside a single `event::read()?` branch.
**Blueprint:** Extracted `run_app` into a new module `app/mod.rs`. Extracted the three main input handling branches (Editing, View Selector, Normal) into a new `app/handlers.rs` file. This drastically simplifies the event loop and paves the way for further domain-specific input handler files.

## 2026-03-13 - [Prologue State Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/vm/prologue/mod.rs` was ~2000 lines long, functioning as both a module coordinator (with 70+ submodules) and storing the core domain state definition (`PrologueState`) and its sprawling initialization.
**Blueprint:** Extracted the structural state definition (`PrologueState`) and `Default` implementations into a dedicated `state.rs` module, converting `mod.rs` to re-export it. This successfully reduced file bloat while preserving API contracts and dodging cross-dependency cascades from attempting to split the highly coupled engine logic.
