# Retinal Glitch 👁️💥

**Genesis Experiment**: `retinal-glitch`

> "The eye alters the observed."

A biologically-inspired retinal simulation where perception distorts reality.

## Concept
This experiment simulates a simplified mammalian retina observing a procedural visual field.
However, the system is closed-loop: the spiking activity of the **Ganglion Cells** feeds back into the visual field generator, warping the space-time of the input.

## Neural Architecture
The retina is modeled as a 3-layer network:
1.  **Photoreceptors**: Transduce light intensity into graded potentials.
2.  **Horizontal/Bipolar Cells**: Implement **Center-Surround** receptive fields (Difference of Gaussians) to detect edges and contrast.
3.  **Ganglion Cells**: **Izhikevich Neurons** that fire action potentials when stimulated by Bipolar cells.

## The Glitch
When a Ganglion cell spikes, it exerts a "force" on the input visual field at its corresponding location. High neural activity causes the visual input to melt, swirl, and distort—creating a feedback loop between the observer and the observed.

## Controls
*   **SPACE**: Reset the distortion field (Saccade).

## Tech Stack
*   **Rust**
*   **Macroquad** for visualization (WASM-ready).
*   **Izhikevich** neuron model.

## Running
```bash
cargo run -p retinal-glitch
```
