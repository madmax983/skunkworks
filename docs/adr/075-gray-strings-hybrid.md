# 075. Gray-Strings Hybrid (Acoustic Morphogenesis)

Date: 2026-03-26

## Status
Accepted

## Context
The repository continuously explores the intersection of different simulational models to discover emergent "phenotypes." We sought to combine the continuous chemical reaction-diffusion dynamics of `crates/gray-scott` with the discrete acoustic physics and Karplus-Strong synthesis of `experiments/ferrous-strings`. The goal was to establish a bidirectional feedback loop where the continuous substrate affects the discrete agents, and the discrete agents, in turn, act as localized catalysts on the continuous substrate.

## Decision
We created the `gray-strings` hybrid experiment.
The system features `GrayString` entities embedded within a `GrayScott` grid.
1. **Grid to String (Sensing):** Each string reads the local chemical concentrations (U and V) at its midpoint to dynamically adjust its physical tension (spring constant), which directly alters its acoustic frequency and vibrational amplitude.
2. **String to Grid (Actuation):** As the string vibrates, its physical displacement acts as a catalyst, injecting the kill chemical ('V') directly into the morphogenetic grid along the length of the string, proportional to the displacement at each point.

## Consequences

### Positive
*   **Emergent Phenotype:** Successfully demonstrates "Acoustic Morphogenesis", a novel bidirectional feedback loop between continuous and discrete systems.
*   **Audio-Visual Correlation:** The visual state of the chemical grid is directly audible through the tension and resulting frequencies of the strings.

### Negative
*   **Computational Cost:** Updating the continuous reaction-diffusion grid alongside the Karplus-Strong audio synthesis and string physics per frame introduces performance bottlenecks, especially at larger grid sizes.
*   **Complexity:** The dual physics update loops (`update_physics` and `perturb_grid`) require careful balancing of constants (tension, damping, chemical addition rates) to prevent the simulation from exploding or decaying instantly.