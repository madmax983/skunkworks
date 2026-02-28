# Git Mycelium 🍄💻

**A Hybrid Experiment by The Splice Surgeon 🧬**

> "The codebase is a substrate. Commits are the nutrients. Let the mycelium find the optimal architecture."

## 🔬 Experiment Analysis

This experiment combines Git metadata parsing with slime mold optimization algorithms to visualize the hidden collaborative highways in a codebase.

- **Parent A**: `experiments/git-harmonograph` (Git Metadata Parsing)
- **Parent B**: `experiments/myco-transit` (Slime Mold Pathfinding & TUI)

### Concept
Instead of slime molds connecting arbitrary cities, the "Cities" are the most frequently modified files (or top authors) in a git repository. The "Commuters" (agents) are spawned based on commit frequency, traveling between files that were changed together in the same commit.

### Lineage & Genetics
- **Allele A (Data Source)**: Parsing Git commit history to extract file co-occurrence and author metrics (`git-harmonograph`).
- **Allele B (Simulation)**: `Physarum polycephalum` agent-based simulation with pheromone trails and gradient descent (`myco-transit`).
- **Emergent Trait (Phenotype)**: A glowing, fungal network that dynamically reveals the "true" architecture of the codebase based on developer activity, bypassing static analysis. Strong trails indicate highly coupled components.

## 🕹️ Controls

- **Q**: Quit

## 📊 Technical Details

- **Simulation**: Parallelized agent updates using `rayon`.
- **Rendering**: `ratatui` Canvas.
- **Data**: Reads local `git log` output.
