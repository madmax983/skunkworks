# neuro-physics

**Parents**: `crates/neuro-sim` + `crates/physics-pbd`
**Concept**: Neural Muscle Contraction.

## Traits
* **Lineage from `neuro-sim`**: Biological Spiking Neural Network evaluating Izhikevich neurons.
* **Lineage from `physics-pbd`**: Soft-body continuous position-based dynamics via constraints and springs.
* **Phenotype**: This hybrid visually maps neural firing spikes directly into physical muscle contractions. A grid of neurons is simulated, and their states actively drive the constraints of a hanging soft-body tissue mesh.

## Quick Start
```bash
cargo run -p neuro-physics --headless
```
