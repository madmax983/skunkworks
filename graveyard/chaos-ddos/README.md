# Chaos DDoS 🌀🦗

**"A Swarm Seeking Chaos"**

A hybrid experiment crossing the macroquad swarm intelligence simulation of `locust-ddos` with the deterministic chaotic physics of `chaos-pendulum`.

## Concept

In a traditional DDoS simulation, botnets (Locusts) seek a static or slowly moving target server.
In `chaos-ddos`, the target Server is attached to the tip of a multi-jointed Chaotic Double Pendulum.

The swarm must constantly adapt to the wildly swinging, unpredictable server. This creates a mesmerizing, swirling hurricane of packets and pheromones dynamically tracing the strange attractor of the pendulum.

## Lineage

*   **From `locust-ddos`**: The 100,000 parallelized agents (Locusts) running via `rayon`, the pheromone grid (Congestion Memory), the firewall mechanics, and the pixel buffer rendering loop.
*   **From `chaos-pendulum`**: The physical node/link system (`physics.rs`) integrating gravity, friction, and Verlet constraints to create deterministic chaotic motion.
*   **Novel Trait**: Chaotic Swarm Mapping. The target is non-stationary and chaotic. The resulting visual phenotype is an organic cloud desperately trying to encapsulate a mathematically chaotic object.

## Controls

*   **Left Click:** Deploy a Firewall (Pesticide zone).
*   **C:** Clear all Firewalls.
