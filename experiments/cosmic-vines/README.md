# Cosmic Vines 🌌🌿

A simulation of biological growth structures adapting to the vibrations of a 3D cosmic string.

## 🧬 Lineage
- **Parent A**: `experiments/cosmic-strings` (Symplectic String Physics, 3D Projection Camera)
- **Parent B**: `experiments/venation-vines` (Space Colonization Growth Algorithm)

## 🔬 Concept
This experiment visualizes "String Theory Botany".
- A **Cosmic String** vibrates in 3D space, acting as a high-energy source.
- **Vines** (based on leaf venation logic) grow towards the string's nodes.
- As the string vibrates, the attractors move, causing the vines to twist and spiral as they chase the energy source.

## 🎮 Controls
- **Arrows**: Rotate Camera (Azimuth / Elevation)
- **Space**: Pluck the string (creates vibration)
- **+/-**: Increase/Decrease String Tension
- **R**: Reset Simulation
- **Q / Esc**: Quit

## 🏗️ Implementation Details
- **Physics**: Mass-spring system with semi-implicit Euler integration (`physics.rs`).
- **Growth**: 3D Space Colonization Algorithm (`growth.rs`). Attractors are tethered to string nodes.
- **Rendering**: Custom 3D-to-2D projection on `ratatui` Canvas (`render.rs`).

## 🧪 Observation
The vines exhibit emergent helical structures as they attempt to colonize the moving string. High-frequency vibrations create "bushy" growth, while low-frequency waves create long, sweeping tendrils.
