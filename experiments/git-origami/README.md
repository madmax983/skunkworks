# 🧬 git-origami

**Structural Codebase Deformation**

This experiment is a hybrid cross between:
- **`git-associates`**: The parent crate providing parsing and mapping of the codebase's git history.
- **`origami`**: The parent crate providing a procedural 3D Miura-ori soft-body mesh generation.

## Novel Trait & Lineage

This experiment demonstrates **Structural Codebase Deformation**.

The discrete history of the repository (from `git-associates`) is projected onto a continuous 3D soft-body mesh (`origami` simulated via `physics-pbd`). As the repository undergoes changes, the physical topological space dynamically buckles, crumples, and breathes.

Git insertions act as positive forces extending the mesh, while deletions act as negative forces collapsing it. The resulting emergent phenotype provides a breathing, continuous representation of abstract codebase history in 3D.

## Usage

```bash
cargo run -p git-origami -- [path_to_repo]
```
