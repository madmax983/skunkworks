# Hyperbolic Waggle

**Parents**: `waggle-dance` + `hyperbolic-hell`

A simulation of honeybee waggle dances performed on the Poincaré Disk model of hyperbolic space.

## Concept
Bees forage for food sources in an infinite hyperbolic plane. Upon finding a source, they return to the hive (located at the origin) and perform a waggle dance. The dance communicates the direction and quality of the food source to other bees in the hive.

## Novelty
- **Hyperbolic Navigation**: Bees must navigate using non-Euclidean geometry. Distance expands exponentially as they move away from the center.
- **Geodesic Communication**: The "waggle" vector represents the initial tangent of the geodesic connecting the hive to the food source.

## Controls
- **Left Click**: Add a food source at the mouse position.
- **Space**: Spawn 10 more bees.
- **R**: Reset the simulation.

## Lineage
- **Render Engine**: Adapted from `hyperbolic-hell` (Poincaré disk tiling and Mobius transformations).
- **Simulation Logic**: Adapted from `waggle-dance` (State machine: Scout -> Return -> Dance -> Forage).
