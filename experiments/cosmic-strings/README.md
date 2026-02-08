# Cosmic Strings 🎻

A simulation of 1D oscillating strings in 3D space, inspired by String Theory and musical physics.

## 🌟 Concept

This experiment fulfills the request from **Genesis (The Astronomer)** to visualize string theory vibration modes.
It simulates a mass-spring system in 3D space, allowing the string to vibrate in multiple dimensions simultaneously.

## 🎮 Controls

- **Arrows**: Rotate Camera (Azimuth / Elevation).
- **Space**: Pluck a random point on the string.
- **1-9**: Pluck specific harmonic modes (1=Fundamental, 2=First Overtone, etc.).
- **+ / -**: Increase / Decrease Tension.
- **R**: Reset Simulation.
- **Q / Esc**: Quit.

## 🔊 Audio

If the `audio` feature is enabled (requires ALSA/CPAL development libraries), the string "hums" based on its physical properties:
- **Pitch**: Calculated from $f = \frac{1}{2L} \sqrt{\frac{T}{\mu}}$.
- **Volume**: Proportional to the total Kinetic Energy of the system.

## 🛠️ Usage

To run without audio (default):
```bash
cargo run -p cosmic-strings
```

To run with audio (requires `libasound2-dev` on Linux):
```bash
cargo run -p cosmic-strings --features audio
```

## 🧮 Physics Model

- **Integration**: Semi-Implicit Euler (Symplectic).
- **Forces**: Hooke's Law (Springs) + Linear Damping.
- **Dimensions**: 3D (Nodes have `Vec3` position/velocity).
