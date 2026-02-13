# Harmonic Mycelium 🍄🎶

**Lineage:** `harmony-of-spheres` × `chaotic-mycelium`

A hybrid experiment combining N-Body gravitational physics with fungal pathfinding algorithms.

## Concept

A fungal network grows from the central star, seeking to connect with orbiting planets. The growth cost is inversely proportional to the gravitational potential—the mycelium thrives in deep gravity wells.

## Emergent Behavior

- **Gravitational Tropism:** The fungus naturally curves towards massive bodies, tracing the contours of the gravitational field.
- **Orbital Harmony:** Planets trigger musical tones when crossing the positive X-axis (inherited from `harmony-of-spheres`).
- **Dynamic Connection:** As planets move, the fungus constantly expands to maintain connection, creating a visual history of the orbital paths.

## Controls

- **Left Click:** Launch a new planet.
- **R:** Reset the simulation (clear planets and fungus).
- **H:** Toggle Harmony Mode (visualize resonant orbits - *Inherited but hidden in code, press H to see if active*).

## Implementation Details

- **Physics:** Symplectic Euler integrator for N-Body system.
- **Fungus:** A* / Dijkstra algorithm on a grid, with edge weights determined dynamically by the gravitational field of the bodies.
- **Audio:** Procedural sine wave synthesis using `macroquad::audio`.
