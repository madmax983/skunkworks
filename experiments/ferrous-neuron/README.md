# Ferrous Neuron ⚛️🧠

**"The brain is a magnet, and the thought is a field."**

A hybrid experiment combining **Spiking Neural Networks** (`synaptic-physics`) with **Magnetic Force-Directed Graphs** (`ferrous-graph`).
This visualization treats the codebase as a living brain, where files are neurons and dependencies are synapses.

## Concept
- **Neurons (Files)**: Each file in the repository is a neuron (Izhikevich model).
- **Synapses (Dependencies)**: Dependencies between files create synaptic connections.
- **Magnetic Physics**:
  - Neurons move based on magnetic forces.
  - **Spiking** triggers a massive release of "magnetic flux", repelling neighbors and writing to the magnetic platter.
  - **Hebbian Magnetism**: Neurons that fire together attract each other physically.

## Controls
- **Arrows**: Pan the view.
- **+/-**: Zoom in/out.
- **Q**: Quit.
- **R**: Reset view.

## Theoretical Basis
This experiment explores the idea of **Physical Intelligence**.
In a biological brain, neurons migrate during development. Here, the "development" is driven by the execution of the neural network itself.
The physical structure of the code (layout) adapts to its functional activity (spiking).

## Lineage
- **Parent A**: `experiments/ferrous-graph` (The Splice Surgeon) - Provided the magnetic physics and scanner.
- **Parent B**: `crates/synaptic-physics` (The Biophysicist) - Provided the Izhikevich neuron model.
- **Novel Trait**: **Magneto-Neuroplasticity**. The network reshapes itself based on its own thoughts.

## Running
```bash
cargo run -p ferrous-neuron
```
