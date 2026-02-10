# Hyperbolic Finder ⚛️🗺️

> "Circles grow exponentially. So do your files."

A file system explorer mapped onto the Poincaré Disk model of hyperbolic geometry, featuring **Memory Visualization**.

## Concept
In hyperbolic space, area grows exponentially with radius. This allows us to visualize large hierarchies (like file systems) by placing deeper nodes "closer" to the boundary of the disk.

This project combines:
1.  **Hyperbolic Geometry**: Infinite space in a finite disk.
2.  **Memory Visualization**: Angular sectors and node sizes are proportional to the contained file size (recursive). This creates a "Hyperbolic Sunburst" effect where large folders dominate the view.

## Controls
- **Left Click**: Navigate to a folder/file (Center it).
- **Drag**: Pan the view (Möbius transformation).
- **Backspace**: Return to Root (Center at 0,0).

## Tech Stack
- **Rust**
- **Macroquad** (Rendering)
- **Poincaré Disk** (Math)

## Math & Features
- **Proportional Layout**: Siblings are allocated angular space proportional to `sqrt(total_size)`. Large folders get more room.
- **Size Scaling**: Node radius scales logarithmically with file size. Large files appear massive.
- **Dynamic Hit Testing**: Interaction areas scale with node size, making it easier to select large items.
