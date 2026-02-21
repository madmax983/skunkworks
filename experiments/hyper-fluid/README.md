# Hyper-Fluid

**Lineage:**
- **Parent A:** `experiments/ferrous-fluid` (Particle Physics, Density Estimation)
- **Parent B:** `experiments/tesseract-ops` (4D Rendering, System Metrics)

**Concept:**
A 4D Fluid Simulation where the "Universe" is a Tesseract that breathes with your computer's load. The fluid particles (representing data or energy) are agitated by CPU usage, damped by Memory pressure, and pulled by Swap gravity.

**Novel Trait:**
**4D Hydrodynamics.** The fluid exists in 4 spatial dimensions (X, Y, Z, W). The W-axis represents "Load". When the system is under load, the 4th dimension expands, and the fluid dynamics shift.

**System Metrics Mapping:**
- **CPU Usage:** Temperature / Agitation. High CPU makes particles jitter.
- **Memory Usage:** Viscosity. High Memory makes the fluid "thicker" (more damping).
- **Swap Usage:** Gravity Strength. High Swap pulls particles down harder.
- **Load Average:** Tesseract Breathing / Rotation Speed. The universe expands and spins faster under load.

**Implementation Details:**
- **Physics:** 4D Particle System with SPH-like density estimation using a 4D Grid.
- **Rendering:** Particles are projected from 4D to 3D using perspective projection. Color represents depth in the 4th dimension.
- **Engine:** Macroquad (from Parent B).

**Controls:**
- **Arrow Keys:** Rotate Camera (3D)
- **W/S:** Zoom Camera
