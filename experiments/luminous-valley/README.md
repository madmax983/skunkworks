# Luminous Valley

**Lineage:** `valley-forge` (Terrain/Reaction-Diffusion) × `luminous-flock` (Boids/Firefly Sync)

## Concept

Boids flocking over a procedural 3D terrain where a Gray-Scott reaction-diffusion simulation is running. The boids act as a mobile sensor network, reacting to the chemical concentrations on the ground.

## Traits

-   **Bio-luminescent Terrain Interaction:** Boids read the chemical state of the terrain below them. High concentration of "Chemical B" (Green/Growth) causes boids to:
    -   Flash more frequently (increased metabolic rate).
    -   Shift color towards Cyan.
-   **Height-Adaptive Flight:** Boids follow the contours of the terrain, maintaining a set altitude.
-   **Synchronized Flashing:** Inherited from `luminous-flock`, boids try to synchronize their flashes using an Integrate-and-Fire model.

## Controls

-   **WASD / Arrow Keys:** Move Camera
-   **Space:** Make it rain (add chemical)
