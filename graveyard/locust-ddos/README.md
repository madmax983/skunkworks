# Locust DDoS

**Experiment Type:** Swarm Intelligence / Cybersecurity Visualization
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon

## Concept

A visual simulation of a Distributed Denial of Service (DDoS) attack using swarm intelligence principles.
- **Locusts (Packets):** Individual agents seeking a target (Server).
- **Target (Server):** The victim system.
- **Firewalls (Pesticide):** Defensive perimeters that kill packets.
- **Emergence:** Dead packets leave "Congestion Pheromones" (Warning signals), causing other packets to divert and flow around defenses, visualizing the adaptive nature of botnets.

## Controls

- **Left Click:** Deploy a Firewall (Pesticide zone).
- **C:** Clear all Firewalls.

## Technical Details

- **100,000 Agents:** Simulated in parallel using `rayon`.
- **Render Buffer:** Direct pixel manipulation for high-performance visualization of the swarm.
- **Pheromone Grid:** 250x250 grid for environmental memory.
