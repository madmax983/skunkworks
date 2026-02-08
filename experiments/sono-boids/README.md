# Sono-Boids 🦇

> "In the dark, we see with sound."

A hybrid experiment combining **Luminous Flock** (Boids) and **Sono-Scapes** (Wave Physics).

## 🧬 Lineage

*   **Parent A**: `luminous-flock` (TUI Boids logic)
*   **Parent B**: `sono-scapes` (Wave Simulation via `resonance-audio`)
*   **Novel Trait**: **Echolocation Navigation**. Boids emit acoustic pulses (ripples) into the medium and navigate based on wave interference patterns.

## 🔬 Experiment

The terminal becomes a wave tank. Boids (represented by `*`) flock together while emitting random pings. These pings create ripples in the underlying physics grid.

*   **Red**: Wave Peak
*   **Blue**: Wave Trough
*   **Yellow**: Boid

Boids steer away from high-intensity wave areas to avoid acoustic overcrowding, creating complex feedback loops between the agents and their environment.

## 🎮 Controls

*   **Space**: Trigger a massive splash at the center.
*   **Q**: Quit.

## 🧠 Technical Details

*   Uses `ratatui` for TUI rendering.
*   Uses `resonance-audio` for FDTD (Finite Difference Time Domain) wave simulation.
*   Uses `tui-shared::math::Vec2` for vector physics.
*   Runs a headless audio simulation to drive the visuals.
