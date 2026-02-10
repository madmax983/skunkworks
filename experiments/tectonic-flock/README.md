# 🦅 Tectonic Flock

**Parents**: `tectonic-git` × `luminous-flock`

A hybrid experiment where the geological strata of git history are traversed by a flock of "debugger" boids.

## Concept

- **Terrain**: The background is generated from `git log`. Each commit is a stratum. Stress (TODOs, FIXMEs) causes tectonic shifts and fissures.
- **Agents**: A flock of boids flies through this history.
- **Interaction**: The boids are attracted to the "Fissures" (bugs/debt) in the history. They glow red when investigating high-stress areas.

## Novel Trait: Swarm Debugging

Visualizing where the "bugs" are by where the flock gathers. The collective intelligence of the swarm highlights the most unstable parts of the codebase history.

## Controls

- `Space`: Pause/Resume
- `+ / -`: Increase/Decrease speed (commits per tick)
- `z / x`: Zoom In/Out
- `Arrows`: Manual Scroll (though it auto-scrolls to follow history)
- `q`: Quit

## Lineage

- **From `tectonic-git`**: Git scanning, Strata generation, Fissure logic.
- **From `luminous-flock`**: Boid physics, Flocking rules, Firefly synchronization.
- **Emergent**: Boids treating code debt as a physical attractor.
