import re

with open("MUTATIONS.md", "r") as f:
    text = f.read()

attempted = """## 🌿 Attempted Crosses

### myco-resonance
- **Parents**: experiments/myco-transit + crates/resonance-audio
- **Concept**: Pheromone-Guided Acoustic Wave Advection.
- **Novel trait**: Slime mold agents distribute the active source on a 2D acoustic wave tank grid. The biological paths physically disrupt and reflect acoustic waves. As agents deposit pheromones to form organic highways, these concentrated highways pluck the continuous audio simulation engine, creating organic acoustic rhythms shaped entirely by biological slime mold growth patterns.
- **Predicted Phenotype**: Acoustic Pheromone Interference. As biological agents deposit pheromones and establish organic network highways, these trails act as acoustic exciters in the continuous physical space, bridging biological network pathfinding with physical wave propagation.
- **Status**: experiments/myco-resonance
- **Evaluation**: Success. Compiled. The biological paths successfully disrupt and pluck the continuous audio engine.
"""

text = text.replace("## 🌿 Attempted Crosses", attempted)

with open("MUTATIONS.md", "w") as f:
    f.write(text)
