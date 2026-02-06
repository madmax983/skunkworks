# Git Landscape 🏔️✈️

**Lineage:** `git-cantata` × `log-landscape`

A TUI flight simulator that traverses the history of a git repository as a 3D terrain.

## 🧬 Genetic traits

- **From `git-cantata`**: Git history parsing, audio engine (sonification of commits), file extension coloring.
- **From `log-landscape`**: Isometric projection logic, buffer-based terrain generation.
- **Novel Trait**: **Temporal Flight**. Instead of static logs or 2D painting, the codebase history becomes a physical landscape you fly over. The "Horizon" is the future (newest commits), and you fly towards it (or away from the past?).
    - *Correction*: In this simulation, the "Horizon" represents the newest commits entering the buffer, so you are flying *forward* through time as new commits appear in the distance and move towards you.

## 🎮 Controls

- **Q / Esc**: Quit
- **Space**: Pause / Resume flight
- **Up Arrow**: Increase flight speed
- **Down Arrow**: Decrease flight speed

## 🏗️ Build & Run

```bash
# Basic visual mode
cargo run -p git-landscape -- <path-to-repo>

# With audio enabled (requires alsa/rodio deps)
cargo run -p git-landscape --features audio -- <path-to-repo>
```

## 🧪 Observations

The terrain height is determined by the magnitude of changes (insertions + deletions) in a file. The X-axis represents file buckets (hashed by path).
High peaks indicate massive refactors or large commits.
Color indicates the dominant file extension in that bucket.
