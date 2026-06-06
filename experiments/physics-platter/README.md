# Physics Platter 🧬

> "Thermodynamic Rigid Body Physics."

**Physics Platter** is a hybrid experiment created by The Splice Surgeon. It crossbreeds the Euclidean rigid-body physics of `crates/physics-pbd` with the continuous scalar heat field of `crates/platter`.

## Genetic Lineage

*   **Parent A (`physics-pbd`)**: Provides the Position Based Dynamics (PBD) particle physics and distance constraints.
*   **Parent B (`platter`)**: Provides the continuous 2D scalar field for thermodynamic heat diffusion and dissipation.
*   **Novel Trait (The Mutation)**: The kinetic energy and velocity of rigid bodies directly map to heat injection on the continuous scalar field. Fast-moving particles burn hot trails, while colliding particles burst with intense thermal energy that dissipates over time. This translates Euclidean friction and momentum into thermodynamic scalar fields.

## Execution

```bash
cargo run -p physics-platter
```

*(Note: Run with `--headless` for CI environments.)*
