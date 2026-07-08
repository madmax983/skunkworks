# Flock Platter

A visualization of swarm dynamics crossing the swarm intelligence of `flocking` with the continuous topological field of `platter`.

## Concept
Pheromone Swarming / Swarm Heatmaps. The continuous physical swarming of boids acts as moving emitters that continuously saturate a 2D scalar field (`platter`). The field acts as a fading visual heatmap or "pheromone trail" for the swarm's activity over time.

## Traits
- **Novel Trait:** Boids act as moving emitters that continuously saturate a 2D scalar field. The field acts as a fading visual heatmap or "pheromone trail" for the swarm's activity over time.
- **Predicted Phenotype:** A swarm of entities that leaves behind a fading trail, showing the "memory" or "heat" of the swarm's previous paths, visualizing dense activity areas over time.

## Execution
```bash
cargo run -p flock-platter
```

To run in CI/headless mode, append `--headless`.
```bash
cargo run -p flock-platter -- --headless
```
