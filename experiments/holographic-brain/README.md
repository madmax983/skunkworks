# Holographic Brain 🧠✨

A neural network where memory is stored as a holographic interference pattern in the frequency domain.

## Concept

Standard neural networks store information in the synaptic weights between neurons ($W_{ij}$).
In this experiment, the "weight matrix" is replaced by a **Holographic Memory** plate.

1.  **Neurons**: A 2D grid of Izhikevich spiking neurons.
2.  **Hologram**: A complex-valued grid in the frequency domain.
3.  **Interaction**:
    *   **Recording**: The spatial firing pattern of the neurons is treated as an "Object Wave". It is interfered with a "Reference Beam" (a plane wave at a specific angle) and the resulting interference pattern is accumulated in the Hologram via FFT.
    *   **Reconstruction**: The Hologram is illuminated by the Reference Beam (via IFFT) to produce a "Ghost Image" (Reconstruction) of the stored memories.
    *   **Feedback**: The Ghost Image injects current back into the neurons, closing the loop.

## Emergent Behavior

*   **Associative Memory**: The system should be able to complete partial patterns.
*   **Distributed Storage**: Memories are not local; they are distributed across the frequency domain.
*   **Dreaming**: Noise injection can cause the system to settle into stored attractors.

## Controls

*   `[L]`: Toggle **Learning** (Recording). When ON, current activity is added to the hologram.
*   `[N]`: Inject **Noise** into the neurons.
*   `[C]`: **Clear** neuron voltages (reset to resting).
*   `[R]`: **Reset** the Hologram (wipe memory).
*   `[Q]`: Quit.

## Lineage

*   **Parent A**: `experiments/hologram-text` (FFT/Holography logic)
*   **Parent B**: `experiments/lattice-brain` (Spiking Neuron grid)
*   **Novel Trait**: Holographic Associative Memory Loop.
