# Flock Origami

**Status:** CONDEMNED (Grace Period)
**Lineage:** `flocking` × `origami`.

## Description

This experiment crosses Craig Reynolds' continuous 2D spatial swarming mechanics (`flocking`) with the procedural Miura-ori soft body mesh generation (`origami`). The fundamental concept is "Flocking Morphogenesis", where the spatial density of the boid swarm dynamically stiffens and actuates the structural constraints of the 3D origami paper.

As boids gather and swarm tightly in specific areas of the 2D plane, their localized density is mapped to the `stiffness` and `factor` of the underlying `PbdSystem` constraints. This causes the paper to dynamically fold, crease, and crumple specifically where the swarm's concentration is highest. When the boids disperse, the paper relaxes and unfolds.

## Lineage Details
- **Parent A (flocking):** Provides the continuous 2D spatial swarming mechanics and Boid force calculations (separation, alignment, cohesion).
- **Parent B (origami):** Provides the Position Based Dynamics (`pbd`) constraint system and the mathematical `generate_miura_grid` logic for folding patterns.
- **Novel Trait:** Flocking density directly acts as an input stimulus to physically mutate the environment's structural constraints.

## Controls
- **Mouse Left Click + Drag:** Rotate Camera (Yaw / Pitch).
- **Mouse Scroll Wheel:** Zoom in / out.

## Fixes Implemented
- Created complete documentation detailing lineage and novel traits.
- Removed unused `actuators` vector and vestigial logic in `mesh_gen.rs`.
- Fixed runtime panics in headless environments by gracefully terminating if an X server connection is unavailable.
