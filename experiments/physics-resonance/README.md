# Acoustic Rigid Body Physics (`physics-resonance`)

## Lineage
**Parents:**
- `physics-pbd`: Position Based Dynamics physics engine, solving rigid constraints and momentum in Euclidean space.
- `resonance-audio`: A 2D Finite Difference Time Domain (FDTD) wave grid for modeling acoustic pressure and standing wave propagation.

## Phenotype
An acoustic generator where physical rigid body constraints and collisions drive localized acoustic exciters. As objects fall, swing, and collide in the Euclidean physics space, they structurally hit the continuous FDTD acoustic wave grid, transferring kinetic momentum into standing waves and sound pressure. The result is sonification of rigid body physics in real time.

## Architecture
- The Position Based Dynamics solver updates constraints every tick.
- The `AudioModel` listens to particle locations and relative kinetic velocity to strike the 2D grid.
- Particles are mapped visually and aurally as they resonate against the void.
- Built with a `--headless` UI bypass to survive headless environments without X11 panics.
