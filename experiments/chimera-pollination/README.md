# Chimera Pollination 🌸🧬

A hybrid experiment combining `hyper-pollination` (4D ecosystem driven by system metrics) and `chimera-lang` (genetic programming).

## Concept

In this simulation, **Plants** are generated from Git History using L-Systems, existing in a 4D Hypercube that is distorted by real-time system metrics (CPU, RAM, Swap).

**Agents** are independent `ChimeraVM` instances. They act as "Pollinators".
- **Brain:** A genetic program (DNA) that processes sensory inputs.
- **Body:** A physics-based entity subject to 4D flocking rules and system turbulence.
- **Sensory Inputs:**
    - Flocking vectors (Alignment, Cohesion, Separation)
    - Nearest Plant direction
    - System Load (Wind)
- **Motor Outputs:**
    - Volition vector (modifies trajectory)
    - Pollination signal

## Lineage

- **Parent A:** `experiments/hyper-pollination`
    - Contributed: 4D Plant generation (L-Systems), System Monitor integration, 4D-to-3D projection logic.
- **Parent B:** `experiments/chimera-lang`
    - Contributed: `ChimeraVM` agents, Genetic Code execution, Ether channels for I/O.
- **Novel Trait:** **Evolving Social Behavior**. The agents' movement is a hybrid of traditional flocking algorithms and their own evolved "Volition". They can choose to follow the flock, seek plants, or fight the wind based on their DNA.

## Usage

```bash
cargo run -p chimera-pollination
```

## Controls

- **Arrow Keys / WASD:** Rotate Camera
- **Simulation:** Driven by your computer's metabolic state (CPU Load).

## Implementation Details

The `Agent` struct wraps a `ChimeraVM`. Every tick, the agent feeds physical forces into the VM's input channels (The Ether). The VM executes a few steps of its genetic code and outputs a "Volition" vector, which is added to the physical acceleration. This creates a feedback loop between the rigid physics engine and the chaotic biological brain.
