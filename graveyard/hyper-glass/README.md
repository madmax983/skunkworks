# Hyper Glass

**A 4D Spin Glass Simulation**

> "The system heat melts the magnetic order."

## Concept

This experiment simulates the **4D XY Model** (or Spin Glass) where spins live on the vertices of a 4-dimensional hypercube grid. The simulation visualizes the phase transition between ordered (Ferromagnetic) and disordered (Paramagnetic) states, driven by the computer's real-time CPU temperature.

## Lineage

*   **Parent A**: `experiments/broken-mirror` (XY Model Physics, Symmetry Breaking)
*   **Parent B**: `experiments/tesseract-ops` (4D Hypercube Geometry, System Metrics)
*   **Novel Trait**: **4D Phase Transitions**. The critical temperature is modulated by the host machine's workload.

## Features

*   **4D Spin Grid**: A grid of spins ($\theta \in [0, 2\pi)$) in 4 dimensions.
*   **Metropolis-Hastings Dynamics**:
    *   Spins update based on energy minimization (alignment with neighbors).
    *   **Temperature ($T$)**: Driven by **CPU Usage**. High load = High Entropy (Melting).
    *   **External Field ($B$)**: Driven by **RAM Usage**. High Memory = Strong Field (Forced Alignment).
*   **Visualization**:
    *   4D spins projected to 3D.
    *   **Color**: Spin angle mapped to Hue.
    *   **Geometry**: The 4D Tesseract rotates and breathes based on system load.

## Controls

*   **Arrow Keys**: Rotate Camera around the 3D projection.
*   **W/S**: Zoom In/Out.
*   **Physics**:
    *   **CPU Load**: Increases Temperature (Disorder).
    *   **RAM Usage**: Increases Magnetic Field (Order).

## Building

*   `cargo run -p hyper-glass`
