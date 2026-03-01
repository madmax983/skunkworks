# gray-flock 🐦‍🔥

**Lineage**: `gray-scott` × `luminous-flock`

## Concept
Reaction-Diffusion Swarming. A TUI visualization where boids flock inside a complex, dynamic chemical environment. The boids are attracted to the V chemical gradient (chemotaxis) and, in turn, deposit the V chemical into the grid.

## Emergent Traits
This creates a macroscopic-microscopic feedback loop. The flock shapes the environment that guides it, turning the chemical gradients into structural highways for the swarm.

## Usage
Run with:
```bash
cargo run -p gray-flock
```
Controls:
- `q`: Quit
- `r`: Reset simulation
