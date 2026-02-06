# Klein Flock 🐦♾️

**Genetic Cross:** `klein-fs` × `luminous-flock`

A TUI simulation of a flock of boids navigating the surface of a **Klein Bottle**.

## 🧬 Lineage

*   **Parent A (`klein-fs`)**: Provided the topology and rendering engine.
    *   **Allele:** `figure_8_klein` immersion formula mapping $(u, v)$ coordinates to 3D space.
    *   **Allele:** 3D-to-2D wireframe projection logic for the terminal.
*   **Parent B (`luminous-flock`)**: Provided the swarm intelligence.
    *   **Allele:** Boid flocking rules (Separation, Alignment, Cohesion).
    *   **Allele:** Agent-based simulation structure.

## 🦋 Emergent Behavior

The boids inhabit a 2D parameter space representing the fundamental polygon of the Klein Bottle.
*   **Twist Boundary:** When boids wrap around the horizontal axis (the "twist"), their vertical position is mirrored ($y \to H-y$) and their vertical velocity is inverted ($v_y \to -v_y$).
*   **Topology in Motion:** This creates a non-orientable flock. Two boids flying parallel can separate, wrap around different axes, and meet again swimming in opposite "up" directions relative to the manifold.

## 🎮 Controls

*   **Arrows / WASD**: Rotate and zoom camera.
*   **Q**: Quit.
