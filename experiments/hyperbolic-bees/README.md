# Hyperbolic Bees 🐝 ∞

**Lineage:** `waggle-dance` × `hyperbolic-finder`

A simulation of bees foraging on the Poincaré Disk. The bees communicate the location of food sources using a "Hyperbolic Waggle Dance".

## Concept

In Euclidean space, honeybees communicate the vector to a food source (angle relative to sun, distance via waggle duration). In the hyperbolic plane (represented by the Poincaré Disk model), the concept of "straight line" becomes a geodesic (circular arc).

- **Hive:** Located at the center of the disk (0,0).
- **Movement:** Bees move along hyperbolic geodesics using Möbius transformations.
- **Dance:**
    - **Angle:** Since the hive is at the origin, the geodesic to any source is a straight line in the disk model. The angle is conformal (Euclidean).
    - **Distance:** The duration of the waggle run encodes the *hyperbolic distance* ($d(0, r) = 2 \text{tanh}^{-1}(r)$), which grows exponentially as sources approach the boundary.

## Controls

- **Left Click:** Add a food source at the mouse position.
- **Bees:**
    - **White:** Scouting (Random Walk)
    - **Orange:** Foraging (Moving to source)
    - **Blue:** Returning (Moving to hive)
    - **Gold:** Dancing (Recruiting at hive)
    - **Grey:** Observing (Waiting for instructions)

## Implementation Details

- Uses `poincare-disk` crate for hyperbolic geometry (Möbius addition, distance).
- Uses `macroquad` for rendering.
- State machine based on `waggle-dance`.

## Emergent Behavior

- Sources near the edge are "further away" in hyperbolic space, so bees dance longer for them.
- Bees moving to edge sources appear to slow down (in screen space) as they traverse the compressed space near the boundary.
