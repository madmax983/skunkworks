# 🧬 Neuro-Origami

**Concept**: Neural Morphogenesis of Origami.

This experiment crosses the continuous 3D soft-body constraints of `crates/origami` (simulated via `crates/physics-pbd`) with the discrete biological spiking neurons of `crates/neuro-sim`.

## Lineage
- From `origami`: The structural generation of a Miura-ori procedural mesh and its underlying mathematical formulation.
- From `physics-pbd`: The Position Based Dynamics solver that simulates the physical constraints and forces acting upon the paper.
- From `neuro-sim`: The Izhikevich Spiking Neural Network (SNN) that provides the biological logic.

## Emergent Phenotype
A bidirectional bio-mechanical feedback loop. Every vertex in the origami fold is embedded with a spiking neuron. The discrete neural spikes physically contract the distance constraints of the Miura-ori folds, acting as muscular actuators. In return, the physical stretching and compression of the paper feed back as sensory input currents to the neurons.

The resulting entity is a continuous, breathing sheet of origami tissue that folds and locomotes dynamically based on the internal, self-sustaining metabolic state of its neural network. The purely structural and geometric `origami` is brought to life by the bio-rhythmic oscillations of the `neuro-sim`.
