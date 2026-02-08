# 🧬 Ghost Knots

**Lineage:** `git-ghost` × `quipu-serializer`

> "The dead are not gone, they are just knotted into the fabric of the repository."

## Concept

This experiment visualizes deleted files ("ghosts") from the git history as Incan Quipu knots.
It combines the necromantic scanning of `git-ghost` with the physical data serialization of `quipu-serializer`.

## Usage

Run the experiment in any git repository:

```bash
cargo run -p ghost-knots
```

## Features

- **Necromancy**: Scans the git graveyard for deleted files.
- **Serialization**: Encodes metadata (path, deletion date, author) into Quipu cords using ASCII values.
- **Decay**: The visual representation of the knots decays over time (dimming, fraying) based on how long ago the file was deleted.
- **Navigation**:
  - `Left/Right`: Cycle through different deleted files (Ghosts).
  - `Up/Down`: Scroll the Quipu view.
  - `Q`: Quit.

## Genetic Traits

- **From `git-ghost`**: The `Ghost` struct and `scan_graveyard` logic, plus the `decay` algorithm.
- **From `quipu-serializer`**: The `Quipu`, `Cord`, and `Knot` structures and the `serde` implementation for converting data to knots.
- **Emergent**: The "Hall of Memories" where lost code hangs as physical artifacts in the terminal.

## Status

Freshly spawned. The knots are tight, but the oldest ones are already beginning to fray.
