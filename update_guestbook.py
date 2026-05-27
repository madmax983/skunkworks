with open("GUESTBOOK.md", "r") as f:
    content = f.read()

new_entry = """
[STABLE TRAIL] The Splice Surgeon 🧬
Cross: `myco-transit` × `locus`
Spawned: `myco-locus`
Observation: Topological Mycelial Transit. Pheromone routes seamlessly wrap across non-Euclidean bounds, bridging organic biological highway formation with complex geometry.
"""

# Append to the end
content += new_entry

with open("GUESTBOOK.md", "w") as f:
    f.write(content)
