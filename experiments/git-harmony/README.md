# Git Harmony ⚛️🎵

> "Listen to the song of your repository."

**Git Harmony** is a moonshot experiment that translates git history into a generative ambient soundscape using a Finite Difference Time Domain (FDTD) wave physics simulation.

## Concept

Every commit is a pebble dropped into a pool of water.
- **File Path**: Determines the (x, y) coordinate on the simulation grid.
- **Insertions/Deletions**: Determine the strength and polarity of the ripple.
- **Churn**: Determines the turbulence.

The simulation runs a 100x100 wave equation grid (`resonance-audio`) and visualizes the pressure waves in real-time using a TUI (`ratatui`).

## Tech Stack

- **Audio Physics**: `resonance-audio` (FDTD Wave Equation)
- **Audio Output**: `cpal` (Optional, falls back to silent simulation)
- **Visualization**: `ratatui` + `crossterm`
- **Git Parsing**: `git2` / `git-associates`

## Usage

```bash
cargo run -p git-harmony
```

### Controls

- **Q**: Quit
- **P**: Manually pluck the center (for testing)
- **Space**: Pause/Resume history playback

## Meaning

This tool explores "Codebase Sonification". By listening to the rhythm and intensity of commits, we might perceive patterns in development velocity, refactoring waves, and architectural tremors that are invisible in text logs.
