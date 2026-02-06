# Chimera Sculpt 🧬🗿

**Hybrid of**: `sculpt-term` × `chimera-lang`

A "Genetic Sculpture" garden where the shape of 3D objects is defined by the DNA of a `ChimeraVM` organism. The organism's code acts as the Signed Distance Function (SDF).

## Concept

- **Phenotype**: The 3D shape visible in the terminal.
- **Genotype**: The `ChimeraVM` bytecode (DNA) that calculates the distance from a point `(x, y, z)` to the surface.
- **Mutation**: Press `M` to mutate the DNA (currently tweaks the radius) and see the shape evolve.

## Lineage

- **sculpt-term**: Provided the ASCII raymarching engine and `ratatui` integration.
- **chimera-lang**: Provided the Virtual Machine and Genetic Code structure.

## Controls

- `W/A/S/D`: Move Camera
- `M`: Mutate Genome
- `Q`: Quit

## Technical Details

The VM operates on fixed-point integers (Scale 1:100). The raymarcher injects `(x, y, z)` onto the stack, runs the gene, and expects a distance value returned on the stack.
