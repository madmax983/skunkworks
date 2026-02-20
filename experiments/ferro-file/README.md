# Ferro-File 🧲📁

> "Data has mass. Data has spin. The filesystem is a magnetic domain."

**Ferro-File** is a hybrid experiment combining `ising-tide` (GPU Ising Model) and `miller-fs` (Crystallographic Filesystem Visualization). It visualizes your filesystem as a 3D magnetic lattice where files are dipoles.

## 🧬 Lineage

*   **Parent A**: `ising-tide` (WGPU Compute Shader Ising Model)
*   **Parent B**: `miller-fs` (Crystallographic Filesystem Scanning)
*   **Genetic Trait**: Files act as magnetic spins in a 3D grid. The "void" between files is non-magnetic vacuum.

## 🧪 Experiment

The simulation maps your current directory structure onto a 3D grid.
*   **Files**: Represented as magnetic dipoles (Spins).
*   **Spin**: Up (Blue) or Down (Red).
*   **Dynamics**: The spins interact via the Ising Model (Ferromagnetic). Files want to align with their neighbors.
*   **Temperature**: System activity (simulated) affects the disorder. High temperature causes random flipping (paramagnetism). Low temperature causes ordering (ferromagnetism).

## 🎮 Controls

*   **Arrows**: Move camera (orbit).
*   **Scroll**: Zoom.
*   **Up/Down**: Increase/Decrease Temperature.
*   **Left/Right**: Increase/Decrease External Field.
*   **Space**: Toggle Brush (Magnetize locally).
*   **1/2/3**: Change Lattice Topology (SC, BCC, FCC) - *Note: This changes the neighbor graph, creating "wormholes" in the filesystem topology.*

## 📸 Visualization

The filesystem is sparse, but the Ising grid is dense. Files are mapped to grid coordinates modulo 64. This creates a "folded" space where distant files might interact if they hash to adjacent cells.

## 🏗️ Build

```bash
cargo run -p ferro-file
```
