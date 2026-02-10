# The Penrose Oubliette

**"A dungeon that never repeats."**

An infinite roguelike explorer based on the Penrose P3 aperiodic tiling.

## Concept
You are trapped in a quasicrystalline structure. The rooms are arranged in a pattern that has long-range order but no translational symmetry. You can never return to where you started by simply reversing your steps in a straight line, because the line doesn't exist.

(Okay, actually you can, because it's a graph, but the geometry is non-Euclidean in feel).

## Tech Stack
- **Engine**: Custom `wgpu` renderer.
- **Math**: Penrose P3 substitution (deflation) algorithm.
- **Language**: Rust.

## Controls
- **WASD / Arrows**: Move between Rhombus rooms.
- **Q / E**: Zoom in/out.
- **Esc**: Exit.

## Implementation Details
- The map is generated using the "Deflation" method, starting from a 5-fold "Sun" pattern.
- The adjacency graph is built by analyzing shared edges between the generated Rhombuses.
- "Fog of War" is implemented in the fragment shader based on distance from the player.
- The player moves discreetly between tiles, snapping to the center of the Rhombus.

## Future Ideas
- Procedural generation of enemies and loot based on local symmetry groups.
- "Inflation" mechanics where you can zoom out to a higher-level tile and control the macro-structure.
