# klein-magnetron 🧬

**Lineage:** `klein-fs` × `magnetron-decay`

A "Topological Bit Rot" simulator. This experiment visualizes the filesystem as sectors mapped onto a **Klein Bottle** surface (specifically a Figure-8 immersion). A "Radiation Wave" travels across the parametric space of the manifold.

## 🧬 Genetic Traits

*   **From `klein-fs` (Parent A):**
    *   3D scanning of the local filesystem.
    *   Mathematical topology of the Klein Bottle (Figure-8 immersion).
    *   Wireframe TUI rendering engine using `ratatui` canvas.
*   **From `magnetron-decay` (Parent B):**
    *   Simulation of magnetic media decay (coercivity, magnetization).
    *   Bit rot simulation where data flips based on entropy.
*   **Emergent Trait (The Mutation):**
    *   **Inverted Decay Topology:** Due to the non-orientable nature of the Klein Bottle, when the radiation wave wraps around the "twist" of the manifold, its polarity inverts. Instead of flipping single bits (0↔1), it inverts the entire byte topology (logic NOT), creating a visual artifact of the mathematical structure.

## 🕹 Controls

*   **Arrows:** Rotate Camera (Left/Right) / Select Sector (Up/Down - mapped to A/D in code for now).
*   **W/S:** Adjust Camera Height.
*   **Space:** Scrub/Repair the current sector (temporarily restores signal, degrades medium).
*   **+/-:** Adjust Time Scale.
*   **Q:** Quit.
