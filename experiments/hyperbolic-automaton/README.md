# Hyperbolic Automaton

**Lineage:** `hyperbolic-hell` × `chimera-automaton`

**Concept:**
A simulation of autonomous vehicles navigating the infinite Poincaré Disk model of hyperbolic space. Each vehicle is controlled by a genetic program (ChimeraVM) that evolves to survive and explore.

**Novel Trait:**
**Non-Euclidean Navigation.** The vehicles must adapt to the exponential expansion of space and the specific movement rules (Möbius transformations) of the hyperbolic plane. Their sensors perceive distance and direction in a world where parallel lines diverge and space is infinite.

## Implementation

- **Environment:** Infinite procedural dungeon on the Poincaré Disk (from `hyperbolic-hell`).
- **Agents:** Vehicles with `ChimeraVM` brains (from `chimera-automaton`).
- **Physics:** Movement via Möbius addition.
- **DNA:** Simple genome driving forward motion and random turns (currently).

## Controls

- **WASD**: Move Camera (Player).
- **Q/E**: Rotate Camera.
- **Click**: Spawn a new Vehicle.

## Technical Details

- Uses `macroquad` for rendering.
- Uses `poincare-disk` for hyperbolic geometry calculations.
- Uses `chimera-lang` for agent logic.
