# Bifurcation Crawler

**Parents**: `bifurcation-landscape` × `chimera-tissue`

A soft-body organism simulation where agents ("Crawlers") navigate the **Bifurcation Diagram of the Logistic Map**.

The terrain is mathematical. The "safe zones" are the attractor points of the Logistic Map ($x_{n+1} = r \cdot x_n \cdot (1 - x_n)$). As the parameter `r` increases (moving right), the system bifurcates from stability into chaos.

## Mechanics

- **Physics**: Soft-body simulation using `physics-pbd`. The Crawler is a segmented worm with muscle actuators.
- **Environment**: The background is the bifurcation diagram.
  - **Darkness**: The void. Agents lose energy here.
  - **Light/Color**: The attractor. Agents gain energy here.
- **Biology**: Each Crawler is controlled by a `ChimeraVM` executing genetic code (DNA).
  - **Inputs**: Chaos Level (Distance from attractor), Internal Energy.
  - **Outputs**: Muscle Contraction.
- **Objective**: Traverse the landscape from Order (Left) to Chaos (Right) without starving.

## Lineage

- **From `bifurcation-landscape`**: The mathematical environment (Logistic Map).
- **From `chimera-tissue`**: The soft-body physics engine and genetic control system.
- **Novel Trait**: **Chaotic Selection Pressure**. Survival depends on adapting to an environment that becomes mathematically more complex (fractal) as you progress.
