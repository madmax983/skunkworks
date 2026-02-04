# Penrose Genes 🧬

**Lineage**: `aperiodic-citadel` × `chimera-lang`

## Concept
What if the memory of a Virtual Machine wasn't a contiguous array or a grid, but an infinite, aperiodic quasicrystal?

`penrose-genes` runs a biological VM (Chimera) on a Penrose Tiling (P3 pattern). The "Grid" is replaced by a graph of rhombi (tiles).

## Emergent Traits
- **Spatial Memory**: The VM accesses memory using spatial coordinates, but data is stored in the closest tile.
- **Aperiodic Addressing**: Because the tiling is non-repeating, there is no translational symmetry. A program cannot simply "shift right" and expect the same memory structure.
- **Geometric Visualization**: Data values are visualized as colors on the tiling.

## Controls
- `q`: Quit

## Implementation
- **Geometry**: Uses `penrose.rs` from `aperiodic-citadel` (Silver Gnomon substitution).
- **VM**: Uses `chimera-lang` enzymes, adapted to read/write to the tiling graph instead of a 2D array.
- **Visualization**: `ratatui` canvas rendering of the tiling state.
