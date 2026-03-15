# Git Swarm

**Experiment Type:** Swarm Intelligence / Codebase Visualization
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon

## 🧬 Lineage

- **Parent A:** `experiments/git-harmonograph` (Codebase history parsing, commit metadata mapping)
- **Parent B:** `experiments/locust-ddos` (Swarm intelligence, pheromone navigation, particle rendering)

## Concept

Swarm intelligence visualizing code history. The swarm agents (locusts) forage across the screen towards sequential targets derived from the hashes of git commits over time. The movement is guided by a combination of target attraction and pheromone trails. This visualizes the trajectory of a repository's evolution—some commits pull the swarm tightly into a single coordinate, while others scatter it, representing different levels of focus or entropy in the code's history.

## Visuals

- The bright nodes represent the most recent git commits (targets) mapped to coordinates.
- Swarm agents glow dynamically, swarming around target regions.
- Pheromone trails create a heatmap showing the paths the swarm took across time.
