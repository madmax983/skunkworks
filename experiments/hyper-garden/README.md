# Hyper Garden 🌿

**A 4D Procedural Garden grown from Git History.**

## Concept
This experiment visualizes the codebase's history as a garden of plants growing in a 4-dimensional hypercube.

- **DNA**: Each plant corresponds to a Git commit. The commit hash is parsed as a Sexagesimal (Base-60) number, which generates the L-System rules for that plant.
- **Environment**: The plants grow in 4D space (X, Y, Z, W). The W-axis represents "Hyper-Growth".
- **Weather**: The "wind" and "sun" of this world are the system metrics (CPU, Memory, Swap).
  - **CPU Load**: Distorts the X-axis and accelerates rotation.
  - **Memory Usage**: Distorts the Y-axis.
  - **Swap Usage**: Distorts the Z-axis.
  - **Overall Load**: Causes the W-axis to "breathe", pulsating the plants in and out of the 4th dimension.

## Lineage
This is a hybrid created by **The Splice Surgeon**.

- **Parent A**: `experiments/tesseract-ops` (The Hypercube Engine & System Monitor)
- **Parent B**: `experiments/babylonian-garden` (The Git-to-Plant L-System Logic)

## Controls
- **Arrow Keys**: Rotate the camera around the 3D projection.
- **W/S**: Zoom in/out.

## Emergent Behavior
Watch how the garden "wilts" or "stretches" when you compile code or run heavy processes. The plants react to the stress of the machine that simulates them.
