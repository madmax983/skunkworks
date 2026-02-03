# Git Cellular Automata (git-ca) 🦠

> "Every commit has a life of its own."

**git-ca** is a generative art experiment that visualizes the "vitality" of your code changes. It takes the diff hash of the current commit (or any commit) and uses it to seed a Conway's Game of Life simulation.

## Concept

- **Seed**: The hash of the `git diff` output becomes the DNA.
- **Grid**: A 64x64 toroidal field.
- **Evolution**: Standard B3/S23 rules.

## Why?

To answer the questions:
- Is this refactor "stable"? (Dies out quickly into blocks/beehives)
- Is this feature "chaotic"? (Explodes into gliders)
- What is the "texture" of my work today?

## Usage

```bash
cargo run -p git-ca
```

Controls:
- `Space`: Pause/Resume
- `R`: Reset to initial seed
- `Q`: Quit
