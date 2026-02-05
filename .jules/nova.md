## [Manifold Shaping]
**Concept:** Added `Topology` enum and `OpCode::Shape` to `chimera-lang` to dynamically change the grid topology (Plane, Torus, Cylinder, Klein, Mobius). Refactored all movement and diffusion logic to respect the current topology.
**Fate:** Merged
**Lesson:** Centralizing coordinate logic into `normalize_coords` was crucial. Testing weird topologies requires careful edge case analysis.
