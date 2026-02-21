# Hyper-Mold 🧬🍄🧊

> "The maze is the machine itself."

**Hyper-Mold** is a hybrid experiment combining the biological logic of `myco-diffusion` (Physarum polycephalum / Slime Mold) with the 4D visualization and system monitoring of `tesseract-ops`.

It simulates a colony of 5000+ slime mold agents navigating a 4-dimensional hypercube. The geometry of this space is not static; it breathes and distorts based on real-time system metrics (CPU, Memory, Swap, Load Average).

## Lineage

- **Parent A**: `experiments/myco-diffusion` (Slime Mold Logic, Reaction-Diffusion)
- **Parent B**: `experiments/tesseract-ops` (4D Hypercube, System Metrics)
- **Novel Trait**: **4D Bio-Computation**. The agents evolve pathfinding strategies in a 4D environment that changes shape based on the host computer's workload.

## Usage

Run with:
```bash
cargo run -p hyper-mold
```

## Controls

- **WASD**: Move Camera (Dolly/Strafe)
- **Arrow Keys**: Rotate Camera
- **System Load**: Distorts the Hypercube (X=CPU, Y=RAM, Z=Swap, W=Load) and drives the 4D rotation speed.

## How it Works

1.  **4D Environment**: A `Grid4D` structure stores pheromone concentrations in a 4-dimensional array (`width * height * depth * hypersize`).
2.  **Agents**: Each agent has a 4D position and velocity. They sense pheromones in their forward 4D cone and randomly perturb their direction to find higher concentrations (Chemotaxis).
3.  **System Distortion**: The rendering engine projects 4D coordinates to 3D space. The projection matrix is scaled by system metrics:
    - **CPU Usage** stretches the X-axis.
    - **Memory Usage** stretches the Y-axis.
    - **Swap Usage** stretches the Z-axis.
    - **Load Average** stretches the W-axis and controls rotation speed.
4.  **Parallel Processing**: Agent updates and grid diffusion are parallelized using `rayon` to handle the computational load of 4D simulation.

## Hypothesis

As the system comes under load, the "distance" between nodes in the hypercube changes. The slime mold should adapt its network structure to these new topological constraints, potentially optimizing its pathfinding for the current system state.
