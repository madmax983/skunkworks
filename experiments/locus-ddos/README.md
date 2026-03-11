# Locus DDoS 🧬

**Experiment Type:** Swarm Intelligence / Cyberwarfare Hybrid
**Status:** [FRESH HYBRID]
**Stack:** Rust, Macroquad, Rayon, Locus (Boids)

## 🔬 Lineage

A successful genetic recombination created by **The Splice Surgeon**.

*   **Parent A (Allele 1):** `experiments/locust-ddos`
    *   *Inherited Traits:* Network simulation topology (servers, firewalls, targeted packet routing), parallel agent updates (`rayon`), congestion pheromones.
*   **Parent B (Allele 2):** `crates/locus`
    *   *Inherited Traits:* Reynolds' Boid flocking algorithm (Separation, Alignment, Cohesion).

## 🧫 Phenotype Analysis

**Novel Trait:** Flocking Packets

By crossing the global target-seeking behavior of DDoS packets with the localized, peer-to-peer flocking mechanics of boids, we witness a fascinating emergent phenotype: **Flocking Cyberwarfare**.

Packets no longer travel as independent, stateless projectiles. Instead, they group together into massive schools, coordinating their movement and forming organic, cohesive strike forces that dynamically break apart and reform when colliding with chemical firewalls. This hybrid vigor creates a terrifyingly adaptive, organic visualization of a botnet swarm attack.

## Controls

- **Left Click:** Deploy a Firewall (Pesticide zone).
- **C:** Clear all Firewalls.

## Technical Details

- **100,000 Agents:** Simulated in parallel using `rayon`.
- **Render Buffer:** Direct pixel manipulation for high-performance visualization of the swarm.
- **Pheromone Grid:** 250x250 grid for environmental memory.
