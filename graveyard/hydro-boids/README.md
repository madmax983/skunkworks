# Hydro-Boids 🧬🐟

**Hybrid Lineage**: `luminous-flock` × `primordial-soup`

## 🧬 Concept
A simulation where **Boids** (bird-oid objects) swim in a **Smoothed-Particle Hydrodynamics (SPH)** fluid.
The boids are treated as particles within the fluid system, experiencing pressure and viscosity, but they also possess "will"—steering forces for separation, alignment, and cohesion.

## 🧪 Traits
-   **From luminous-flock**: Flocking behavior (Separation, Alignment, Cohesion).
-   **From primordial-soup**: SPH fluid dynamics (Density, Pressure, Viscosity).
-   **Emergent Behavior**: Boids create turbulence as they swim. Fluid currents push the boids. The flock creates a "wake" in the fluid density field.

## 🎮 Controls
-   `q`: Quit
-   Boids are Yellow characters (`>`, `v`, `<`, `^`).
-   Fluid density is visualized as Blue/Cyan patterns (`#`, `=`, `-`, `.`).

## 🏗️ Implementation
-   **Physics**: A unified `FluidSolver` that handles both passive fluid particles and active boid particles.
-   **Rendering**: `ratatui` TUI.

## 🔮 Predictions
The flock should be harder to form than in a vacuum because viscosity drags the boids, and pressure prevents them from overlapping too closely (stronger separation).
