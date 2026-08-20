# Neuro Sim

A high-level Spiking Neural Network (SNN) simulation crate powered by the [`Izhikevich`] neuron model.

This crate provides a [`Network`] abstraction that manages a collection of [`Izhikevich`] neurons
connected by [`Synapse`]s. It handles spike propagation, synaptic delays, and weights.

## The Model

- **Neurons**: Uses the Izhikevich model which balances biological plausibility with performance.
- **Synapses**: Directed connections with:
    - **Weight**: Strength of the connection (positive = excitatory, negative = inhibitory).
    - **Delay**: Discrete time steps before a spike reaches the target.
- **Time**: Discrete steps. By convention, 1 step $\approx$ 1ms (though this is adjustable via interpretation).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
neuro-sim = "0.1.0"
```

## Hero's Journey: Building a Brain

```rust
fn main() {
    use neuro_sim::Network;

    // 1. Create a blank network
    let mut brain = Network::new();

    // 2. Add two neurons
    let sensory = brain.add_neuron(); // Neuron 0
    let motor = brain.add_neuron();   // Neuron 1

    // 3. Connect them (Sensory -> Motor)
    //    Weight: 15.0 (Strong excitation)
    //    Delay: 0 (Immediate effect in next step)
    brain.add_synapse(sensory, motor, 15.0);

    // 4. Simulate!
    // We'll inject current into the sensory neuron to make it fire.
    for t in 0..10 {
        // Input: 20.0 units to Neuron 0, 0.0 to Neuron 1
        brain.step(&[20.0, 0.0]);

        if brain.is_spiking(sensory) {
            println!("t={}: Sensory Neuron Spiked! ⚡", t);
        }
        if brain.is_spiking(motor) {
            println!("t={}: Motor Neuron Responded! 🦾", t);
        }
    }

}
```
