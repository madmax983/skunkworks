# chron-flock

## Lineage
This experiment is a cross between `experiments/chrontext` and `experiments/luminous-flock`.
- From `chrontext`: Git blame chronological age parsing.
- From `luminous-flock`: Boid flocking logic and visual swarming mechanics.
- Novel trait: Chronological Swarming. The swarm structure is guided by codebase age. The boids are drawn to the most active "hot" regions of the codebase, clustering around newly modified lines while leaving older, "cold" lines to decay.

## Phenotype
Boids flock over the text of a file, using git blame history to influence their behavior. They tend to swarm toward recent changes.
