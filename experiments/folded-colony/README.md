# Folded Colony 🐜📄

A hybrid experiment combining rigid origami kinematics with biological swarm agents.

## Lineage 🧬

- **Parent A**: `experiments/rigid-origami` (Mechanical Structure)
  - Provided the `MiuraGrid` kinematics and `get_vertices` logic for calculating the 3D shape of a folded sheet.
- **Parent B**: `experiments/biomimetic-bridge` (Swarm Logic)
  - Inspired the ant movement and foraging behavior.

## Concept

Ants inhabit the surface of a Miura-ori tessellation. The surface can fold and unfold (expand/contract).
When the surface is fully expanded (flat), ants must traverse the Euclidean distance between points.
When the surface folds (contracts), points that are far apart in "paper space" (UV coordinates) become close in 3D space.

**Emergent Trait**: Ants can perform "Wormhole Jumps" across the folds, using the crumpled topology to bypass long distances.

## Controls

- **Space**: Toggle auto-deployment.
- **Left/Right**: Manual expansion control.
- **O**: Toggle oscillation mode.
- **Mouse**: Orbit camera.
