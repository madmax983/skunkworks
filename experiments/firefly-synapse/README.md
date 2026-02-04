# ⚛️ Firefly Synapse

> "The whole is greater than the sum of its parts." - Aristotle, but he was probably watching fireflies.

**Firefly Synapse** is a simulation of emergent synchronization in a swarm of 100,000 autonomous agents. It explores how local interactions (seeing a neighbor flash) can lead to global consensus (the entire swarm pulsing in unison), modeling phenomena from biological bioluminescence to distributed clock synchronization.

![Status](https://img.shields.io/badge/status-active-brightgreen)
![Agents](https://img.shields.io/badge/agents-100k-blue)
![Stack](https://img.shields.io/badge/stack-macroquad%20%2B%20rayon-orange)

## 🧪 The Science

The simulation implements the **Kuramoto Model** of coupled oscillators.

Each agent (firefly) has:
- An internal **phase** ($\theta$) that rotates at a natural frequency ($\omega$).
- A sensitivity to its neighbors.

When a neighbor flashes (phase peaks), it exerts a "pull" on the agent's phase:

$$ \frac{d\theta_i}{dt} = \omega_i + \frac{K}{N} \sum_{j=1}^{N} \sin(\theta_j - \theta_i) $$

Where $K$ is the coupling strength. Over time, despite random noise and different natural frequencies, the swarm phase-locks.

## 🏗️ The Engineering

To simulate **100,000 agents** at 60 FPS on the CPU, we employ:
- **Spatial Partitioning:** A **Linked Cell List** algorithm (O(N)) to query neighbors within a radius, avoiding the O(N²) all-to-all check.
- **Parallelism:** `rayon` spreads the update logic across all CPU cores.
- **Direct Pixel Access:** Instead of issuing 100,000 draw calls, we compute the pixel buffer directly in parallel and upload a single texture to the GPU.

## 🕹️ How to Run

```bash
cargo run --release -p firefly-synapse
```

*Note: Use `--release` for full performance. Debug mode may struggle with 100k agents.*

## 🔭 Observations

- **Chaos:** Initially, the screen is a static noise of random flashes.
- **Waves:** Small clusters form, creating traveling waves of light.
- **Pulse:** Eventually, the entire screen throbs in a unified heartbeat.
