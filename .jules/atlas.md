# Atlas's Journal

## [TUI Lifecycle Extraction]
**Tangle:** The Sprawl - `main.rs` in every experiment was copy-pasting terminal initialization code (enable raw mode, enter alternate screen, etc.).
**Blueprint:** Extracted `Tui` lifecycle into `crates/tui-shared`. This crate provides a RAII `Tui` struct that handles initialization and cleanup.
**Stability:** Reduced boilerplate, centralized terminal handling, ensuring consistent setup/teardown across experiments.
**Verification:** Refactored `automata-warfare` and `neuro-terminal` to use the shared crate. Verified with `cargo check` and `cargo test`.
