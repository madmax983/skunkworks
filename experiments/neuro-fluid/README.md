# `neuro-fluid` (Magneto-Neural Symbiosis)

**Lineage:** `crates/neuro-sim` × `experiments/ferrous-fluid`

A symbiotic visualization bridging a discrete, Spiking Neural Network (SNN) with a continuous magnetic fluid simulation.

## Concept
In `neuro-fluid`, the discrete nodes of a Spiking Neural Network (Izhikevich neurons) are embedded directly within the continuous 2D space of a magnetic fluid simulation (`ferrous-core` Platter + particle system).

This hybrid establishes a bidirectional **Magneto-Neural Symbiosis**:
1. **Neural -> Fluid (Macroscopic impact):** When a neuron spikes, it emits a sudden, intense magnetic burst, acting as a temporary magnetic pole. This pushes/pulls the surrounding magnetic fluid particles, sending physical ripples and waves through the environment.
2. **Fluid -> Neural (Microscopic feedback):** The localized density of the fluid particles at a neuron's spatial coordinate is sampled and converted into an external input current for that neuron.

## Predicted Phenotype
An emergent bio-magnetic entity. As the initial spike propagates through the neural ring, it should create localized magnetic storms that violently swirl the fluid. The moving fluid will then alter the firing rates of the embedded neurons via density feedback, creating dynamic, self-sustaining neural-fluid oscillations far more complex than simple propagating waves.

## Controls
- `[Q]` or `[Esc]` to quit.
- `[Enter]` to manually kick the neural network and inject a spike.