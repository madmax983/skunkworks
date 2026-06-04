# 📄💥 physics-origami

**Soft-Body Collision Dynamics**

This experiment is a hybrid crossing `physics-pbd` and `origami`. It demonstrates the physical interaction between Euclidean rigid-body physics particles and a procedural Miura-ori soft-body mesh.

As the physical particles collide and bounce across the grid, their kinetic energy directly strikes and dynamically actuates the topological Z-depth tension constraints of the `origami` sheet, physically crumpling the soft-body structure upon impact.

## Usage

```bash
cargo run -p physics-origami
```

## Lineage
- **Parent A (`physics-pbd`)**: Rigid body particle physics, velocity, and gravity.
- **Parent B (`origami`)**: Procedural soft-body mesh generation.
- **Novelty**: Combining discrete colliding Euclidean particles with a continuous deformable soft-body topography.
