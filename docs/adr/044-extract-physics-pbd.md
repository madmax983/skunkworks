# 044. Extract Physics PBD

* **Status:** Accepted
* **Context:**
  Several experiments (`neuro-fold`, `neuro-cipher`, `origami-swarm`) require physical simulation of interconnected particles, such as for simulating folding structures or force-directed graph layouts. Implementing physics engines individually in each experiment leads to significant code duplication, potential errors, and inconsistencies in simulation quality and stability. Position Based Dynamics (PBD) is a robust and stable method well-suited for these interactive simulations.
* **Decision:**
  We decided to create a shared physics library, `crates/physics-pbd`, implementing a Position Based Dynamics engine.
  This crate encapsulates:
  - **`PbdSystem`**: The core solver managing particles and constraints.
  - **`Particle`**: Represents a point mass with position, velocity, and inverse mass.
  - **`Constraint`**: Defines rules limiting particle movement, including:
    - **`Distance`**: Keeps two particles at a fixed distance.
    - **`Actuator`**: Changes distance dynamically (e.g., for muscles).
    - **`Pin`**: Fixes a particle to a specific world position.
  - **`Solver`**: Iteratively resolves constraints to simulate physical behavior.

  The crate utilizes `macroquad`'s `Vec3` for vector mathematics to ensure seamless integration with the repository's primary rendering engine.

* **Consequences:**
  - **Positive:** Provides a single, optimized, and stable physics backend for all experiments requiring particle simulation. Bug fixes and performance improvements (e.g., SIMD optimizations) are applied globally.
  - **Negative:** Adds a dependency on `macroquad` (for `glam` vector types), which might be a constraint if the library needs to be used in a headless environment without `macroquad`.
