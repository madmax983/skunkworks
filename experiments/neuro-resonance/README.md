# 🧬 Neuro-Resonance

This experiment acts as a continuous 2D acoustic wave tank where the discrete firing cascades of an embedded Spiking Neural Network (SNN) are translated into physical "plucks". It embodies the "Bio-Acoustic Rhythm" concept.

**Lineage:**
- From `crates/neuro-sim`: Accurate Izhikevich neurons and synaptic networking.
- From `crates/resonance-audio`: A continuous 2D Finite Difference Time Domain (FDTD) solver for acoustic wave propagation.
- **Novel Trait**: Bio-Acoustic Feedback. Neural spikes physically strike the continuous acoustic medium.

## Execution
```bash
cargo run -p neuro-resonance
```
Press 'p' to randomly inject current (pluck) into the neural network, creating spreading cascades of spikes that physically interact with the acoustic medium.
