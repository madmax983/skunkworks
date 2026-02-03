# Cargo Rocket 🚀

**Moonshot:** Homan Transfers + Resource Delivery Optimization

A gamified visualization of the Rust dependency graph where crates are planets in a solar system, and the compiler is a rocket ship collecting source code.

## 🔭 The Concept
We visualize the output of `cargo metadata` as an N-Body gravitational system.
- **The Sun**: The root package.
- **Planets**: Dependencies.
- **Mission**: Fly the ship to visit dependencies and "compile" them (collect them).
- **Physics**: Symplectic integration (Semi-Implicit Euler) for stable orbits and thrust mechanics.

## 🕹️ Controls
- **Left / Right**: Rotate Ship.
- **Space**: Toggle Thrust (Main Engine).
- **+/-**: Zoom In/Out.
- **Q**: Quit.

## 🛠️ Stack
- `ratatui`: Terminal rendering.
- `cargo_metadata`: Dependency graph extraction.
- `crossterm`: Input handling.
- `rand`: Procedural generation of planet properties.

## ⚠️ Notes
- The "Physics" uses a simplified N-Body simulation.
- Orbits are initialized to be roughly circular but will drift and perturb over time due to N-Body interactions (Chaos!).
- Fuel is limited! (Just kidding, it regenerates for now).
