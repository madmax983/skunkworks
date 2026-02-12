# Harmonic Mycelium

A hybrid of `harmony-of-spheres` (N-Body Audio) and `chaotic-mycelium` (Fungal Pathfinding).

## Concept
A solar system where the interstellar medium is a fungal network. The mycelium grows along gravitational potential lines, seeking mass (planets).

## Lineage
*   **Parent A**: `experiments/harmony-of-spheres`
    *   Allele: N-Body Physics Engine (`physics.rs`)
    *   Allele: Pentatonic Audio Synthesizer (`audio.rs`)
    *   Allele: "Harmony Mode" concept (crossing axis triggers sound)
*   **Parent B**: `experiments/chaotic-mycelium`
    *   Allele: Dijkstra-based fungal growth (`fungus.rs`)
    *   Allele: Grid-based substrate interaction
*   **Novelty**: `GravityMap` substrate.
    *   The "Cost" of growth is inversely proportional to gravitational potential.
    *   Fungi grow "downhill" into gravity wells.
*   **Emergent Trait**: Conductive Mycelium.
    *   Planets only produce sound when crossing the axis IF they are connected to the central Star by the mycelium.
    *   The network must "bridge the void" to enable the song.

## Controls
*   **Left Click**: Launch a new planet.
*   **Space**: Clear system (reset to just Star).

## Implementation Details
*   **Physics**: Simple symplectic Euler integrator.
*   **Fungus**: Dijkstra pathfinding re-run every frame on a 150x100 grid mapped to the dynamic N-Body system.
*   **Audio**: Procedural sine wave synthesis using `macroquad::audio`.
