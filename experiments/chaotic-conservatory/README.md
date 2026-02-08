# Chaotic Conservatory 🌿🌀

**Lineage:** `recursive-conservatory` × `system-attractor`

A procedural vegetation generator where L-System growth is influenced by the vector field of a Lorenz Attractor. Instead of growing in straight lines, the branches curve and twist, following the chaotic streamlines of the attractor.

## Concept
The "Turtle" that interprets the L-System string is not moving in a vacuum. It is swimming in a chaotic fluid (the Lorenz system).
- **F (Forward):** Integrate position along the Lorenz vector field.
- **+ / - (Turn):** Rotate the orientation relative to the flow.
- **Parameters:** The L-System rules determine the topology (branching), while the Lorenz parameters (`rho`, `sigma`, `beta`) determine the geometry (shape).

## Controls
- **Arrow Up/Down:** Increase/Decrease L-System iterations (Generation).
- **Arrow Left/Right:** Decrease/Increase Chaos (`rho` parameter).
- **WASD:** Rotate Camera.
- **Q/E:** Zoom In/Out.

## Observations
- At low chaos influence, the tree looks like a normal plant.
- As chaos increases, the branches start to swirl into the famous "butterfly" shape.
- High iteration counts create dense, chaotic foliage that visualizes the attractor's structure using biological rules.

## The Splice Surgeon's Notes
"I have infected the garden with chaos. The plants no longer seek the sun; they seek the strange attractor. The result is a hybrid that is neither plant nor equation, but a living visualization of mathematical turbulence."
