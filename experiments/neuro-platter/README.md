# neuro-platter 🧠🍽️

**A Spiking Heatmap Scan.**

This experiment bridges the biological logic of `crates/neuro-sim` with the continuous 2D scalar fields of `crates/platter`.

## Lineage 🧬

*   **Parent A (`neuro-sim`):** Provides the Izhikevich spiking neuron models, simulating realistic biological brainwaves, continuous membrane currents, and discrete synaptic transmission delays.
*   **Parent B (`platter`):** Provides the generic 2D grid capable of accumulating, saturating, and exponentially decaying floating-point values over time.

## Novel Trait & Phenotype 🦋

In this hybrid, **neurons are physically embedded into a 2D scalar field**. The discrete nature of the Spiking Neural Network is translated into a continuous, lingering heatmap.

**Phenotype:** When a neuron spikes, it "saturates" the grid at its physical location. Because `platter` naturally decays over time, the sharp discrete biological spikes leave glowing, fading trails. This creates a visual "brain scan" where thoughts physically light up regions of the field, and recent activity glows dimly as it fades back to zero.

## Execution

```bash
cargo run -p neuro-platter
```

Press `q` or `Esc` to gracefully exit the TUI simulation.
