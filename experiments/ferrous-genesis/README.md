# Ferrous Genesis 🧬🧲

**Lineage:** `chimera-genesis` × `ferrous-graph`

## Concept
**Amorphous Cellular Automata**.
Traditional Cellular Automata (like Conway's Game of Life) live on a rigid grid. `ferrous-genesis` liberates the cells.

Each cell is a **Magnetic Particle** floating in continuous space.
The "Neighborhood" is defined by proximity, not grid index.
The "State" is not just On/Off, but a continuous **Magnetism** value.
The "Update Rule" is a **ChimeraVM Genetic Program** executed by each particle.

## Mechanics
1. **Physics**: Particles repel each other (Pauli Exclusion) but are attracted/repelled by Magnetic Forces based on their `magnetism` state.
2. **Sensation**: Each particle senses the local magnetic field and the number of neighbors.
3. **Cognition**: The particle executes its DNA (Chimera Assembly) to decide its new `magnetism` level.
   - Example DNA: "If it's too crowded (Hot), cool down (become less magnetic). If it's lonely (Cold), heat up (become attractive)."
4. **Emergence**: The particles self-organize into clusters, chains, or amorphous blobs based on the interplay between their physical motion and their programmed magnetic response.

## Controls
- `[Q]`: Quit
- `[Space]`: Pause/Resume
- `[+/-]`: Zoom In/Out

## Novel Trait
**Bio-Physical Feedback Loop**: The code controls the physics (Magnetism), which controls the topology (Position), which feeds back into the code (Sensation).
