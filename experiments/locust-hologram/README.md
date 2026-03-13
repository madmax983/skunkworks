# Holographic Cyberwarfare (locust-hologram)

A TUI simulation crossing the DDoS swarm logic of `locust-ddos` with the optical FFT interference rendering of `hologram-text`.

## Lineage

* **Parent A:** `experiments/locust-ddos` - Provides the underlying swarm logic: simulated packets navigating a grid, avoiding danger (pheromones), and attacking a central server.
* **Parent B:** `experiments/hologram-text` - Provides the FFT-based transformation that converts 2D spatial data into a holographic spectral projection.

## Novel Trait

**Spectral Attack Vectors.** Instead of viewing the physical position of the packets, we observe the cyber attack entirely through its frequency domain interference pattern. As the botnet searches, the spectral fingerprint is noisy. When the swarm locks onto the target or compresses violently against a firewall bottleneck, distinct high-frequency resonant modes emerge. We are visually observing the *acoustic resonance* of a cyber attack.

## Running

```bash
cargo run -p locust-hologram
```

* **Controls:**
  * `Space`: Spawn a firewall (blocks packets and forces bottlenecking).
  * `Arrows`: Shift the reconstruction angle.
  * `q`: Quit.
