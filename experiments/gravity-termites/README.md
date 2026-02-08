# Gravity Termites 🐜🌍

A hybrid experiment combining the agent-based building of `thermo-termites` with the gravitational physics of `celestial-cantata`.

## Concept

Termites (Agents) live in a 2D space. They can pick up `Dust` and deposit it as `Rock`.
Crucially, every piece of `Rock` and `Dust` contributes to a **Global Gravitational Field**.

The field is solved in real-time using a Poisson Solver (Jacobi Iteration) on the grid:
`∇²Φ = 4πGρ`

Agents are attracted to high-potential areas (Mass). They walk on the surface of the structures they build, and fly freely in the vacuum.

## Emergent Behavior

- **Accretion**: Agents naturally gather dust into clumps due to gravity.
- **Planet Formation**: Clumps merge into a central planetoid.
- **Atmosphere**: Agents swarm around the planetoid, bound by its gravity.

## Controls

- **Arrow Keys**: Pan the camera.
- **+/-**: Zoom in/out.

## Lineage

- **Parent A**: `experiments/thermo-termites` (Grid-based agents, picking/dropping logic).
- **Parent B**: `experiments/celestial-cantata` (Orbital mechanics, Gravity).
- **Novel Trait**: **Self-Gravitating Architecture**. The environment is shaped by the agents, and the agents are shaped by the environment's gravity.
