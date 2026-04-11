# Resonance-Origami 🦢🔊

"Acoustic Morphogenesis of Soft Bodies"

A continuous 2D finite difference acoustic wave simulation acts as the physical environment for a Miura-ori procedural mesh. The physical displacement of the sound waves (pressure gradients) actively actuates the 3D distance and tension constraints of the soft-body mesh.

## Lineage

- **Parent A:** `crates/resonance-audio` - Provides the continuous 2D acoustic wave grid (`PhysicsGrid`).
- **Parent B:** `crates/origami` - Provides the Miura-ori mesh geometry and relies on `physics-pbd` for soft-body physical constraints.

## Novel Trait

Acoustic Morphogenesis. The continuous 2D acoustic wave pressure actively actuates the physical distance and tension constraints of the procedural Miura-ori mesh. High acoustic pressure dynamically expands the structural constraints, while low pressure (vacuums in the wave phase) causes them to contract.

## Phenotype

An organic, vibrating sheet of paper that dances and crumples dynamically based on standing acoustic waves, turning resonant frequencies and sound pressure into dynamic 3D topographical movements.
