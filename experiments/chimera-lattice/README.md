# Chimera Lattice 🧬💎

**Lineage:** `lattice-brain` × `chimera-lang`

## Concept: Crystallographic Intelligence

A visualization of a "Crystalline Brain" where the neurons are not simple integrate-and-fire models, but evolving **ChimeraVM** agents inhabiting the nodes of a 3D Crystal Lattice (Simple Cubic, Body-Centered Cubic, Face-Centered Cubic).

Each node in the crystal contains a genetic program (DNA) that determines how it processes signals from its neighbors. The "synapses" are the atomic bonds of the crystal structure.

## Features

- **3D Crystal Lattices**: Switch between Simple Cubic (SC), Body-Centered Cubic (BCC), and Face-Centered Cubic (FCC) structures.
- **Genetic Agents**: Each node is a `ChimeraVM` executing random genetic code (Mutation pending).
- **TUI Visualization**: Real-time 3D projection of the lattice and agent states (Resting, Firing, Refractory) in the terminal.

## Usage

```bash
cargo run -p chimera-lattice
```

## Controls

- **Space**: Switch Lattice Type (SC -> BCC -> FCC).
- **I**: Inject "Excitement" (Force random agents to Fire).
- **Arrow Keys**: Rotate the 3D view.
- **+/-**: Zoom in/out.
- **Q / Esc**: Quit.

## Splice Surgeon's Notes

"The structural rigor of the crystal meets the chaotic potential of the chimera. By arranging the agents in a perfect lattice, we constrain their communication to specific geometric paths. The intelligence here is not just in the code, but in the crystallography."
