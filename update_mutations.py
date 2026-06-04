import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

# Append to Attempted Crosses
new_cross = """
### physics-origami
- **Parents**: crates/physics-pbd + crates/origami
- **Concept**: Soft-Body Collision Dynamics.
- **Novel trait**: The kinetic energy of Euclidean rigid-body physics particles directly strikes and dynamically actuates the topological Z-depth tension constraints of a procedural Miura-ori soft-body mesh, physically crumpling the soft-body structure upon impact.
- **Predicted Phenotype**: A visualization where the physical particles deform the continuous deformable soft-body topography upon collision.
- **Status**: experiments/physics-origami
- **Evaluation**: Success. Compiled. The soft-body mesh is physically deformed by colliding particles acting as strikers.
"""

content = content.replace("## 🔮 Proposed Crosses", new_cross + "\n## 🔮 Proposed Crosses")

with open("MUTATIONS.md", "w") as f:
    f.write(content)
