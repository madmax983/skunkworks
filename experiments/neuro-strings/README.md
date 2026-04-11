# Neuro Strings

This experiment is a hybrid of the `neuro-sim` crate and the `ferrous-strings` experiment. It maps the spiking output of an Izhikevich neural network to the physical plucking of acoustic-magnetic strings, which in turn feed back their magnetic disturbances into the neural network's synaptic weights.

## Concept
- **Parents**: `crates/neuro-sim` + `experiments/ferrous-strings`
- **Concept**: Acoustic Neural Dynamics. A Spiking Neural Network (SNN) where the continuous membrane voltages of Izhikevich neurons physically pluck a set of resonant acoustic-magnetic strings.
- **Novel trait**: Spiking Acoustic Feedback. Synchronized neural bursts pluck the strings, and the resulting physical magnetic turbulence modifies the synaptic weights.

## Lineage
- From `crates/neuro-sim`: The Izhikevich neurons and Spiking Neural Network framework.
- From `experiments/ferrous-strings`: The physical vibrating string simulation, interacting with a magnetic `Platter`.
- Novel emergence: The visual and acoustic representation dynamically couples discrete neural events to continuous physical string dynamics, demonstrating Hebbian-like plasticity driven by physical magnetic fields.

## Usage
Run the experiment with `cargo run -p neuro-strings --release`.