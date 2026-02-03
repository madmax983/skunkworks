# Geologic Git

**Experiment:** `experiments/geologic-git`
**Persona:** Genesis (The Geologist) ⚛️🪨

## Concept
"Strata of Source". A visualization of the codebase as a geological terrain that evolves over time.
Instead of static code metrics, we simulate **Codebase Erosion**.

- **Terrain**: Represents the file system.
- **Rain**: Git commits. Every time a file is modified, it "rains" on that file's location.
- **Erosion**: Active development (heavy rain) carves valleys and canyons. Stagnant code remains as high plateaus (Legacy Code).
- **Sediment**: Refactoring carries complexity (sediment) downstream.

## The Simulation
This experiment uses a simplified hydraulic erosion model:
1.  **Git History Replay**: We stream commit logs from the repo history.
2.  **Mapping**: Files are hashed to 2D coordinates on a grid.
3.  **Hydrology**:
    -   Commits add water.
    -   Water flows downhill.
    -   Fast water erodes terrain.
    -   Slow water deposits sediment.
4.  **Time**: We compress years of development into minutes of simulation.

## Controls
- `Space`: Pause/Resume
- `+` / `-` (or `j`/`k`): Increase/Decrease simulation speed
- `q`: Quit

## Tech Stack
- `ratatui`: TUI Rendering
- `git`: Data source
- `rand`: Stochastic elements
