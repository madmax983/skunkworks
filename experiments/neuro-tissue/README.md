# neuro-tissue 🧠🧵

**Lineage:** `crates/neuro-sim` × `crates/physics-pbd`

## Concept: Neural Muscle Contraction

A Spiking Neural Network embedded in a soft-body mesh simulated via Position Based Dynamics (PBD). When a neuron spikes, it acts as a muscle contraction, physically shortening the `Distance` constraints between its adjacent nodes in the PBD simulation. The physical deformation of the soft body acts as a sensory input current back to the neurons.

## Phenotype

A bio-mechanical organism that twitches and locomotes across the screen, driven entirely by an SNN, forming a self-sustaining bio-mechanical feedback loop.

## Lineage Details

- **From `neuro-sim`:** The discrete Spiking Neural Network (Izhikevich nodes).
- **From `physics-pbd`:** The continuous soft-body mesh physics engine using Distance constraints.
- **Emergent Trait:** Neural Muscle Contraction. Bidirectional feedback loop bridging SNN spikes to PBD distance constraints.
