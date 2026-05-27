import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

# Update Proposed Crosses if empty, otherwise remove myco-locus from Proposed Crosses (though it was an invented cross)

new_entry = """### myco-locus
- **Parents**: crates/myco-transit + crates/locus
- **Concept**: Topological Mycelial Transit.
- **Novel trait**: Slime mold agents (Physarum polycephalum) forage and deposit pheromones on a continuous 2D grid governed by non-Euclidean topological boundaries (Torus, Klein Bottle, Sphere, Projective).
- **Predicted Phenotype**: An organic network of fungal highways demonstrating shortest-path routing over continuous non-Euclidean boundary layers, producing seamless wrap-around structural clusters.
- **Status**: experiments/myco-locus
- **Evaluation**: Success. Compiled. Pheromone routes successfully wrap seamlessly across topological boundaries.

"""

# Insert under "## 🌿 Attempted Crosses"
content = content.replace("## 🌿 Attempted Crosses\n", "## 🌿 Attempted Crosses\n\n" + new_entry)

# Update Phase 1 Evaluation (Latest Run)
phase_1_update = """**Phase 1 Evaluation (Current Run) 🧬:** I have evaluated all previous hybrids. They continue to compile successfully and exhibit highly viable emergent phenotypes. I am advancing the autonomous cross of `myco-transit` with `locus` (`myco-locus`) to map biological pathfinding directly onto topological boundaries, extending the Swarm mechanics of the `locus-flock` success to pheromone decay grids.\n\n"""

content = content.replace("## 🌸 Proposed Crosses\n", phase_1_update + "## 🌸 Proposed Crosses\n")


with open("MUTATIONS.md", "w") as f:
    f.write(content)
