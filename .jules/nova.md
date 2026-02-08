## [Manifold Shaping]
**Concept:** Added `Topology` enum and `OpCode::Shape` to `chimera-lang` to dynamically change the grid topology (Plane, Torus, Cylinder, Klein, Mobius). Refactored all movement and diffusion logic to respect the current topology.
**Fate:** Merged
**Lesson:** Centralizing coordinate logic into `normalize_coords` was crucial. Testing weird topologies requires careful edge case analysis.

## [The Akashic Record]
**Concept:** Persistent global key-value storage for Chimera VMs using JSON.
**Fate:** Merged
**Lesson:** Serialization of recursive enums in Rust is easy with Serde. Global state across runs adds a new dimension to the "Evolution" simulation.

## [The Chimera Microscope]
**Concept:** Added `ViewMode::Microscope` to `chimera-lang` TUI. This view provides a detailed dashboard of the currently selected grid cell, visualizing hormone levels (RGB), waste, mutagen, light, and inspecting resident Organelles (Stack, IP). Implemented via a new `vm::microscope` module that scans the VM state.
**Fate:** Merged
**Lesson:** Adding specialized views to a TUI significantly improves debuggability of complex simulations. Handling feature flags (`#[cfg(feature = "nova")]`) in UI rendering requires careful block management.

## [Photon Racer]
**Concept:** A TUI puzzle game (`experiments/photon-racer`) combining cellular grid mechanics with discrete ray-tracing optics. Users place mirrors and obstacles to guide a photon to a target.
**Fate:** Merged
**Lesson:** Discrete ray-tracing in a TUI is surprisingly intuitive and visually satisfying. Using `locus::Vec2` for vector math simplified the reflection logic significantly.

## [The Summoning Circle]
**Concept:** Added `OpCode::Summon(filename)` to dynamically load and spawn organisms from a `bestiary/` directory. Allows modular composition of Chimera ecosystems.
**Fate:** Merged
**Lesson:** Relocating jump targets when appending DNA is tricky. A simple offset strategy works for immediate arguments, but stack-based jumps require runtime resolution or absolute addressing.
