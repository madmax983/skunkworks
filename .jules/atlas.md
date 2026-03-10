## YYYY-MM-DD - [Chimera TUI Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` was ~4800 lines long. Most of this was a monolithic `run_app` event loop with thousands of lines of `match` blocks handling input for over 40 different `ViewMode` variants inside a single `event::read()?` branch.
**Blueprint:** Extracted `run_app` into a new module `app/mod.rs`. Extracted the three main input handling branches (Editing, View Selector, Normal) into a new `app/handlers.rs` file. This drastically simplifies the event loop and paves the way for further domain-specific input handler files.

## YYYY-MM-DD - [VM Engine Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/vm/mod.rs` was 5,643 lines long, containing the massive `ChimeraVM` struct and thousands of lines of instruction execution logic (`exec_*` methods) inside a single `impl ChimeraVM` block.
**Blueprint:** Extracted all `exec_*` instruction handling methods (1,935 lines) into a new `ops.rs` module, significantly reducing the size of the core `mod.rs` file while maintaining the exact same public API and state visibility.
