# Klein Flock 🧬

> "The flock flies not in space, but in the topology itself."

**Parents**: `klein-magnetron` × `luminous-flock`

A simulation of "Luminous Boids" (firefly-boids) inhabiting the surface of a Klein Bottle.

## 🧬 Genetic Lineage

*   **From `luminous-flock`**: The boid behavior (separation, alignment, cohesion) and the pulse-synchronization mechanic (Kuramoto model).
*   **From `klein-magnetron`**: The topological manifold (Figure-8 immersion) and the visualization logic (wireframe rendering, projection).
*   **Novel Mutation**: The flocking logic has been adapted to handle non-orientable wrapping. When a boid traverses the "twist" boundary (U-wrap), its vertical coordinate (V) and vertical velocity are inverted.

## 🕹️ Controls

*   **Arrows**: Rotate camera (Left/Right) / Zoom (A/D mapped in code, but UI says Arrows/WS?)
    *   Left/Right: Rotate
    *   W/S: Height
    *   A/D: Zoom
*   **P**: Pause
*   **R**: Reset
*   **Q**: Quit

## 🧪 Observations

The flock exhibits "Moebius Flocking". Sub-flocks that separate and travel around the twist can collide with themselves *upside down*. The synchronization waves must also traverse this twist, potentially leading to phase cancellation or complex interference patterns.
