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
**Verification:** Verified with `cargo check` and `cargo test` for all affected crates.

## [Standardize TUI Lifecycle Phase 2]
**Tangle:** The Sprawl - Manual terminal setup found in `sculpt-term` and `l-system-garden`. `epicycle-draw` had inconsistent dependency paths and redundant mouse capture calls.
**Blueprint:** Refactored `sculpt-term` and `l-system-garden` to use `tui-shared`. Updated `epicycle-draw` to use workspace dependency and removed redundant `crossterm` calls.
**Stability:** Eliminated code duplication, enforced single source of truth for TUI initialization, and standardized workspace dependency usage.
**Verification:** Verified with `cargo check` and `cargo test` for affected packages.

## [Standardize TUI Lifecycle Phase 3]
**Tangle:** The Sprawl - Repeated TUI setup code in `git_rhythm`, `hyphal-commute`, and `lattice-hunter`.
**Blueprint:** Refactored these to use `tui-shared`.
**Stability:** Reduced boilerplate and enforced consistent TUI lifecycle management.
**Verification:** Verified with `cargo check` and `cargo test`.

## [Standardize TUI Lifecycle Phase 4]
**Tangle:** The Sprawl - Repeated TUI setup code in `crystal-structure`, `resonance-chamber`, `system-bio-dome`, and `schrodingers-text`.
**Blueprint:** Refactored these to use `tui-shared`.
**Stability:** Reduced boilerplate and enforced consistent TUI lifecycle management (including proper mouse capture for `crystal-structure` and `resonance-chamber`).
**Verification:** Verified with `cargo check` for all affected packages.

## [Standardize Resonance Audio]
**Tangle:** The Sprawl - Duplicate `physics.rs` and `audio.rs` in `resonance-chamber` and `chimera-resonance`.
**Blueprint:** Extracted `crates/resonance-audio` to house shared logic.
**Stability:** Reduced duplication, enforced single source of truth for physics and audio logic.
**Verification:** Verified with `cargo test`. Fixed `chimera-lang` feature compilation errors.

## [Poincare Disk Extraction]
**Tangle:** The Copy-Paste - Identical hyperbolic geometry logic duplicated across `hyperbolic-dungeon`, `hyperbolic-ants`, `hyperbolic-roots`, and `hyperbolic-finder`.
**Blueprint:** Extracted shared geometry to `crates/poincare-disk`.
**Stability:** Single source of truth for complex hyperbolic math.
**Verification:** Verified via `cargo check` on all consumers.

## [Poincare Disk Extraction Phase 2]
**Tangle:** The Copy-Paste - `hyperbolic-fs` contained a local `hyperbolic.rs` module duplicating Möbius transformation logic found in `crates/poincare-disk`.
**Blueprint:** Refactored `hyperbolic-fs` to depend on `poincare-disk` and removed the duplicate local module.
**Stability:** Reduced code duplication, enforcing single source of truth for hyperbolic geometry.
**Verification:** Verified with `cargo check` and `cargo test`.

## [Poincare Crawl Refactor]
**Tangle:** The Copy-Paste - `experiments/poincare-crawl` duplicated `Mobius` logic and `Geodesic` struct from `crates/poincare-disk`, and had raw TUI boilerplate ("The Sprawl").
**Blueprint:** Moved `Geodesic` and missing `Mobius` methods to `crates/poincare-disk`. Refactored `poincare-crawl` to use `tui-shared` and `poincare-disk`.
**Stability:** Centralized math logic, standardized TUI lifecycle.
**Verification:** `cargo check` and `cargo test` passed.

## [Market Simulation Extraction]
**Tangle:** The Copy-Paste - `experiments/market-flow`, `experiments/market-rogue`, `experiments/market-swarm`, and `experiments/chimera-market` all implemented nearly identical `Grid` and `Particle` logic for market simulation.
**Blueprint:** Extracted `Grid`, `Particle`, and `TradeEvent` to `crates/market-sim`. Standardized `Particle` to use the more capable version (with owner ID) from `chimera-market`.
**Stability:** Centralized market physics logic.
**Verification:** `cargo check` passed for all 4 experiments. `cargo test` passed for `market-rogue`.

## [Synaptic Physics Time Consistency]
**Tangle:** The Leak - `Izhikevich::update` accepted `dt` but implemented decay based on loop iterations, making the physics dependent on frame rate (simulation step size).
**Blueprint:** Refactored `crates/synaptic-physics` to use exponential decay based on `dt`, ensuring consistent behavior regardless of time step size.
**Stability:** Correct physics simulation across variable time steps.
**Verification:** Added `test_decay_consistency` comparing `dt=1.0` vs 10x `dt=0.1` updates.
