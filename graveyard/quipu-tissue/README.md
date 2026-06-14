# Quipu Tissue 🧶🦠

> "The data is alive. It hangs, it breathes, it contracts."

**Parents**: `experiments/quipu-tissue` (Hybrid of `quipu` and `chimera-tissue`)

A visualization where **Quipu Cords** (Inca data structures) are simulated as soft-body physics objects. Each "Knot" is not just a digit, but a living cell containing a `ChimeraVM` organism.

## 🧬 Genetic Lineage

*   **Parent A (Structure):** `quipu` crate.
    *   Inherited: The logic of Cords, Clusters, and Knots (Simple, Long, Figure-Eight).
    *   Inherited: The hierarchical tree structure of the data.
*   **Parent B (Physiology):** `chimera-tissue` (and `physics-pbd`).
    *   Inherited: The soft-body physics simulation (Particles and Constraints).
    *   Inherited: The `ChimeraVM` driving mechanical actuators (muscles).

## 🧪 Novel Traits

*   **Physical Data Weight:** The value of a knot determines its physical mass. Higher numbers are heavier and pull the cord down.
*   **Living Knots:** Each knot executes genetic code that senses its velocity (gravity) and contracts/relaxes the cord segment above it.
*   **Emergent Homeostasis:** The cords try to "lift" themselves against gravity, creating a writhing, living data structure.

## 🎮 Controls

*   **Arrow Keys / WASD:** Move Camera.
*   **R:** Reset Simulation (Generate new random data).

## 🔬 Implementation Details

*   **Physics:** Position Based Dynamics (`physics-pbd`) simulates the cords as chains of particles.
*   **Biology:** `ChimeraVM` runs a simple "Anti-Gravity" gene on each knot.
    *   Input: Vertical Velocity.
    *   Output: Contraction factor (0.0 - 1.0).
*   **Rendering:** `macroquad` renders the cords and knots.
