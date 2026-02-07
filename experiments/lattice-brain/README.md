# Lattice Brain 🧠💎

> "The geometry of the crystal determines the song of the mind."

A hybrid experiment combining 3D crystallographic lattice generation with sonified neural networks.

## Lineage
- **Parent A**: `experiments/lattice-hunter` (3D Lattice Optimization in TUI)
- **Parent B**: `experiments/synaptic-pachinko` (Sonified Izhikevich Neurons)

## Concept
Neurons are arranged in perfect 3D crystal lattices (Simple Cubic, Body-Centered Cubic, Face-Centered Cubic). The user can switch between lattice types and inject current into random neurons. The neural activity is visualized in a TUI and (optionally) sonified.

## Features
- **3D TUI Rendering**: Uses `ratatui` + `nalgebra` to project 3D points and edges to the terminal.
- **Crystallography**: Generates SC, BCC, and FCC lattices.
- **Neural Physics**: Uses `synaptic-physics` (Izhikevich model) to simulate spiking neurons.
- **Audio Sonification**: (Optional) Uses `cpal` to sonify the mean field voltage of the network.

## Controls
- `Space`: Switch Lattice Type (SC -> BCC -> FCC)
- `I`: Inject current into a random neuron
- `Arrow Keys`: Rotate camera
- `+ / -`: Zoom in/out
- `Q`: Quit

## Audio Note
This crate has an `audio` feature which is disabled by default. To enable audio, you need `alsa` development libraries installed on your system (Linux).

```bash
cargo run --features audio
```
