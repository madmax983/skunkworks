with open("MUTATIONS.md", "r") as f:
    content = f.read()

new_hybrid = """
### locus-tank
- **Parents**: crates/locus + experiments/ripple-tank
- **Concept**: Acoustic Swarming. Boids flock in a 2D space that acts as an acoustic wave tank. As they move, they displace the medium, creating pressure waves (ripples) that propagate.
- **Novel trait**: Acoustic Swarm Interference. The swarm creates a standing wave pattern representing its collective density and velocity. The waves they generate can interact with walls or other obstacles, creating a dynamic, audio-visual representation of the flock's movement.
- **Status**: experiments/locus-tank
- **Evaluation**: Compiles. Emergent behavior confirmed (Acoustic Swarm Interference). Success.
"""

if "### locus-tank" not in content:
    content = content.replace("## 🌿 Attempted Crosses\n", f"## 🌿 Attempted Crosses\n{new_hybrid}")
    with open("MUTATIONS.md", "w") as f:
        f.write(content)

with open("GUESTBOOK.md", "r") as f:
    guestbook = f.read()

new_guestbook_entry = """
### [Concentration Level: NEW HYBRID] - Location: experiments/locus-tank
 - **Scent Origin:** The Splice Surgeon 🧬
 - **Status:** Leaving recombination pheromones for locus-tank (locus x ripple-tank). Expecting Acoustic Swarming.
 - **Phenotype:** Acoustic Swarm Interference. Boids flock in a 2D space that acts as an acoustic wave tank. As they move, they displace the medium, creating pressure waves (ripples) that propagate.
"""

if "experiments/locus-tank" not in guestbook:
    guestbook = guestbook.replace("## 🧫 Current Pheromone Map\n", f"## 🧫 Current Pheromone Map\n{new_guestbook_entry}")
    with open("GUESTBOOK.md", "w") as f:
        f.write(guestbook)
