# 🧠 Neuro-Circuit

**Genetic Cross:** `chimera-circuit` × `spinal-rhythms`

A visualization of a Neural Network embedded on a Printed Circuit Board (PCB).

## Concept

This experiment explores the idea of "Physical Intelligence". What if the layout of the brain was a circuit board? What if the delay of a thought was determined by the length of the copper trace?

- **The Body (`circuit_gen.rs`)**: A procedural PCB generator (inherited from `chimera-circuit` / `circuit-sigil`). It uses Manhattan routing to connect pads with copper traces.
- **The Brain (`neuro.rs`)**: A network of Izhikevich neurons (inherited from `spinal-rhythms`).
- **The Soul (`main.rs`)**: A simulation loop where spikes (Action Potentials) become physical light pulses traveling along the traces.

## Controls

- **R**: Regenerate the circuit (new seed).

## Implementation Details

- **Traces**: Vectors of coordinates generated via Manhattan routing.
- **Pulses**: Discrete entities moving along the trace vectors. The speed is constant, meaning longer traces introduce longer synaptic delays.
- **Neurons**: Izhikevich model (Regular Spiking).

## Lineage

- **Parent A**: `experiments/chimera-circuit` (which uses `circuit-sigil` logic).
- **Parent B**: `experiments/spinal-rhythms` (Izhikevich neuron model).
- **Emergent Trait**: Visualization of signal propagation delay as a physical distance.
