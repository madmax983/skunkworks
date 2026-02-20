# Lithosphere Git 🌋

> "Code is not static. It is a living landscape, uplifted by commits and weathered by time." - Genesis: The Geologist

**Lithosphere Git** is a "Moonshot" experiment that visualizes a git repository as a geological terrain. It combines **Hydraulic Erosion Simulation** with **Version Control History**.

## 🌍 The Concept

*   **Terrain Generation**: The file system is mapped to a 2D grid. Initial terrain is generated with noise.
*   **Tectonics (Uplift)**: Git insertions raise the land at specific coordinates corresponding to files.
*   **Weathering (Erosion)**: Git modifications and deletions trigger "rain" events.
*   **Hydraulic Erosion**: Water flows downhill, carrying sediment. Areas of high churn (frequent changes) become eroded valleys or sediment-filled basins. Stable code remains as high, jagged peaks.

## 🎮 Controls

*   **Arrow Keys**: Move camera (X/Z plane).
*   **PageUp / PageDown**: Move camera Up/Down (Y axis).
*   **Space**: Pause/Resume simulation.
*   **+ / -**: Increase/Decrease simulation speed.

## 🏗️ Running

```bash
cargo run -p lithosphere-git
```

By default, it tries to visualize the repository it is running in (the root of the monorepo). If it fails to find `.git`, it runs in demo mode with random events.

## 🧬 Lineage

*   **Genesis**: Created by the "Mad Scientist" agent specializing in Moonshots.
*   **Inspiration**: `tectonic-git` (strata), `chimera-erosion` (fluids), `git-landscape` (flight).
*   **Tech**: Rust, Macroquad, Git2.

## 🔬 Observation

This tool reveals "Hotspots" in a codebase not just as red lines in a diff, but as physically eroded canyons. A deep canyon implies a file that has seen a lot of flow (change) over time. A tall mountain implies a file that grew large and stayed stable.
