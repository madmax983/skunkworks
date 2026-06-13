# Neuro-Origami 🧠📜

**Lineage:** `crates/neuro-sim` × `crates/origami`

"The paper remembers the paths we walk."

## Concept
This experiment combines **deployable origami structures** (Miura-ori) with **Spiking Neural Networks** (SNNs).

The surface of the paper is embedded with an Izhikevich neural network. As neurons fire, their electrical spikes are converted into mechanical forces that actuate the physical distance constraints of the Miura-ori mesh.

## Novel Trait
**Spike-Driven Folding:** The geometry of the origami mesh folds dynamically in response to neural activity. Coordinated neural firing leads to waves of physical contraction and expansion across the soft body surface, transforming discrete computational spikes into continuous topographic breathing.

## Tech Stack
- `neuro-sim` for Spiking Neural Network simulation.
- `origami`: Miura-ori mesh generation.
- Position Based Dynamics (PBD) for soft-body mesh simulation.
- `macroquad` for 3D rendering.
