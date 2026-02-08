# Code Canyon ⚛️🪨

> "Every valley is a river's signature. Every mountain is a tectonic argument." — Genesis (The Geologist)

**Code Canyon** is a geological simulation of your codebase. It maps your file system to a Hilbert Curve terrain, where file size determines elevation. It then replays your `git` history as a hydraulic erosion simulation.

*   **Mountains**: Large files.
*   **Valleys**: Eroded by the flow of changes.
*   **Rain**: Commits.
*   **Erosion**: Code churn.

## 🎮 Controls

| Key | Action |
| :--- | :--- |
| **`H`** | **Replay History** (The main event) |
| **`Space`** | Toggle random rain (Standard erosion) |
| **`R`** | Reset terrain & history |
| **`W`** / **`S`** | Zoom In/Out |
| **`Arrows`** | Orbit Camera |
| **`Esc`** | Exit |

## 🧪 The Science

1.  **Topography**: Files are sorted alphabetically and mapped to a 2D grid using a **Hilbert Curve**. This ensures that files in the same directory tend to be spatially local, creating "continents" of modules.
2.  **Uplift**: Initial height is derived from `ln(file_size)`.
3.  **Hydraulic Erosion**:
    *   Rain drops fall on modified files during history replay.
    *   Water flows downhill, carrying sediment.
    *   Velocity increases with slope.
    *   Sediment is deposited in depressions or when water slows down.
4.  ** Thermodynamics**:
    *   Active files flash **RED** (Hot).
    *   Heat decays over time (Cooling), returning to the biome color.

## 📦 Usage

```bash
cargo run --release -- [path/to/repo]
```

Defaults to the current directory if no path is provided.
