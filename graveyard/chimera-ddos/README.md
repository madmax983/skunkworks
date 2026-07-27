# chimera-ddos

A hybrid genetic cyberwarfare visualization combining `locust-ddos` (macroquad swarm visualization) with `chimera-lang` (genetic VM).

## Lineage
- **Parent A (`locust-ddos`)**: Provided the macroquad visual scaffolding, firewall physics, and pathfinding goals.
- **Parent B (`chimera-lang`)**: Provided the `ChimeraVM` and `Dna` structures allowing agents to execute logical sequences.
- **Novel Trait (Genetic Attack Vectors)**: Rather than following a hardcoded Boid algorithm or simple pathfinding, each DDoS packet executes an evolving `ChimeraVM` genetic sequence to calculate its steering vectors. Packets that bypass firewalls successfully pass their DNA to the next generation.

## Run

```bash
cargo run -p chimera-ddos
```
