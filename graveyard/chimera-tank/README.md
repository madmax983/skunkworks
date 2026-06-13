# Chimera-Tank 🧬🌊

**Cross:** `chimera-lang` × `ripple-tank`

## Concept
Acoustic Genetic Organisms navigating a 2D acoustic wave tank.

## Lineage
- **Parent A**: `chimera-lang` (The Brain)
  - Provides the `ChimeraVM` genetic programming engine.
  - Agents evolve `Dna` sequences to sense and act. Note: Requires `features = ["nova"]` and initialization via `Dna::from_genes(genes)`.
- **Parent B**: `ripple-tank` (The Physics)
  - Provides the 2D acoustic wave simulation and macroquad rendering.
  - Agents navigate through physical waves.

## Novel Trait: Acoustic Locomotion
Agents must evolve to pluck waves to propel themselves, translating genetic instructions into physical acoustic locomotion. They sense pressure gradients and output physical acoustic forces.

## Status
- Compiles: ✅
- Phenotype: Oscillatory cymatic swimming.
