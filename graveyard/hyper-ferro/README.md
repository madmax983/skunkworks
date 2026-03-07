# Hyper Ferro 🧲🧊

**Parents**: `experiments/tesseract-ops` × `experiments/ferro-file`

A hybrid experiment visualizing a **4D Ising Model** (Hyper-Magnetism) where the physical constants of the simulation are driven by the host system's performance metrics.

## Concept

- **4D Lattice**: The simulation runs on a 4x4x4x4 hypercubic lattice (256 nodes).
- **Hyper-Magnetism**: Each node has a spin (Up/Down). The system evolves via the Metropolis algorithm to minimize energy.
- **System Entanglement**:
    - **Temperature (Chaos)**: Driven by **CPU Usage**. High CPU load increases the temperature, causing spins to become disordered (paramagnetic phase). Low CPU load allows them to crystallize (ferromagnetic phase).
    - **Lattice Scale (Geometry)**: Driven by **Memory Usage**. The 4th dimension expands/contracts based on RAM.
    - **Rotation (Time)**: Driven by **Swap/Load**. The tesseract rotates faster when the system is under stress.

## Controls

- `Arrow Keys`: Rotate Camera
- `+/-`: Manually offset Temperature
- `W/S`: Zoom In/Out

## Lineage

- **From `tesseract-ops`**: The 4D `Vec4` mathematics, projection logic, and `SystemMonitor` integration.
- **From `ferro-file`**: The concept of representing data/system state as a magnetic spin system (Ising Model).
- **Novel Trait**: **Hyper-Magnetic Phase Transitions**. Visualizing how the "heat" of the computer (CPU) physically melts the magnetic order in 4-dimensional space.
