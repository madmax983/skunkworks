# Fluid Rain 💧

**A Hybrid Experiment by The Splice Surgeon**

> "The code melts into the void."

## 🔬 Experiment Analysis

This experiment combines source code visualization with fluid dynamics.
- **Parent A**: `experiments/source-rain` (Falling Code)
- **Parent B**: `experiments/term-fluids` (SPH Fluid Simulation)

### Concept
Code characters fall from the sky (like "The Matrix" or `source-rain`). When they hit the bottom of the terminal, they don't just disappear—they **liquefy**. The characters transform into fluid particles that accumulate, slosh, and settle at the bottom, governed by Smoothed Particle Hydrodynamics (SPH).

### Lineage & Genetics
- **Allele A (Precipitation)**: `RainManager` adapts the file-scanning and falling text logic from `source-rain`.
- **Allele B (Hydrodynamics)**: `FluidSolver` adapts the SPH physics engine (Poly6/Spiky kernels) from `term-fluids`.
- **Emergent Trait**: Transition of state from "discrete information" (text) to "amorphous matter" (fluid).

## 🕹️ Controls

- **Q / Esc**: Quit

## 📊 Technical Details

- **Physics**: Naive O(N^2) SPH implementation.
- **Rendering**: `ratatui` Canvas.
- **State Transition**: Rain drops carry kinetic energy (speed) which translates to initial velocity for fluid particles upon impact.
