# 🧬 Mnemonic Strings

**Parents:** `experiments/mnem-rot` × `experiments/ferrous-strings`

A hybrid experiment demonstrating "Entropy-Driven Acoustic Strings". The graph structure of a rotting codebase physically vibrates acoustic strings, translating digital decay into high-fidelity acoustic oscillations mapped onto a magnetic field.

## 🔬 Concept

In `mnem-rot`, a simulated codebase slowly decays over time. The "health" of each node visually represents its entropy.
In `ferrous-strings`, physical string simulations are coupled bidirectionally with a magnetic `Platter` (ferrous fluid grid).

In this hybrid, **Mnemonic Strings**, the two worlds collide. As the codebase nodes rot, they generate massive amounts of entropy which physically manifests as a repulsive magnetic force. This force physically pushes the nodes around and forcefully plucks the nearby acoustic strings. The strings in turn vibrate, playing audio and altering the ambient magnetic field.

## 🧬 Emergent Trait: Acoustic Code Rot

The soundscape and magnetic fluid patterns are driven entirely by the health of the underlying repository graph.
- **Healthy code** maintains stable structural graphs and gently reinforces positive magnetic flow.
- **Rotting code** violently strikes the strings and injects negative (repulsive) magnetic fields into the environment.

This creates a high-fidelity acoustic fingerprint of the codebase's real-time architectural health.

## 🕹️ Controls

-   **Right Click + Drag**: Pan camera.
-   **Mouse Scroll**: Zoom camera.
-   **Hover Node**: Heal the node (reduces its entropy, silencing its plucks).

## 📦 Lineage

-   **From `mnem-rot`**: Codebase graph mapping, decay over time, nodes with entropy (`Graph`, `glitch`).
-   **From `ferrous-strings`**: The magnetic string simulation, TUI rendering of strings, magnetic platter interaction, and audio synthesis (`FerrousString`, `Particle`, `AudioCommand`).
