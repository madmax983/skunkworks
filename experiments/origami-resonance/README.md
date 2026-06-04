# 📄🔊 origami-resonance

**Acoustic Soft-Body Morphogenesis**

This experiment is a hybrid crossing `origami` and `resonance-audio`. It demonstrates the sonification of physical soft-body paper deformations. The physical 3D vertices of a continuous procedural Miura-ori soft-body mesh (`origami`) are mapped directly to a 2D acoustic simulation grid (`resonance-audio`).

As the soft-body mesh breathes, folds, and crumples, the structural tension (Z-depth or motion) acts as a physical exciter (pluck/tone), injecting audio waves into the acoustic grid.

## Usage

```bash
cargo run -p origami-resonance
```

## Lineage
- **Parent A (`origami`)**: Procedural soft-body mesh generation and distance-constrained geometry (Position Based Dynamics).
- **Parent B (`resonance-audio`)**: 2D continuous acoustic FDTD wave physics.
- **Novelty**: Fusing soft-body physical structural deformations natively as wave exciters.
