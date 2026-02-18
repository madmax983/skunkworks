# Code Catchment

**"The Valley of Refactoring"**

A Moonshot experiment combining **Hydraulic Erosion** and **Codebase Evolution**.

## Concept
The repository is visualized as a geological terrain.
- **Initial State**: A heightmap where each file is a point on a grid. Height is proportional to the file size (Lines of Code). Large files are mountains.
- **Erosion**: "Rain" falls on files that are modified in commits.
- **Simulation**: Water flows downhill, carrying sediment (code complexity/refactoring potential) and depositing it in valleys.
- **Result**: Highly active files erode into canyons. Stable files remain as ancient plateaus.

## Tech Stack
- **Rust**
- **Bevy** 0.14 (PBR, ECS)
- **Git2** for repository analysis
- **Hydraulic Erosion Simulation** (Particle-based)

## Controls
- **WASD / Q / E**: Move Camera
- **Space**: Look at center / Reset Camera view
- **Rain**: Automatically triggered by simulating commits (5 commits/tick).

## Running
```bash
cargo run -p code-catchment
```
(Requires a Git repository in the current directory or `.` passed as argument in code)
