# Gray-Origami 🦢🧪

"Reaction-Diffusion Morphogenesis of Soft Bodies"

A 2D Gray-Scott chemical simulation runs on the surface of a Miura-ori procedural mesh. The chemical Turing patterns physically warp and fold the 3D space they are running on, creating an organism that changes shape based on its own internal chemical reactions.

## Lineage

- **Parent A:** `crates/gray-scott` - Provides the continuous 2D chemical reaction-diffusion grid.
- **Parent B:** `crates/origami` - Provides the Miura-ori mesh geometry and relies on `physics-pbd` for soft-body physics.

## Novel Trait

The concentration of the "kill" chemical ('V') locally increases the rest-length of the distance constraints (causing it to expand), while the "feed" chemical ('U') decreases it (causing it to crumple). This creates a bidirectional visual feedback loop where the 2D computational domain actively bends the 3D physical domain.
