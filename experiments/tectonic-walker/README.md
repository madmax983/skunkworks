# Tectonic Walker 🧬

**Lineage:** `tectonic-flock` × `ik-codewalker`

A procedural creature that walks along the timeline of this repository's Git history. The terrain is generated from commit data, where the X-axis represents time and the Y-axis represents "Stress Level" (calculated from keywords like `TODO`, `FIXME`, `unwrap`, `panic`).

## Concept
The "Walker" is an Inverse Kinematics (IK) creature that navigates the "Tectonic" landscape of the codebase. High-stress commits create jagged, steep terrain, while stable commits create flat plains.

## Parents
- **[tectonic-flock](../tectonic-flock)**: Provided the `GitScanner` logic and the concept of mapping git history to geological features.
- **[ik-codewalker](../ik-codewalker)**: Provided the Bevy engine scaffolding, the IK system, and the procedural animation logic.

## Phenotype
- **Visuals**: A vector-graphics path representing the commit history (Strata).
- **Agent**: A multi-segmented creature that physically reaches for the next commit in the sequence.
- **Behavior**: The walker autonomously traverses the history of the repository.

## Usage
Run with:
```bash
cargo run -p tectonic-walker
```
Ensure you have `git` installed and are inside a git repository.
