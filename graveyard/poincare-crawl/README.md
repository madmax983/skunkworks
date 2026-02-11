# Poincaré Crawl

A rogue-like explorer set in the **Poincaré Disk model** of the hyperbolic plane.
This experiment visualizes a {5, 4} hyperbolic tiling (pentagons, 4 meeting at each vertex) using ASCII/Unicode characters in the terminal.

## The Impossible Space
In hyperbolic geometry, space expands exponentially. Walking in a "square" (turning 90 degrees left 4 times) does not bring you back to the start. The "straight lines" are circular arcs orthogonal to the boundary circle.

## Controls
*   **WASD / Arrows**: Move (Translation via Möbius transformation).
*   **E / R**: Rotate view.
*   **Q / Esc**: Quit.

## Math
*   **Poincaré Disk**: Points are complex numbers $z$ with $|z| < 1$.
*   **Isometries**: Movement is simulated by applying Möbius transformations of the form:
    $$f(z) = \frac{az + b}{\bar{b}z + \bar{a}}$$
    where $|a|^2 - |b|^2 = 1$.
*   **Rendering**: Geodesics (hyperbolic straight lines) are rendered as Euclidean circle arcs. The tiling is generated recursively and transformed into the player's view frame before drawing.

## Credits
Genesis: The Topologist ⚛️🍩
