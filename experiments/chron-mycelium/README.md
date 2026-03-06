# chron-mycelium

**Lineage:** `experiments/chrontext` × `experiments/myco-transit`

This experiment crosses the Git blame chronological age parsing of `chrontext` with the Physarum polycephalum (slime mold) pathfinding logic of `myco-transit`.

## Concept
"Codebase Foraging". The biological pheromone system treats the Git commit history as an organic resource grid. The git blame age score becomes a food source/pheromone trail for slime mold agents. Old code and new code become discrete nodes in a scavenging network, where agents leave glowing architectural pathways between the modified locations of a file.

## Emergence
The slime mold network reveals the hidden structural paths of code activity across long files. Hot spots from recent commits attract agents, pulling trails of structural layout from older code strata, creating a physical manifestation of code dependency over time.

## Execution
Run `cargo run -p chron-mycelium -- <file_path>`