# Origami History 📜🦢

**Lineage:** `experiments/origami-satellite` × `experiments/git-cantata`

A visualization of git history as a deployable Miura-ori fold structure. Each panel represents a commit. The structure unfolds to reveal the timeline.

## Concept
- **Structure:** A Miura-ori tessellation where the grid size is determined by the number of commits (sqrt(N) x sqrt(N)).
- **Data:** Git history (hash, author, timestamp) fetched via `git-associates`.
- **Rendering:** Flat-shaded panels color-coded by author hash.
- **Interaction:** Unfold the history physically using the mouse.

## Controls
- **Left Click + Drag:** Rotate Camera
- **Right Click + Drag (or Shift + Left Click):** Deploy/Retract the history (Time Travel)
- **Scroll:** Zoom

## Technical Details
- Uses `macroquad` for 3D rendering.
- Uses `git-associates` to parse the `.git` directory.
- Implements "Exploded Mesh" rendering to achieve flat shading per commit panel.
- Falls back to dummy data if no git repository is found.

## Emergent Traits
- **Physical History:** The timeline has physical properties (stiffness, extension limit).
- **Spatial Clustering:** Commits are arranged in a 2D grid, revealing patterns in commit frequency/authorship spatially rather than just linearly.
