# Hyper-Neuron 🧠

**"The computer is thinking, and its thoughts are shaped by its burden."**

A hybrid experiment combining 4D visualization (`hyper-pollination`) with Spiking Neural Networks (`synaptic-physics`).

## 🧬 Lineage

*   **Parent A:** `experiments/hyper-pollination` (4D Rendering, System Distortion)
*   **Parent B:** `crates/synaptic-physics` (Izhikevich Neuron Model)
*   **Method:** Splicing the Izhikevich neuron model into the vertices of a 4D hypercube, where the edges (synapses) stretch and contract based on system load.

## 🔬 The Experiment

This simulation visualizes a Spiking Neural Network (SNN) inhabiting a 4-Dimensional Hypercube.

1.  **4D Brain:** Neurons are positioned in 4D space.
2.  **System Distortion:** Real-time system metrics (CPU, Memory, Swap, Load) distort the 4D geometry.
    *   **CPU:** Stretches the X-axis.
    *   **Memory:** Stretches the Y-axis.
    *   **Swap:** Stretches the Z-axis.
    *   **Load Avg:** Oscillates the W-axis.
3.  **Hyper-Neuroplasticity:**
    *   Signal propagation delay is determined by the *distorted distance* between neurons.
    *   When the system is under load (e.g., High CPU), the universe expands, and signals take longer to travel.
    *   This effectively changes the synchronization properties of the network based on the computer's physical effort.

## ⚗️ Emergent Behavior

*   **Stress Desynchronization:** Under heavy load, the increased delays may break the synchronous firing of neuron clusters.
*   **Metric-Driven Thought:** The firing patterns are a direct function of the system state. The computer's "mood" (load) alters its "mind" (network).

## 🎮 Controls

*   **Arrow Keys:** Rotate Camera (3D projection).
*   **W/S:** Zoom.

## 📦 Run

```bash
cargo run -p hyper-neuron
```
