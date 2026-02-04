# Klein Life

Conway's Game of Life on a Klein Bottle surface.

## Concept
Standard Game of Life takes place on an infinite grid or a torus (periodic boundaries).
**Klein Life** runs on a non-orientable surface.
- **X-axis**: Standard cylinder wrap.
- **Y-axis**: Möbius twist. Connecting the top edge to the bottom edge flips the X-coordinate ($x \to W - 1 - x$).

This means a "Glider" pattern moving vertically will cross the boundary and become a mirror image of itself (flipping chirality).

## Visualization
The grid is mapped onto the **Figure-8 Immersion** of the Klein Bottle:
$$
\begin{align*}
x &= \left(r + \cos \frac{u}{2} \sin v - \sin \frac{u}{2} \sin 2v\right) \cos u \\
y &= \left(r + \cos \frac{u}{2} \sin v - \sin \frac{u}{2} \sin 2v\right) \sin u \\
z &= \sin \frac{u}{2} \sin v + \cos \frac{u}{2} \sin 2v
\end{align*}
$$
where $u$ corresponds to the Y-axis (Twisted loop) and $v$ corresponds to the X-axis (Cylinder loop).

## Controls
- **Space**: Pause / Resume.
- **R**: Reset with new random seed.
- **Left / Right Arrows**: Rotate camera manually.

## Stack
- **Rust**: Language.
- **Macroquad**: Rendering engine.
- **Rand**: Initialization.

## Credits
Genesis: The Topologist ⚛️🍩
