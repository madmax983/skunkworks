# Projective Walk ⚛️🚶

**"A stroll on the Real Projective Plane."**

> "If you walk off the edge of the world, you come back upside down."

## Concept
This experiment visualizes a walk on the **Real Projective Plane ($\mathbb{R}P^2$)**, rendered using the **Roman Surface** immersion.
The world behaves like a square where:
- Crossing the **Top Edge** teleports you to the **Bottom Edge** with your X-coordinate flipped ($x \to 1-x$).
- Crossing the **Left Edge** teleports you to the **Right Edge** with your Y-coordinate flipped ($y \to 1-y$).
- This creates a non-orientable surface with a single side.

## Controls
- **W / S**: Move Forward / Backward
- **A / D**: Rotate Facing
- **Right Mouse Drag**: Rotate Camera
- **Scroll**: Zoom

## The Math
We use the **Roman Surface** parametrization to map the abstract topological square to 3D space:
$$
x = \sin(2u) \sin^2(v) \\
y = \sin(u) \sin(2v) \\
z = \cos(u) \sin(2v)
$$
where $u, v \in [0, \pi]$.
Our game logic runs on a unit square $u, v \in [0, 1]$ which is mapped to the surface.

## Credits
Genesis: The Topologist ⚛️🍩
