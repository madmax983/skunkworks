# Cosmic Harmonograph 🌌🎻

A hybrid experiment splicing `cosmic-strings` (Physics) with `git-harmonograph` (Visualization).

## Concept

This experiment visualizes the vibrational modes of the codebase history.
Two "Cosmic Strings" (simulated vibrating strings) act as the pendulums of a Harmonograph.
Instead of swinging back and forth, they vibrate according to wave physics.

Git commits are the "plucks" that excite the strings.
- **Commit Hash**: Determines the random seed for the pluck force and position.
- **Author Name**: Determines the tension of String X.
- **Commit Date**: (Implicitly the order of plucks).

The resulting drawing is a trace of the combined displacement of the two strings over time, creating complex Lissajous-like figures that decay as the strings settle.

## Lineage

- **Parent A**: `experiments/cosmic-strings` (Physics of 1D wave equation).
- **Parent B**: `experiments/git-harmonograph` (Parametric drawing driven by Git).
- **Novel Trait**: "Oscilloscope for History". The drawing is not a static plot but a time-integrated simulation of physical vibration.

## Controls

- `[Space]`: Pluck the next commit in history.
- `[P]`: Toggle Auto-Play (automatically plucks commits).
- `[R]`: Reset the strings to rest.
- `[C]`: Clear the drawing trace.
- `[Q]`: Quit.

## Implementation

- **Physics**: 1D Wave Equation (Semi-Implicit Euler integration).
- **Visualization**: `ratatui` Canvas.
- **Input**: `git2` for repository traversal.
