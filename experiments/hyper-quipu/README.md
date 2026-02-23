# Hyper-Quipu 🧬🕸️🖥️

**A 4D System Monitor woven into Knots.**

> "The threads of the machine are not flat. They weave through time." - The Splice Surgeon

## 🧬 Lineage
- **Parent A**: `experiments/tesseract-ops` (The Hypercube Monitor)
  - *Inheritance*: 4D Vector Math (`Vec4`), System Monitoring (`SystemMonitor`), Rotation Logic.
- **Parent B**: `experiments/quipu-symphony` (The Incan Sequencer)
  - *Inheritance*: The concept of cords recording history, knots representing values.
- **Hybrid Trait**: **4D Knots**.
  - Traditional Quipus are 1D cords in 3D space.
  - Hyper-Quipu cords hang in 4-Dimensional space ($x, y, z, w$).
  - The $w$-axis is Time. The structure rotates in 4D to reveal hidden correlations between cores.

## 🕹️ Controls
- **Arrow Keys**: Rotate the 4D Viewport (XW / YW planes).
- **W / S**: Zoom In / Out.
- **Space**: Pause / Resume.
- **Q**: Quit.

## 🎵 How it Works
- **Cords**: Each CPU core is a cord.
- **Knots**: Points added at the current time ($w$).
- **Position**:
  - $x, y$: Base position in a circle + Load distortion.
  - $z$: Load distortion depth.
  - $w$: Time.
- **Visualization**:
  - The entire structure is projected from 4D -> 3D -> 2D using stereographic projection.
  - As time passes, new knots appear at the "front" ($w=0$ relative to camera) and flow "back" into history.

## 🏗️ Architecture
Uses `ratatui` for TUI rendering and `hyper-system` for 4D math. It demonstrates how high-dimensional data can be visualized in a terminal environment.
