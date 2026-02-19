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

## [Reduction]
**Bloat:** Massive manual `impl FromStr` and `impl Display` for `OpCode` in `chimera-lang` (~600 lines).
**Cut:** Replaced with `strum` derives: `EnumString` and `AsRefStr`.
**Saved:** ~550 lines of brittle boilerplate code.

## [Reduction]
**Bloat:** Re-implementation of `Vec2` logic and tuple-based math in `luminous-flock`.
**Cut:** Replaced with `tui_shared::math::Vec2`. Fixed `DNA` naming.
**Saved:** ~30 lines of boilerplate math, improved type safety and readability.

## [Reduction]
**Bloat:** `Puppeteer` struct in `experiments/metric-marionette` was a "Manager" class that only held time state and called static methods.
**Cut:** Merged `Puppeteer` logic into `Skeleton` struct (state + behavior).
**Saved:** 1 File (`puppeteer.rs`), ~50 lines of boilerplate/delegation.

## [Reduction]
**Bloat:** `experiments/dx-audit` was a redundant crate that merely wrapped `chimera-lang/examples/story_demo.rs`.
**Cut:** Deleted the entire crate.
**Saved:** 1 Crate, 1 Cargo.toml entry, 1 duplicate main.rs.

## [Reduction]
**Bloat:** Redundant `locus` dependency in `photon-racer` and `metric-marionette` when `tui-shared` already re-exports it.
**Cut:** Removed direct `locus` dependencies.
**Saved:** 2 Dependency lines, enforced single source of truth for `Vec2`.

## [Reduction]
**Bloat:** Manual `integrate` function in `system-bio-dome/lorenz.rs` and verbose neighbor calculation in `reaction.rs`.
**Cut:** Moved `integrate` to `LorenzState::update` and simplified neighbor loops.
**Saved:** Reduced cognitive load, ~20 lines of code, and improved encapsulation.

## [Reduction]
**Bloat:** `Protocol` and `Symbol` wrapper structs in `experiments/protocol-jungle`.
**Cut:** Removed wrapper structs, used `HashMap<Meaning, u8>` directly in `Agent`, and flattened `Agent` logic.
**Saved:** ~50 lines of code, 2 unnecessary abstractions.

## [Reduction]
**Bloat:** "Speculative Generality" in `crates/locus::Topology` handling `i64::MAX` grid wrapping, and "Signature Bloat" in `crates/flocking::compute_force` requiring redundant `me` argument.
**Cut:** Simplified `normalize` to use standard modulo arithmetic; Removed `me` argument from `compute_force`.
**Saved:** ~100 lines of complex overflow handling logic, removed 1 argument from critical path in physics engine.

## [Reduction]
**Bloat:** `Plant.index` dead code in `quantum-garden`; `Garden::apply_cnot` dead code re-implemented in `main.rs`.
**Cut:** Removed `index`, centralized `apply_cnot` logic, deleted manual visual updates in `main.rs`.
**Saved:** ~15 lines of duplicate logic, 1 dead field, enforced "ViewModel" pattern.

## [Reduction]
**Bloat:** `experiments/valley-forge` listed in `Cargo.toml` but missing from filesystem.
**Cut:** Removed from `Cargo.toml`.
**Saved:** Workspace build sanity.

## [Reduction]
**Bloat:** Duplicated grid seeding logic in `App::new` and `App::resize`, and inefficient `Span` allocation in `ui` loop in `experiments/biomorphic-clock`.
**Cut:** Encapsulated seeding in `Grid::random_seed_center`, implemented run-length encoding for UI rendering.
**Saved:** ~20 lines of duplicated logic, significantly reduced heap allocations per frame.

## [Reduction]
**Bloat:** `nova_astrology` (Starfall, Gaze) and `nova_gastronomy` (Cook, Savor). Niche "flavor" features adding 8 OpCodes, struct fields, and heavy modules.
**Cut:** Deleted both modules, their tests, and all references in OpCode enum, VM dispatch, and struct definitions.
**Saved:** ~500 lines of code, 8 OpCodes, 2 VM fields, 4 files.

## [Reduction]
**Bloat:** `SynapseData` struct in `biomimetic-synth` duplicating `Synapse` from `network.rs`; `Snapshot` struct allocation heavy vectors.
**Cut:** Reused `Synapse` struct, removed `SynapseData`, and cloned vectors directly.
**Saved:** ~20 lines of code, reduced memory fragmentation and cognitive load.

## [Reduction]
**Bloat:** `experiments/slime-trash` was a "Condemned" experiment with no documentation and generic implementation.
**Cut:** Deleted the entire experiment.
**Saved:** 1 Experiment, 1 Cargo.toml entry, cleaned up workspace.

## [Reduction]
**Bloat:** Redundant `width`, `height`, `depth` fields in `dependency-karst::VoxelGrid` always equal to `GRID_SIZE`.
**Cut:** Removed fields and used `GRID_SIZE` constant directly.
**Saved:** 3 struct fields, multiple initialization lines, and cognitive load of checking if dimensions vary.

## [Reduction]
**Bloat:** Manual TUI setup/teardown boilerplate in `experiments/dependency-karst/src/main.rs`.
**Cut:** Replaced with `tui_shared::Tui` RAII wrapper.
**Saved:** ~15 lines of sensitive terminal handling code.
