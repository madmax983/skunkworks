## YYYY-MM-DD - [Semantic Bridge Extraction]
**Tangle:** The Blob - `crates/tui-shared/src/semantic.rs` was ~565 lines long, mixing several domain distinct concepts (`Entity`, `Region`, `Snapshot`, `Action`, and `PropValue`) into a single file.
**Blueprint:** Extracted the types into separate modules (`entity.rs`, `region.rs`, `snapshot.rs`, `action.rs`) under `crates/tui-shared/src/semantic/`, and converted `crates/tui-shared/src/semantic.rs` (now `semantic/mod.rs`) into a facade that `pub use`s these types. This improves cohesion, limits file length, and cleanly bounds the domain logic without breaking consumer imports.

## YYYY-MM-DD - [Chimera TUI Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` was ~4800 lines long. Most of this was a monolithic `run_app` event loop with thousands of lines of `match` blocks handling input for over 40 different `ViewMode` variants inside a single `event::read()?` branch.
**Blueprint:** Extracted `run_app` into a new module `app/mod.rs`. Extracted the three main input handling branches (Editing, View Selector, Normal) into a new `app/handlers.rs` file. This drastically simplifies the event loop and paves the way for further domain-specific input handler files.

## YYYY-MM-DD - [Normal Handler Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/app/handlers/normal.rs` was ~2300 lines long, containing a monolithic event loop `match` block.
**Blueprint:** Extracted the main input branches into a new `normal/` module, separating character inputs (`chars.rs`), directional navigation (`navigation.rs`), and other actions (`actions.rs`). The `normal/mod.rs` acts as a facade delegator for `handle_normal_input`. This improves cohesion and significantly reduces file length while maintaining the TUI input handling domain logic boundary.

## YYYY-MM-DD - [Chimera TUI App Extraction Finalization]
**Tangle:** The Blob - The event loop extraction for `experiments/chimera-lang/src/tui/mod.rs` was started but left abandoned in `app/mod.rs` and its handlers. The 4000+ line monolithic `run_app` loop was duplicated and running alongside the dormant, unlinked `app/mod.rs` structure.
**Blueprint:** Deleted the duplicate `run_app` loop from `tui/mod.rs` and properly re-exported `pub(crate) mod app;`. Wired the TUI's primary loop up to `app::run_app`. Fixed all dangling imports (`super::super`), visibility boundaries (`pub(crate) fn apply_glitch_fx`), and type annotations inside the nested `handlers/` submodules to finalize the structural boundary extraction.
