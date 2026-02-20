# 043. Extract Neuro Sim

* **Status:** Accepted
* **Context:**
  Multiple experiments in the repository (`neuro-fold`, `neuro-cipher`, `origami-swarm`) require simulating networks of spiking neurons. While `synaptic-physics` provides the low-level Izhikevich neuron model, the higher-level network topology—managing synapses, synaptic delays, spike propagation, and external inputs—was being duplicated or re-implemented across these projects. This led to code redundancy and potential inconsistencies in how the neural networks behaved.
* **Decision:**
  We decided to extract the network simulation logic into a shared library crate, `crates/neuro-sim`.
  This crate encapsulates:
  - **`Network`**: The main container for neurons and synapses, handling the simulation step.
  - **`Synapse`**: Defines the connection between neurons, including weight and delay.
  - **`Izhikevich`**: Re-exports or utilizes the neuron model from `synaptic-physics`.
  - **`Spike Propagation`**: Centralized logic for collecting spikes and applying synaptic currents.

  The crate depends on `synaptic-physics` for the core neuron mathematics.

* **Consequences:**
  - **Positive:** Centralized network logic ensures consistent behavior across all neural experiments. Improvements to the simulation loop (e.g., adding STDP learning, optimizing spike propagation) will benefit all dependent projects.
  - **Negative:** Introduces a dependency on `synaptic-physics` and potentially `macroquad` (if visualization helpers are included), which might increase build times slightly.
