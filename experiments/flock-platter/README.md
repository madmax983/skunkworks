# `flock-platter` 🧬

**Lineage**: `crates/flocking` × `crates/platter`

## Novel Trait

A swarm of boids navigates a 2D space, depositing heat/pheromones into a continuous scalar field (`platter`).

## Phenotype Prediction

A glowing, fading network of trails where the flock traverses, showing the density and path history of the swarm over time.

## Execution

Run the simulation:
```bash
cargo run -p flock-platter
```

To run in CI/headless mode without freezing the terminal:
```bash
cargo run -p flock-platter -- --headless
```