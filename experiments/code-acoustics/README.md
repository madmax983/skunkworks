# Code Acoustics 🔊🧬

> "Hearing the shape of the code."

A hybrid experiment combining **Codebase Mapping** with **Acoustic Simulation**.

## 🧬 Lineage
- **Parent A**: `experiments/cymatic-mesh` (FDTD Waveguide Mesh Physics)
- **Parent B**: `experiments/code-canyon` (File System Scanning & Mapping)

## 🧪 Concept
The repository file structure is mapped to a 2D acoustic grid.
- **Directories** form the walls of the chambers.
- **Files** are the open air within.
- **File Size** determines the acoustic impedance (Large files = Dense/Slow medium).
- **Code Structure** becomes the resonant cavity.

## 🕹️ Controls
- **L-Click**: Pluck the code string at cursor.
- **R-Drag**: Draw custom walls.
- **R**: Reset waves.

## 📦 Build
```bash
# Standard build (Audio disabled by default for compatibility)
cargo run -p code-acoustics

# With Audio enabled (Requires ALSA dev headers on Linux)
cargo run -p code-acoustics --features audio
```
