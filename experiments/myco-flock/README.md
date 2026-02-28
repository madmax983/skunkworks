# myco-flock

**Parent A**: `experiments/myco-transit`
**Parent B**: `experiments/luminous-flock`

## Concept
Pheromone-Guided Flocking. Boids leave a pheromone trail on a grid (like `myco-transit` slime mold agents). At the same time, boids sense the pheromone grid and are attracted to higher concentrations, creating a feedback loop where paths emerge and flocks self-organize into stable "highways" rather than aimlessly wandering.

## Novel Trait
Pheromone-Guided Flocking. The boids exhibit behavior resembling foraging ants or slime molds, organizing into stable structural paths rather than purely fluid swarms.

## Lineage
- **From myco-transit**: Pheromone grid simulation (`trails` and `next_trails`), diffusion and decay mechanics using `rayon`.
- **From luminous-flock**: Boid genetics (`Dna`), boid physics (`position`, `velocity`, `acceleration`), `locus::flocking` forces (Separation, Alignment, Cohesion), TUI render loop logic.
- **Novel Mutation**: Boid `Dna` now includes `pheromone_attraction_weight` and `pheromone_deposit_amount`. Boids use simple directional sensors to find higher pheromone concentrations and steer towards them.
