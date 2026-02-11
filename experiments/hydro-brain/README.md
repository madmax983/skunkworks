# Hydro-Brain 🧠💧

**Hybrid Experiment: `neuro-terminal` × `fluid-rain`**

A "Hydraulic Neural Network" where information is represented by fluid particles. Neurons are containers; synapses are pipes. Activation is driven by fluid accumulation and overflow/drainage logic.

## Concept

In a standard Spiking Neural Network (SNN), neurons fire discrete electrical spikes. In **Hydro-Brain**, these spikes are physically simulated clusters of fluid particles.

-   **Neurons** are physical containers (buckets).
-   **Synapses** are routes that "teleport" fluid from the bottom of one bucket (drain) to the top of another.
-   **Activation** occurs when fluid hits the drain. The amount of fluid represents the signal strength.
-   **Learning** (Static weights for now) determines the probability of a fluid particle traveling to a specific target neuron.

## Lineage

-   **Parent A**: `experiments/neuro-terminal`
    -   Provided the Neural Network topology (Layers, Neurons, Synapses).
    -   Provided the concept of visualizing a brain.
-   **Parent B**: `experiments/fluid-rain`
    -   Provided the Smoothed Particle Hydrodynamics (SPH) solver.
    -   Provided the TUI fluid rendering technique.

## Controls

-   `[1]`: Pour fluid into Input Neuron 1.
-   `[2]`: Pour fluid into Input Neuron 2.
-   `[3]`: Pour fluid into Input Neuron 3.
-   `[Q]`: Quit.

## Emergent Behavior

The "Spikes" travel visibly across the screen, creating a delay line memory effect. The fluid dynamics inside the buckets create a natural "leaky integrator" effect—if you pour slowly, the fluid might slosh around before draining, adding noise and temporal dynamics to the signal processing.
