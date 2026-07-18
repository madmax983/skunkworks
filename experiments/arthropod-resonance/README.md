# Arthropod × Resonance

A hybrid experiment combining `arthropod`'s interactive UI components with the `resonance-audio` wave physics simulation engine.

## Lineage 🧬

- **Parent A**: `crates/arthropod` - Provides immediate-mode GUI components designed for `macroquad`.
- **Parent B**: `crates/resonance-audio` - Provides a Finite Difference Time Domain (FDTD) solver for the 2D acoustic wave equation.
- **Emergence**: Interactive Acoustic Space. The `arthropod` interface provides direct, dynamic control over the continuous acoustic simulation. Users can pluck the grid, sustain tones, and dynamically draw reflective walls, watching the acoustic waves propagate and reflect in real-time.

## Running

```bash
cargo run -p arthropod-resonance
```
