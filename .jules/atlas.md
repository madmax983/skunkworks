# Atlas's Journal

## [TUI Lifecycle Extraction]
**Tangle:** The Sprawl - `main.rs` in every experiment was copy-pasting terminal initialization code (enable raw mode, enter alternate screen, etc.).
**Blueprint:** Extracted `Tui` lifecycle into `crates/tui-shared`. This crate provides a RAII `Tui` struct that handles initialization and cleanup.
**Stability:** Reduced boilerplate, centralized terminal handling, ensuring consistent setup/teardown across experiments.
**Verification:** Refactored `automata-warfare` and `neuro-terminal` to use the shared crate. Verified with `cargo check` and `cargo test`.

## [Standardize TUI Lifecycle]
**Tangle:** The Sprawl - Widespread duplication of terminal setup/teardown code in `code-metropolis`, `git_galaxy`, `karman-text-street`, and `term-fluids`.
**Blueprint:** Refactored these experiments to use `tui-shared`, which centralizes the TUI lifecycle (initialization, cleanup, event handling setup).
**Stability:** Reduced lines of code, enforced consistent terminal behavior (e.g. mouse capture), and simplified `main` functions.
**Verification:** Ran `cargo check` and `cargo test` for all affected crates.

## [Standardize TUI Lifecycle Phase 2]
**Tangle:** The Sprawl - Manual terminal setup found in `sculpt-term` and `l-system-garden`. `epicycle-draw` had inconsistent dependency paths and redundant mouse capture calls.
**Blueprint:** Refactored `sculpt-term` and `l-system-garden` to use `tui-shared`. Updated `epicycle-draw` to use workspace dependency and removed redundant `crossterm` calls.
**Stability:** Eliminated code duplication, enforced single source of truth for TUI initialization, and standardized workspace dependency usage.
**Verification:** Verified with `cargo check` and `cargo test` for affected packages.
