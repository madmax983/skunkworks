**[Title] Fix capacity overflow panic in origami
**Tangle:** The `crates/origami/src/lib.rs` file calculated capacity for `Vec::with_capacity` using user-provided input parameters, but failed to bound the capacity allocation against the process limits (`isize::MAX / size_of<T>`), causing fatal `capacity overflow` panics.
**Blueprint:** Replaced unbounded vector capacities with `if c <= isize::MAX / ...` bounds checks that fallback to returning empty instances rather than panicking on absurd input sizes.

**[Title] Encapsulate tui-shared modules
**Tangle:** The `crates/tui-shared/src/lib.rs` leaked internal modules via `pub mod`, breaking the intended Facade pattern.
**Blueprint:** Changed `pub mod action`, `entity`, `region`, and `snapshot` to `pub(crate) mod` to enforce strict boundaries while preserving re-exports.

**[Title] Enforce Module Boundaries via Facade
**Tangle:** Internal modules within `crates/git-associates`, `crates/locus`, `crates/resonance-audio`, and `crates/hyper-system` were exposed as `pub mod`, breaking the Facade pattern by leaking implementation details.
**Blueprint:** Modified the module definitions to use `pub(crate) mod` and explicitly `pub use` only the intended types/structs to ensure high cohesion and strict encapsulation.

**[Title] Enforce Module Boundaries in Experiments via Facade**
**Tangle:** Several experimental crates (`spqr-rsa`, `lensing-poetry`, `colony-concerto`, `system-attractor`, `clockwork-concerto`, `dependency-karst`, `cargo-rocket`, `phonetic-flock`, `synaptic-pachinko`, `hyperbolic-dungeon`, `heap-arena`) leaked their internal submodules directly via `pub mod`, breaking the Facade pattern and exposing implementation details.
**Blueprint:** Replaced `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` in library `lib.rs` files, and demoted to `mod` in binary `main.rs` files. Enforces a strict structural boundary while preserving the external API.

**[Title] Further Enforce Module Boundaries via Facade**
**Tangle:** Several experimental crates leaked their internal submodules directly via `pub mod`, breaking the Facade pattern and exposing implementation details. `crates/hyper-system` also leaked `monitor` and `physics` modules.
**Blueprint:** Replaced `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` in library `lib.rs` files, and demoted to `mod` in binary `main.rs` files across various experimental crates and `hyper-system`.

**[Encapsulate Chimera VM Modules]**
**Tangle:** The `chimera-lang` VM contained over 100 submodules exposed publicly as `pub mod`. This leaked the internal VM architecture and created a massive, tightly coupled surface area.
**Blueprint:** Replaced `pub mod` with `pub(crate) mod` across `vm/mod.rs`, `vm/ops/mod.rs`, `vm/systems/mod.rs`, `vm/prologue/mod.rs`, `tui/mod.rs`, and `tui/views/mod.rs`. Fixed fallout in internal tests by re-exposing necessary sub-modules for testing where needed. This enforces a facade pattern over the massive VM codebase.
