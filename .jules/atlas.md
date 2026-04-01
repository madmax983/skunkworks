## YYYY-MM-DD - [Semantic Bridge Extraction]
**Tangle:** The Blob - `crates/tui-shared/src/semantic.rs` was ~565 lines long, mixing several domain distinct concepts (`Entity`, `Region`, `Snapshot`, `Action`, and `PropValue`) into a single file.
**Blueprint:** Extracted the types into separate modules (`entity.rs`, `region.rs`, `snapshot.rs`, `action.rs`) under `crates/tui-shared/src/semantic/`, and converted `crates/tui-shared/src/semantic.rs` (now `semantic/mod.rs`) into a facade that `pub use`s these types. This improves cohesion, limits file length, and cleanly bounds the domain logic without breaking consumer imports.

## YYYY-MM-DD - [Chimera TUI Blob Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/mod.rs` was ~4800 lines long. Most of this was a monolithic `run_app` event loop with thousands of lines of `match` blocks handling input for over 40 different `ViewMode` variants inside a single `event::read()?` branch.
**Blueprint:** Extracted `run_app` into a new module `app/mod.rs`. Extracted the three main input handling branches (Editing, View Selector, Normal) into a new `app/handlers.rs` file. This drastically simplifies the event loop and paves the way for further domain-specific input handler files.

## YYYY-MM-DD - [Normal Handler Extraction]
**Tangle:** The Blob - `experiments/chimera-lang/src/tui/app/handlers/normal.rs` was ~2300 lines long, containing a monolithic event loop `match` block.
**Blueprint:** Extracted the main input branches into a new `normal/` module, separating character inputs (`chars.rs`), directional navigation (`navigation.rs`), and other actions (`actions.rs`). The `normal/mod.rs` acts as a facade delegator for `handle_normal_input`. This improves cohesion and significantly reduces file length while maintaining the TUI input handling domain logic boundary.

## YYYY-MM-DD - [TUI Views Facade Extraction]
**Tangle:** The Leak - `experiments/chimera-lang/src/tui/views/mod.rs` was acting as a leaky abstraction by using wildcard exports (`pub(crate) use module::*;`) for all 7 of its submodules (`bio`, `audio`, `magic`, etc.). This caused over 70 `render_*` functions to be indiscriminately dumped into the `views::` namespace, breaking explicit boundary contracts and making it extremely difficult to track dependencies or manage `#[cfg(...)]` feature flag boundaries correctly.
**Blueprint:** Refactored `experiments/chimera-lang/src/tui/views/mod.rs` into a strict Facade pattern. Instead of `use *`, it now explicitly enumerates exactly which `render_*` functions are exported from which submodule, and carefully applies the necessary `#[cfg(feature = "...")]` guards above each export. This ensures high cohesion within the `views` directory, tightly controls the public API contract, and correctly enforces feature-flag-driven compilation boundaries without leaking internal module details to the rest of the TUI app.
