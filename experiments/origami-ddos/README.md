# 🍄 origami-ddos

**Origami DDoS: Structural Cyberwarfare**

This experiment crosses the continuous soft-body procedural mesh generation of `origami` with the cyberwarfare swarm intelligence of `locust-ddos`.

## 🧬 Lineage

- **Parent A**: `crates/origami` (Procedural Miura-ori mesh and Position Based Dynamics simulation)
- **Parent B**: `experiments/locust-ddos` (Cyberwarfare DDoS packet swarm simulation)

## 🔬 Phenotype

Instead of simply charting network load, the server infrastructure is represented as a physical, deployable Miura-ori soft-body mesh.

Thousands of malicious DDoS packets (the swarm) navigate across the 2D plane mapped to the 3D surface of this mesh. As the packets swarm around the target and strike firewall bottlenecks, their immense localized density applies a physical contraction force on the structural constraints of the paper in that region.

**Emergent Behavior**: An organic, physical manifestation of a cyber attack. The paper mesh visually buckles, crumples, and collapses under the intense pressure of swarming malicious packets, illustrating network stress via 3D topographical failure.

## 🚀 Running

```bash
cargo run -p origami-ddos
```
