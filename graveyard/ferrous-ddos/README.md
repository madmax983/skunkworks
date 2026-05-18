# 🧬 Ferrous DDoS

**Experiment Type:** Swarm Intelligence / Fluid Dynamics / Cybersecurity Visualization
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon

A visual simulation of a Distributed Denial of Service (DDoS) attack utilizing magnetic fluid dynamics.

## 🧬 Lineage

- **Parent A:** `experiments/locust-ddos` (Swarm intelligence seeking targets, firewalls)
- **Parent B:** `experiments/ferrous-fluid` (Magnetic fluid particles, physical particle repulsion)
- **Novel Trait:** Fluid-Dynamic Bottlenecking. Instead of simple swarm agents, the packets are magnetic fluid particles. When attacking a server, the packets compress into a high-density magnetic fluid. The server exerts an intense magnetic pull, and firewalls exert intense magnetic repulsion. The packets naturally push back against each other when they get too dense, visibly visualizing the bottlenecking and pressure wave of a DDoS attack pushing against defenses.

## Controls

- **Left Click:** Deploy a Magnetic Firewall (Repulsion zone).
- **C:** Clear all Firewalls.

## Technical Details

- Simulates fluid particles in parallel using `rayon`.
- Custom simplified O(N) particle collision/magnetic repulsion using stochastic neighbor sampling.
- Render buffer pixel manipulation for high-performance fluid rendering.
