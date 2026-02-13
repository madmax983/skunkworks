# Synaptic Physics 🧠

A lightweight, efficient Rust library for simulating biological neurons using the [Izhikevich model](https://www.izhikevich.org/publications/spikes.htm).

This crate provides the core physics logic for neural simulation experiments, balancing biological plausibility with computational performance.

## Features

*   **Izhikevich Neuron Model:** Simulates spiking and bursting behavior with only 4 parameters and 2 state variables.
*   **Preset Configurations:** Easy access to common neuron types:
    *   **Regular Spiking (RS):** Standard cortical excitatory neurons.
    *   **Fast Spiking (FS):** Inhibitory interneurons.
    *   **Chattering (CH):** Bursting cortical neurons.
*   **Numerical Integration:** Uses Euler method with internal substeps for stability.
*   **Impulse Injection:** distinct handling of continuous input currents vs. instantaneous synaptic events.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
synaptic-physics = { path = "crates/synaptic-physics" }
```

### Example

```rust
use synaptic_physics::Izhikevich;

fn main() {
    // Create a default "Regular Spiking" neuron
    let mut neuron = Izhikevich::new();

    // Simulation parameters
    let dt = 0.1; // Time step (ms)
    let input_current = 10.0; // Input current (pA/simulated unit)

    // Simulate for 100ms
    for t in 0..1000 {
        // Update the neuron state
        let (voltage, spiked) = neuron.update(dt, input_current);

        if spiked {
            println!("Spike at t = {:.1} ms!", t as f32 * dt);
        }
    }
}
```

## The Physics

The Izhikevich model uses a system of two ordinary differential equations:

1.  $v' = 0.04v^2 + 5v + 140 - u + I$
2.  $u' = a(bv - u)$

With after-spike resetting:
If $v \ge 30$ mV, then:
*   $v \leftarrow c$
*   $u \leftarrow u + d$

Where:
*   $v$: Membrane potential
*   $u$: Membrane recovery variable
*   $I$: Synaptic currents

This allows it to reproduce a wide variety of spiking patterns observed in real biological neurons.

## License

MIT / Apache-2.0
