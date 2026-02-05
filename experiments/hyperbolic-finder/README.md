# Hyperbolic Finder ⚛️🗺️

> "Circles grow exponentially."

A file system explorer mapped onto the Poincaré Disk model of hyperbolic geometry.

## Concept
In hyperbolic space, area grows exponentially with radius. This allows us to visualize large hierarchies (like file systems) by placing deeper nodes "closer" to the boundary of the disk, where they occupy exponentially less visual space until focused.

When you click a node, the entire space is transformed (Möbius transform) to bring that node to the center.

## Controls
- **Left Click**: Navigate to a folder/file (Center it).
- **Backspace**: Return to Root (Center at 0,0).

## Tech Stack
- **Rust**
- **Macroquad** (Rendering)
- **Num-Complex** (Complex number math for transformations)

## Math
The visualization uses the **Poincaré Disk Model**.
- **Points**: Complex numbers $z$ where $|z| < 1$.
- **Distance**: $d(a,b) = 2 \tanh^{-1} | \frac{a-b}{1-\bar{a}b} |$
- **Movement**: Isometries are Möbius transformations of the form $f(z) = \frac{z-a}{1-\bar{a}z}$.
