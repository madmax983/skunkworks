## [Manifold Shaping]
**Concept:** Added `Topology` enum and `OpCode::Shape` to `chimera-lang` to dynamically change the grid topology (Plane, Torus, Cylinder, Klein, Mobius). Refactored all movement and diffusion logic to respect the current topology.
**Fate:** Merged
**Lesson:** Centralizing coordinate logic into `normalize_coords` was crucial. Testing weird topologies requires careful edge case analysis.

## [Metabolic Control]
**Concept:** Added `OpCode::Metabolism(rate)` to `chimera-lang`. Controls execution speed (Hibernation, Normal, Overclock) with quadratic energy cost. Implemented reflex-based wake-up from hibernation.
**Fate:** Merged
**Lesson:** Adding a loop to `step()` was easy, but `trigger_reflex` had to be updated to force wake-up to avoid infinite hibernation. Tests for recursion limits in existing suite are fragile.
