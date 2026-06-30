# neuro-physics

**Parents**: `crates/neuro-sim` + `crates/physics-pbd`
**Concept**: Neural Muscle Contraction

## Traits
* **Lineage from `neuro-sim`**: Biological Spiking Neural Network (SNN) based on the Izhikevich model.
* **Lineage from `physics-pbd`**: Continuous Euclidean soft-body physics engine using Position Based Dynamics.
* **Phenotype**: This hybrid acts as a bidirectional bio-mechanical loop. Spiking behavior from the SNN translates into immediate muscular contraction in the PBD soft-body mesh. When a neuron spikes, it contracts the corresponding soft-body actuator constraints, simulating neural muscular tissue.

## Quick Start
```bash
cargo run -p neuro-physics --headless
```
