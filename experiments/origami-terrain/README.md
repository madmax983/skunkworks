# Origami Terrain 🦢🏔️

A hybrid experiment combining **deployable origami structures** (Miura-ori) with **reaction-diffusion simulations** (Gray-Scott).

The landscape is a folded sheet of paper. The surface texture is alive.
As you fold the paper, the biological reaction changes.

## 🧬 Lineage

- **Parent A**: `experiments/origami-history` (Deployable Miura-ori folds)
- **Parent B**: `experiments/terra-phage` (Reaction-Diffusion Terrain)

## 🔬 Concept: Mechanical Chemotaxis

This experiment explores the interaction between **geometry** and **biology**.
- The world is a **Miura-ori tessellation**, a famous origami fold used for deploying solar panels in space.
- The surface of the fold hosts a **Gray-Scott reaction-diffusion system**, simulating chemical or biological patterns (coral, spots, stripes).
- **Novel Trait**: The folding state (`extension`) modulates the chemical parameters (`feed` rate).
  - When the paper is **flat** (Extension 1.0), the reaction is stable.
  - When the paper is **compressed** (Extension < 1.0), the "pressure" increases the feed rate, altering the pattern formation.

## 🎮 Controls

- **Mouse Right Click + Drag**: Rotate Camera
- **Mouse Scroll**: Zoom
- **UP / DOWN Arrows**: Fold / Unfold the terrain (Change Extension)
- **SPACE**: Make it Rain (Add chemicals)
- **R**: Reset simulation

## 🏗️ Technical Details

- **Engine**: `macroquad`
- **Physics/Math**: `crates/origami` (Miura-ori math), `crates/gray-scott` (Parallel RD solver)
- **Rendering**: Procedural Mesh generation. `u` concentration maps to displacement (Height), `v` concentration maps to color.

## 🧪 Observations

- Compressing the fold creates "valleys" where the reaction seems to intensify due to the parameter modulation.
- The pattern flows over the sharp creases of the origami without breaking, creating a continuous "folded skin".
