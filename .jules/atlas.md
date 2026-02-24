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
**Tangle:** The Copy-Paste - `experiments/market-flow`, `experiments/market-rogue`, `experiments/market-swarm`, and `experiments/chimera-market` all implemented identical `Grid` and `Particle` logic for market simulation.
**Blueprint:** Extracted `Grid`, `Particle`, and `TradeEvent` to `crates/market-sim`. Standardized `Particle` to use the more capable version (with owner ID) from `chimera-market`.
**Stability:** Centralized market physics logic.
**Verification:** `cargo check` passed for all 4 experiments. `cargo test` passed for `market-rogue`.

## [Git Logic Consolidation]
**Tangle:** The Sprawl - Duplicate git parsing logic in `git-rogue` (git2 crawler), `git-landscape` (git2 history), `git-harmony` (Command diff), and `tectonic-git` (Command log).
**Blueprint:** Created `crates/git-associates`, a shared crate using `git2` to handle history, diffs, and graph crawling. Refactored all 4 experiments to use it.
**Stability:** Centralized git logic, standardized types (`Commit`, `FileChange`), and reduced code duplication.
**Verification:** Verified with `cargo check` for all affected crates. Disabled `audio` feature by default in `git-harmony` and `git-landscape` to fix build issues with `alsa-sys`.

## [Flocking Logic Extraction]
**Tangle:** The Copy-Paste - `experiments/luminous-flock`, `quantum-boids`, `tectonic-flock`, and `sono-boids` all implemented Boid flocking physics (Reynolds: Separation, Alignment, Cohesion) with slight variations and code duplication.
**Blueprint:** Created `crates/flocking` to centralize `PhysicsState` and `compute_force` (Reynolds logic). Refactored all 4 experiments to use it.
**Stability:** Single source of truth for flocking behavior. Flexible `FlockingParams` allows customization per experiment.
**Verification:** Verified with `cargo check` and `cargo test` for the new crate and all 4 experiments.

## [Quipu Audio Extraction]
**Tangle:** The Copy-Paste - `experiments/quipu-chimera` and `experiments/quipu-symphony` contained identical `audio.rs` implementations for a rhythmic audio engine.
**Blueprint:** Extracted `AudioEngine` to `crates/quipu` under an `audio` feature flag. Refactored both experiments to use the shared module.
**Stability:** Centralized audio logic, enforced strict feature gating for "silent mode" compatibility.
**Verification:** Verified `cargo check` for `quipu` (without audio due to env), `quipu-chimera`, and `quipu-symphony`. Verified `cargo test` for `quipu`.

## [Gray-Scott Extraction]
**Tangle:** The Copy-Paste - `myco-diffusion` and `system-bio-dome` implemented identical Gray-Scott reaction-diffusion physics (with slightly different parameters and precision), leading to code duplication.
**Blueprint:** Extracted `crates/gray-scott` to centralize the simulation logic. It supports both `f32` (performance) and parallel updates via `rayon` (feature-gated).
**Stability:** Single source of truth for reaction-diffusion math.
**Verification:** Verified via `cargo check` and `cargo test` on consumers.

## [Origami Logic Extraction]
**Tangle:** The Copy-Paste - `origami-constellation`, `rigid-origami`, and `origami-spores` all implemented Miura-ori mesh generation logic with slight variations (different orientations and parameter handling).
**Blueprint:** Created `crates/origami` to centralize `MiuraParams` and `MiuraOri`. Implemented `Orientation::Horizontal` (for visualizer) and `Orientation::Vertical` (for rigid/simulation) strategies. Refactored all 3 experiments to use the shared crate.
**Stability:** Centralized geometry logic, ensured edge length preservation (rigidity) in both orientations via tests.
**Verification:** Verified with `cargo check` and `cargo test`. Added unit tests for flat/folded states and edge length constraints.

## [Neuro-Sim Extraction]
**Tangle:** The Copy-Paste - `experiments/neuro-fold`, `experiments/neuro-cipher`, and `experiments/origami-swarm` all implemented identical `IzhikevichNeuron`, `Network` (SNN), and `PbdSystem` (Position Based Dynamics) logic.
**Blueprint:** Created `crates/physics-pbd` for PBD logic and `crates/neuro-sim` for SNN logic (wrapping `synaptic-physics`). Refactored all 3 experiments to use these shared crates.
**Stability:** Centralized physics and neural logic. Enforced `Clone` on `PbdSystem` to satisfy requirements.
**Verification:** Verified with `cargo check` for all affected crates and `cargo test` for `neuro-sim`.

## [Hyper System Extraction]
**Tangle:** The Copy-Paste - `experiments/hyper-glass`, `hyper-market`, `hyper-acoustics`, `tesseract-ops`, `hyper-flock`, and `hyper-ferro` all implemented identical 4D `Vec4` math and `SystemMonitor` logic (using `sysinfo`).
**Blueprint:** Extracted `Vec4` and `SystemMonitor` to a new shared crate `crates/hyper-system`.
**Stability:** Centralized 4D math and system monitoring logic. Enforced single source of truth for "Hyper" series experiments.
**Verification:** Verified with `cargo check` for all 6 affected experiments and `cargo test` for the new crate.

## [Physics PBD Decoupling]
**Tangle:** The Leak - `physics-pbd` depended on `macroquad` (a game engine) just for vector math, forcing all consumers to pull in heavy graphics dependencies.
**Blueprint:** Replaced `macroquad` with `glam` in `crates/physics-pbd`.
**Stability:** Decoupled physics logic from rendering, enabling use in TUI/server contexts.
**Verification:** Verified `experiments/bifurcation-crawler` and `experiments/chimera-tissue` still compile (due to `macroquad` re-exporting `glam`).
