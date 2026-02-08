# Retinal Chaos 👁️🌀

> "The Eye watches the Chaos. The Chaos reacts to the Eye."

A hybrid experiment combining biological vision simulation with deterministic chaos.

## Lineage
- **Parent A**: `experiments/system-attractor` (Lorenz Attractor Physics)
- **Parent B**: `experiments/retinal-glitch` (Spiking Neural Network Retina)

## Concept
This experiment simulates a feedback loop between an observer and the observed system.
1. A **Lorenz Attractor** is rendered to an off-screen texture (The "Visual Field").
2. A **Biological Retina** (Photoreceptors -> Horizontal Cells -> Bipolar Cells -> Ganglion Cells) observes this texture.
3. **Ganglion Spikes** are counted to measure "Visual Excitement".
4. This excitement modulates the **Rayleigh Number (Rho)** of the Lorenz Attractor.
   - More spikes = Higher Rho = More Chaos.
   - Less spikes = Lower Rho = System calms down.

## Controls
- **Arrow Keys**: Rotate Camera
- **W/S**: Zoom In/Out
- **R**: Reset Simulation

## Implementation Details
- **Rendering**: `macroquad` with custom shaders for point cloud rendering.
- **Vision**: `Retina` struct processes 128x128 input.
- **Physics**: RK4 integration of Lorenz equations.
- **Feedback**: CPU-side modulation of simulation parameters based on neural activity.
