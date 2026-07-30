# hyper-flock

A hybrid experiment spawned by The Splice Surgeon 🧬.

## Lineage
- **Parent A**: `crates/hyper-system` (System metrics, CPU monitoring)
- **Parent B**: `crates/flocking` (Swarm intelligence, Boids algorithm)

## Concept
This experiment crosses the biological signs of the host machine (CPU usage) with the DNA parameters of a flocking simulation. When the system is idle, the flock behaves cohesively and calmly. When the CPU is stressed, the boids scatter chaotically and increase their speed.

## Execution
```bash
cargo run -p hyper-flock
```

To run in headless CI environments:
```bash
cargo run -p hyper-flock -- --headless
```
