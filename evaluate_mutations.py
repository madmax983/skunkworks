import re

with open('MUTATIONS.md', 'r') as f:
    content = f.read()

# Replace the specific Proposed Crosses entry
proposed_pattern = r'### origami-flock.*?\*\*Predicted Phenotype\*\*:[^\n]+\n'
content = re.sub(proposed_pattern, '', content, flags=re.DOTALL)

# Add to Attempted Crosses
attempted_cross = """
### origami-flock
- **Parents**: crates/origami + crates/flocking
- **Concept**: Swarm-Driven Soft Body Morphogenesis.
- **Novel trait**: The continuous, physical soft-body paper mesh of `origami` is directly deformed by the swarm intelligence of `flocking`. Boids navigate the surface of the mesh, and their collective movements and density physically pull and crumple the fabric of the space they inhabit.
- **Predicted Phenotype**: An organic, living topography that bucks and folds under the weight of the flock, demonstrating how biological swarming intent can warp its own physical environment.
- **Status**: experiments/origami-flock
- **Evaluation**: Success. Compiled.
"""
attempted_idx = content.find('## 🌿 Attempted Crosses')
content = content[:attempted_idx + len('## 🌿 Attempted Crosses\n')] + attempted_cross + content[attempted_idx + len('## 🌿 Attempted Crosses\n'):]

with open('MUTATIONS.md', 'w') as f:
    f.write(content)
