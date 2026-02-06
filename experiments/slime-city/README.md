# Slime City 🍄🏙️

**"Slime mold pathfinding + Urban transit network design"**

> *In the concrete jungle, the fungus is the architect.*

## Concept
`slime-city` is a high-performance simulation of *Physarum polycephalum* (slime mold) acting as an emergent urban planner.
By treating cities as nutrient sources and the terrain as a chemo-attractant grid, we unleash **1,000,000 autonomous agents** to weave an organic transit network.

The agents follow simple rules:
1.  **Sense:** Sample trail strength and city gradients.
2.  **Turn:** Rotate towards the strongest signal.
3.  **Move:** Advance and deposit pheromone (trail).
4.  **Commute:** Oscillate between "Home" and "Work" cities.

The result is a mesmerizing visualization of emergent transport infrastructure that balances efficiency (Euclidean paths) with shared cost (bundled highways).

## Specs
- **Resolution:** 1920x1080
- **Agents:** 1,000,000 (Parallelized via `rayon`)
- **Output:** Frame sequences (PNG) for 60fps video generation.

## Usage

```bash
cargo run -p slime-city --release
```

Frames are output to `experiments/slime-city/output/`.

## The Code
- `src/simulation.rs`: The biological engine. Hand-tuned for data locality and parallel throughput.
- `src/main.rs`: The rendering pipeline. Maps chemical gradients to luminance.

## Future Mutations
- [ ] **Multi-species competition:** Red vs Blue slime fighting for rights of way.
- [ ] **Dynamic Cities:** Cities that grow based on agent arrival rates.
- [ ] **3D Terrain:** Agents climbing topographic maps.
