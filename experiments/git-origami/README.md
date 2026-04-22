# Git Origami 🦢

A hybrid experiment visualizing Git history through procedural Miura-ori mesh deformation.

## Lineage

**Parent A: `crates/git-associates`**
- Provides the `GitModel` for fetching repository history and computing file modifications (insertions/deletions).
- The git commit timeline acts as the active force mapping to physical interaction.

**Parent B: `crates/origami`**
- Provides the 3D procedural soft body structure (`generate_miura_grid`).
- The geometric constraints form the physics environment that gets crumpled by the history.

## Phenotype

The discrete timeline of repository commits is mapped onto a continuous 2D plane underlying a 3D soft-body mesh. When a commit occurs, it acts as a physical "tug" on the paper constraints corresponding to the file's hash, causing the codebase's history to literally crumple and fold the architecture over time. High-churn files become physical stress points on the fabric.
