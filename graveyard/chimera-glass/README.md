# Chimera Glass 🧬🧊

**Parents:** `hyper-glass` (Environment) × `chimera-lang` (Life)

## 🔬 Concept

**Chimera Glass** is a 4D Spin Glass (XY Model) inhabited by biological agents (ChimeraVM).

- **The Environment:** A 4D Hypercube where each vertex contains a magnetic spin (angle). The system temperature is driven by your CPU load, and the external magnetic field is driven by your RAM usage.
- **The Life:** ChimeraVM agents live on the vertices of the hypercube. They possess a genome (DNA) that dictates their behavior.
- **The Interaction:** Agents can "flip" the local spin. If the flip aligns the spin with its neighbors (lowering local energy), the agent harvests the released energy. If the flip creates frustration (increasing energy), the agent must pay the energy cost.

## 🧬 Biological Maxwell's Demons

This experiment tests whether biological agents can evolve to become efficient "Maxwell's Demons" — entities that decrease the entropy of a system by intelligently sorting energetic states.

- **Exothermic Flip:** $\Delta E < 0$ -> Agent gains Energy.
- **Endothermic Flip:** $\Delta E > 0$ -> Agent loses Energy.

Agents that efficiently cool the system will survive and reproduce. Agents that create chaos (heat) will starve.

## 🕹️ Controls

- **Arrow Keys:** Rotate Camera (3D Projection).
- **W/S:** Zoom In/Out.
- **System Metrics:**
    - **CPU Load:** Increases Temperature (Background Thermal Noise).
    - **RAM Usage:** Increases Field Strength (Aligns spins to 0).
    - **Swap/Load:** Distorts the 4D Geometry.

## 📦 Run

```bash
cargo run -p chimera-glass
```
