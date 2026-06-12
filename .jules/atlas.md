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
**[Title] Enforce Module Boundaries via Facade in hyper-system
**Tangle:** The `crates/hyper-system/src/lib.rs` leaked the internal `math` module via `pub mod`, breaking the Facade pattern.
**Blueprint:** Replaced `pub mod math` with `pub(crate) mod math` combined with `pub use math::*;` to enforce a strict boundary while preserving the external API.
**[Title] Enforce Module Boundaries via Facade in system-attractor**
**Tangle:** The `experiments/system-attractor/src/audio.rs` leaked its internal `audio_impl` module directly via `pub mod`, breaking the Facade pattern and exposing implementation details.
**Blueprint:** Replaced `pub mod audio_impl` with `pub(crate) mod audio_impl` to enforce a strict structural boundary while preserving the intended external API via `pub use`.
**[Title] Encapsulate VM Modules via Facade in chimera-lang**
**Tangle:** The `experiments/chimera-lang/src/vm/mod.rs` leaked over 100 internal modules directly via `pub mod`, breaking the Facade pattern and exposing all implementation details.
**Blueprint:** Replaced `pub mod` with `pub(crate) mod` across internal VM modules to enforce strict boundaries. Re-exposed specific modules like `nova_diffusion`, `nova_signals`, `prologue`, `nova`, `nova_sigil`, `babel`, `paradox`, `nova_babel_live`, `oracle`, `nova_botany`, `nova_genetics`, `pandemonium`, `memetics`, `nova_biome`, `evolution`, `nova_fluid`, `ipc`, `nova_linguistics`, `akashic`, `nova_ballistics`, `nova_alchemy_prime`, `bard`, `retina`, `verbum`, `nova_attractor`, `nova_hologram`, `nova_metazoa`, `nova_optics`, `piet`, `nova_fractal`, `nova_functional`, `nova_quipu`, `nova_raku`, `nova_simulation`, `phylogeny`, `silicon`, `nova_void`, `nova_strings`, `nova_semiotics`, `nova_security`, `nova_physics`, and `nova_origami` as they were needed by integration tests.

**[Title] Enforce Module Boundaries via Facade in syncopated-threads**
**Tangle:** The `experiments/syncopated-threads/src/lib.rs` leaked internal modules `audio`, `model`, `threads`, and `tui` via `pub mod`, breaking the Facade pattern.
**Blueprint:** Replaced `pub mod` with `pub(crate) mod` combined with `pub use <mod>::*;` to enforce strict boundaries while preserving the external API. Updated tests to match the facade API.
