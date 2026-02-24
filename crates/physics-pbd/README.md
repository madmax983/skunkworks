# Physics PBD ⚛️

A robust, Position Based Dynamics (PBD) physics engine for 2D/3D simulations.

**Position Based Dynamics** is a method where constraints are solved by directly modifying the positions of particles, rather than applying forces (like in traditional Impulse-Based dynamics). This makes the simulation unconditionally stable, even with large time steps or stiff constraints.

## Features

- **Unconditional Stability**: Constraints are resolved geometrically, preventing "explosions" common in force-based engines.
- **Constraints**:
  - `Distance`: Rigid links (rods) or springs.
  - `Actuator`: Muscle-like constraints that expand/contract.
  - `Pin`: Hard anchors to world space.
- **Integration**: Uses Verlet integration for symplectic stability.
- **Safety**: Built-in guards against `NaN` propagation and zero-mass singularities.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
physics-pbd = { path = "../physics-pbd" }
```

## Usage

### The Hero's Journey: Simulating a Pendulum

Here is how to set up a simple pendulum.

```rust
use physics_pbd::{PbdSystem, Constraint};
use macroquad::prelude::Vec3;

fn main() {
    let mut system = PbdSystem::new();

    // 1. Create the Anchor (Static)
    // Mass = 0.0 means infinite mass (immovable)
    let anchor = system.add_particle(Vec3::new(0.0, 10.0, 0.0), 0.0);

    // 2. Create the Bob (Dynamic)
    let bob = system.add_particle(Vec3::new(2.0, 8.0, 0.0), 1.0);

    // 3. Connect them
    // This creates a rigid rod of length ~2.82 (calculated automatically)
    system.add_distance_constraint(anchor, bob, 1.0);

    // 4. Simulate
    // Step forward 16ms, running the constraint solver 5 times per frame.
    // More iterations = stiffer constraints.
    system.step(0.016, 5);
}
```

## Performance Notes

- **Iterations**: The stiffness of constraints depends on the iteration count. With 1 iteration, a "rigid" rod will act like a soft spring. With 10+ iterations, it becomes very stiff.
- **Block-Scoped Access**: The solver uses unsafe-free block-scoped re-borrowing to optimize memory access patterns, making it cache-friendly.
- **NaN Safety**: The engine actively checks for `NaN` masses or positions to prevent "infection" of the entire simulation state.

## License

MIT / Apache-2.0
