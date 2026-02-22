# 049. Hyper Acoustics Architecture

## Status
Accepted

## Context
The **Hyper Series** of experiments explores high-dimensional simulations modulated by system metrics. **Hyper Acoustics** aims to simulate sound propagation in a 4-dimensional space, where the speed of sound and damping are influenced by the host machine's CPU load and memory usage.

Standard 3D audio engines are insufficient for visualizing and sonifying 4D hyperspace. We need a specialized solver that can handle the extra spatial dimension while maintaining real-time performance for audio generation.

## Decision
We implement a **4D Finite Difference Time Domain (FDTD)** wave equation solver.

### 1. 4D Grid Structure
The simulation runs on a flattened 4D grid `PhysicsGrid4D` of size $6^4$ (1296 cells).
*   **Dimensions:** $x, y, z, w$.
*   **State:** Each cell stores pressure $u$, previous pressure $u_{prev}$, and next pressure $u_{next}$.
*   **Solver:** We use a 9-point Laplacian stencil (2 neighbors per dimension) to compute wave propagation.
    $$ \nabla^2 u = \sum_{d \in \{x,y,z,w\}} (u_{d+1} + u_{d-1} - 2u) $$
    $$ u_{next} = 2u - u_{prev} + c^2 \nabla^2 u $$

### 2. Headless Audio Backend
To support environments without audio hardware (CI/CD, servers), we abstract the audio backend:
*   **Feature Flag:** `audio` (default).
*   **Cpal Integration:** When enabled, `cpal` drives the simulation loop via the audio callback thread.
*   **Headless Fallback:** When disabled, a dedicated `std::thread` mimics the audio callback timing to ensure the simulation progresses deterministically.

### 3. System Modulation
The simulation parameters $c^2$ (wave speed) and damping are modulated by:
*   **CPU Load:** Increases wave speed (Energy injection).
*   **RAM Usage:** Increases damping (Viscosity).

## Consequences

### Positive
*   **Novelty:** Enables the exploration of non-euclidean acoustics.
*   **Portability:** The headless mode ensures the experiment can run and be tested in any environment.
*   **Performance:** The small grid size ($6^4$) allows for high-frequency updates (44.1kHz) on standard CPUs.

### Negative
*   **Complexity:** Visualizing 4D fields is inherently difficult; we currently rely on 3D slices or projections.
*   **Memory Bandwidth:** The flattened array access pattern may cause cache thrashing if not optimized, though the small total size mitigates this.
