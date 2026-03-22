# Chimera Acoustics 🧬🔊

> "In the beginning was the Word, and the Word was a waveform." - Genesis (The Audiophile)

**Chimera Acoustics** is a hybrid experiment combining the 4D FDTD Wave Equation Solver from `hyper-acoustics` with the biological agents of `chimera-lang`.

It simulates a population of "Acoustic Grazers" living inside a 4D Hypercube where sound propagates physically. The agents must "sing" (emit pressure) to create waves and "eat" (dampen pressure) to survive.

## 🧬 Lineage

This experiment is a direct cross between:

*   **Parent A**: `experiments/hyper-acoustics` (The Environment)
    *   Provides the `AcousticGrid4D` and the FDTD Wave Equation solver.
    *   Inherits the system-load-driven distortion (CPU speed = Wave speed).
*   **Parent B**: `experiments/chimera-lang` (The Life)
    *   Provides the `ChimeraVM` and genetic code execution.
    *   Agents run a genetic program that decides when to Sing (`Pluck`), Eat (`Dampen`), or Move.

## ⚗️ Novel Traits

*   **Bio-Acoustic Ecology**: The soundscape is not just ambient; it is the food source. Agents reduce the volume of the world by eating it. If they eat too much, the world goes silent and they starve. They must learn to sing to sustain the ecosystem.
*   **4D Sonic Grazing**: Agents navigate a 4D space, listening to pressure gradients and moving towards/away from sound sources.
*   **The Hive Mix**: The audio output is a direct sonification of the pressure at the center of the room, which is the sum of all agent singing and damping activities.

## 🎮 Controls

*   **WASD / Arrow Keys**: Rotate and Zoom the 4D Camera.
*   **Left Click**: Pluck the center of the grid manually (Feed the agents).
*   **System Load**:
    *   **CPU Usage**: Increases Wave Speed ($c^2$).
    *   **RAM Usage**: Increases Base Damping (Viscosity).

## 🧪 Implementation Details

*   **Physics**: 4D Finite Difference Time Domain (FDTD) Wave Equation.
*   **Biology**: ChimeraVM agents with custom I/O channels for Plucking and Damping.
*   **Audio**: Real-time synthesis via `cpal`, driven by the physics simulation running in the main thread.
