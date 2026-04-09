# Ferrous-Origami 🧲🦢

"Magnetic Morphogenesis of Soft Bodies"

A 2D magnetic substrate (`Platter`) runs on the surface of a Miura-ori procedural mesh. The magnetic fields physically warp and fold the 3D space they are running on, creating an organism that changes shape based on magnetic forces.

## Lineage

- **Parent A:** `crates/ferrous-core` - Provides the continuous 2D magnetic field substrate (`Platter`).
- **Parent B:** `crates/origami` - Provides the Miura-ori mesh geometry and relies on `physics-pbd` for soft-body physics.

## Novel Trait

The concentration of the magnetic field locally increases the rest-length of the distance constraints (causing it to expand) or decreases it (causing it to crumple). This creates a bidirectional visual feedback loop where the 2D magnetic domain actively bends the 3D physical domain.
