# System Bio-Dome 🧬

**A Hybrid Experiment by The Splice Surgeon**

> "The chaos of the storm drives the pulse of the life."

## 🔬 Experiment Analysis

This experiment is a genetic hybrid of two previous systems:
- **Parent A**: `experiments/chem-sys` (Gray-Scott Reaction Diffusion)
- **Parent B**: `experiments/sys-weather` (Lorenz Attractor)

### Concept
The **Lorenz Attractor**, a system of chaotic differential equations often used to model atmospheric convection, acts as the "DNA Driver" for a **Gray-Scott** reaction-diffusion simulation.

Normally, Gray-Scott parameters `f` (feed rate) and `k` (kill rate) are static constants that determine the type of pattern formed (spots, stripes, chaos, solitions). In this Bio-Dome, the Lorenz Attractor's state variables (`x`, `z`) dynamically modulate `f` and `k` in real-time.

### Lineage & Genetics
- **Allele A (Visuals/Substrate)**: The grid simulation, Laplacian convolution, and chemical update logic are inherited from `chem-sys`.
- **Allele B (Driver/Logic)**: The chaotic integrator and state vector are inherited from `sys-weather`.
- **Emergent Trait**: The biomorphic patterns on the screen "breathe" and shift phase not randomly, but according to a deterministic chaotic rhythm. The ecosystem travels through different zones of viability as the attractor orbits.

## 🕹️ Controls

- **Q / Esc**: Quit
- **Mouse Click**: Seed chemicals at cursor location
- **Space** (Future): Pause driver?

## 📊 Technical Details

- **Mapping**:
  - `Lorenz.x` (-20..20) -> `GrayScott.f` (0.01..0.10)
  - `Lorenz.z` (0..50) -> `GrayScott.k` (0.03..0.08)
- **Integration**: Euler method for Lorenz (inherited), simple Euler for Diffusion (inherited).
