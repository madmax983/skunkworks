# neuro-resonance

A hybrid experiment crossing `neuro-sim` (Spiking Neural Network) and `resonance-audio` (FDTD continuous acoustic simulation).

## Concept: Sonification of SNN Dynamics
In this experiment, the biological firings (spikes) of a simulated Izhikevich neural network are translated directly into physical acoustic exciters on a 2D finite difference time domain (FDTD) wave grid. The neurons are mapped into a topological 2D grid, and their individual spiking behavior generates acoustic impulses that ripple through the resonance chamber, allowing us to physically "hear" and "see" brainwaves as continuous wave mechanics.

## Lineage
- **Parent A (neuro-sim)**: Provides the discrete Izhikevich spiking neuron dynamics and synaptic transmission network.
- **Parent B (resonance-audio)**: Provides the continuous 2D FDTD simulation and audio processing pipeline.

## Novel Trait
Mapping biological, discrete neural spike patterns directly to physical acoustic wave propagation in a localized topology.

## Setup & Running
```bash
cargo run -p neuro-resonance
cargo run -p neuro-resonance -- --headless
```
