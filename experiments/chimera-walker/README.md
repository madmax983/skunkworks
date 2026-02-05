# Chimera Walker 🧬🚶

A hybrid experiment combining `walker-filesystem` and `chimera-lang`.

## Concept
The "Walker" entity traverses the filesystem terrain (where mountains are large binaries and valleys are directories).
However, instead of a fixed gait, its movement parameters (speed, bounce, step height) are controlled by a **Chimera VM** acting as its brain.

The brain receives input about the terrain (File vs Directory) and outputs gait parameters.
The genome can be mutated to evolve better walking strategies (or just glitchy chaos).

## Controls
- **M**: Mutate the brain's genome (randomly changes instructions/args).
- **R**: Reset the brain to the default genome.
- **Arrow Keys**: (Inherited from walker-filesystem, but effectively overridden by brain if brain writes speed).

## Lineage
- **Parent A**: `walker-filesystem` (Physics, IK, Terrain)
- **Parent B**: `chimera-lang` (Genetics, VM, Evolution)
- **Hybrid Vigor**: Procedural animation driven by an evolving genetic program.
