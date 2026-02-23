# 🧶 Neuro-Quipu

> "The brain is a knot; thought is the unraveling."

**Neuro-Quipu** is a hybrid experiment combining Spiking Neural Networks (SNN) with the ancient Incan Quipu recording device.

## 🧬 Concept

In this simulation, the neural network's topology is mapped to a physical Quipu:

- **Vertical Cords** represent Neurons (Axons).
- **Knots** represent Synapses (Connections to other neurons).
- **Vertical Position** represents Synaptic Delay (Latency).
- **Knot Type** represents Synaptic Weight.

When a neuron spikes, a "Bead of Light" travels down the cord. When it hits a knot, it triggers a signal to the target neuron. The physical distance the bead travels corresponds to the temporal delay of the signal.

## 🔬 Lineage

- **Parent A**: `experiments/quipu-symphony` (Quipu visualization and logic)
- **Parent B**: `crates/neuro-sim` (Spiking Neural Network simulation)
- **Novel Trait**: **Physical Latency**. The geometry of the network determines its temporal properties.

## 🎮 Controls

- **Space**: Pause/Resume simulation.
- **R**: Reset network (re-roll topology).
- **Q**: Quit.

## 🧠 Emergent Behavior

Watch as the "thoughts" trickle down the cords. High-latency connections (knots near the bottom) create long-term memory loops, while low-latency connections (knots near the top) drive immediate reflexes.
