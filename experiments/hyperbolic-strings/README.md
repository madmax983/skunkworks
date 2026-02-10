# Hyperbolic Strings 🎻🍩

**"A string theory for a universe with negative curvature."**

This experiment simulates a vibrating string in the **Poincaré Disk** model of hyperbolic geometry.

## 🧬 Lineage

- **Parent A:** `experiments/cosmic-strings` (Physics of vibrating strings)
- **Parent B:** `experiments/poincare-crawl` (Hyperbolic geometry and visualization)

## 🌟 Concept

What if the fundamental strings of the universe existed in a hyperbolic manifold?
In this simulation:
- **Strings are Geodesics:** The string rests along a geodesic arc.
- **Hyperbolic Tension:** The "springs" connecting the nodes obey hyperbolic metric rules. The force is proportional to the hyperbolic distance between nodes.
- **Möbius Dynamics:** Position updates use Möbius addition to strictly adhere to the geometry of the disk.

## 🎮 Controls

- **WASD / Arrows:** Move the camera (Möbius translation). The world warps around you as you move.
- **Space:** Pluck the string (applies a random force vector in the local tangent space).
- **R:** Reset the simulation.
- **Q / Esc:** Quit.

## 🧮 Physics

The simulation uses a semi-implicit Euler integrator adapted for non-Euclidean space:
1. **Force Calculation:** For each node $i$, we transform its neighbors $j$ to $i$'s local tangent space (origin) using `mobius_sub(P_j, P_i)`.
2. **Hooke's Law:** We calculate the force vector based on the hyperbolic distance $d = 2\text{atanh}(|z|)$.
3. **Integration:** Velocity is updated in the tangent space.
4. **Transport:** The new position is calculated by applying the velocity vector as a Möbius translation to the current position: $P_{new} = \text{mobius\_add}(V \cdot dt, P_{old})$.
