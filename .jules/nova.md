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

## [Shadows of the Genome]
**Concept:** Upgraded `light_grid` from scalar intensity to full RGB. Implemented recursive shadowcasting for `OpCode::Lumine` and `OpCode::LumineRGB`. Light now respects walls (membranes). Updated TUI to render colored light blending.
**Fate:** Pending
**Lesson:** Changing core data structures requires updating every touchpoint (tests, TUI, logic). Bresenham's algorithm is surprisingly effective for small grid shadowcasting.
